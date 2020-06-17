use failure::Error;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::{ConsoleService, DialogService};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

use crate::common::{add_auth, Person, BACKEND_SITE};

pub struct PersonsListModel {
    fetching: bool,
    fetch_service: FetchService,
    ft: Option<FetchTask>,
    link: ComponentLink<PersonsListModel>,
    dialog: DialogService,
    id_to_find: Option<u32>,
    name_portion: String,
    filtered_persons: Vec<Person>,
    selected_ids: std::collections::HashSet<u32>,
    can_write: bool,
    go_to_one_person_page: Option<Callback<Option<Person>>>,
    console: ConsoleService,
    username: String,
    password: String,
}

#[derive(Debug)]
pub enum PersonsListMsg {
    IdChanged(String),
    FindPressed,
    PartialNameChanged(String),
    FilterPressed,
    DeletePressed,
    AddPressed,
    SelectionToggled(u32),
    EditPressed(u32),
    ReadyFilteredPersons(Result<Vec<Person>, Error>),
    ReadyDeletedPersons(Result<u32, Error>),
    ReadyPersonToEdit(Result<Person, Error>),
    Failure(String),
}

#[derive(PartialEq, Clone)]
pub struct PersonsListProps {
    pub can_write: bool,
    pub go_to_one_person_page: Option<Callback<Option<Person>>>,
    pub username: String,
    pub password: String,
}

impl Default for PersonsListProps {
    fn default() -> Self {
        PersonsListProps {
            can_write: false,
            go_to_one_person_page: None,
            username: "".to_string(),
            password: "".to_string(),
        }
    }
}

impl Component for PersonsListModel {
    type Message = PersonsListMsg;
    type Properties = PersonsListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = PersonsListModel {
            fetching: false,
            fetch_service: FetchService::new(),
            ft: None,
            link,
            dialog: DialogService::new(),
            id_to_find: None,
            name_portion: "".to_string(),
            filtered_persons: Vec::<Person>::new(),
            selected_ids: std::collections::HashSet::<u32>::new(),
            can_write: props.can_write,
            go_to_one_person_page: props.go_to_one_person_page,
            console: ConsoleService::new(),
            username: props.username,
            password: props.password,
        };
        model.update(PersonsListMsg::FilterPressed);
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PersonsListMsg::IdChanged(id_str) => self.id_to_find = id_str.parse::<u32>().ok(),
            PersonsListMsg::FindPressed => match self.id_to_find {
                Some(id) => {
                    self.update(PersonsListMsg::EditPressed(id));
                }
                None => {
                    self.dialog.alert("No id specified.");
                }
            },
            PersonsListMsg::PartialNameChanged(s) => self.name_portion = s,
            PersonsListMsg::DeletePressed => {
                if self
                    .dialog
                    .confirm("Do you confirm to delete the selected persons?")
                {
                    self.fetching = true;
                    let callback =
                        self.link
                            .send_back(move |response: Response<Json<Result<u32, Error>>>| {
                                let (meta, Json(data)) = response.into_parts();
                                if meta.status.is_success() {
                                    PersonsListMsg::ReadyDeletedPersons(data)
                                } else {
                                    PersonsListMsg::Failure("No persons deleted.".to_string())
                                }
                            });

                    let mut request = Request::delete(&format!(
                        "{}persons?id_list={}",
                        BACKEND_SITE,
                        self.selected_ids
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    ))
                    .body(Nothing)
                    .unwrap();

                    add_auth(&self.username, &self.password, &mut request);
                    self.ft = Some(self.fetch_service.fetch(request, callback));
                }
            }
            PersonsListMsg::ReadyDeletedPersons(response) => {
                self.fetching = false;
                let num_deleted = response.unwrap_or(0);
                self.console
                    .log(&format!("ReadyDeletedPersons: {}.", num_deleted));

                self.update(PersonsListMsg::FilterPressed);
                self.dialog.alert("Deleted.");
            }
            PersonsListMsg::AddPressed => {
                if let Some(ref go_to_page) = self.go_to_one_person_page {
                    go_to_page.emit(None);
                }
            }
            PersonsListMsg::SelectionToggled(id) => {
                if self.selected_ids.contains(&id) {
                    self.selected_ids.remove(&id);
                } else {
                    self.selected_ids.insert(id);
                }
            }
            PersonsListMsg::EditPressed(id) => {
                self.fetching = true;
                self.console.log(&format!("EditPressed: {:?}.", id));
                let callback =
                    self.link
                        .send_back(move |response: Response<Json<Result<Person, Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            if meta.status.is_success() {
                                PersonsListMsg::ReadyPersonToEdit(data)
                            } else {
                                PersonsListMsg::Failure(
                                    "No person found with the indicated id".to_string(),
                                )
                            }
                        });

                let mut request = Request::get(format!("{}person/id/{}", BACKEND_SITE, id))
                    .body(Nothing)
                    .unwrap();

                add_auth(&self.username, &self.password, &mut request);
                self.ft = Some(self.fetch_service.fetch(request, callback));
            }
            PersonsListMsg::ReadyPersonToEdit(person) => {
                self.fetching = false;
                let person = person.unwrap_or(Person {
                    id: 0,
                    name: "".to_string(),
                });
                if let Some(ref go_to_page) = self.go_to_one_person_page {
                    self.console
                        .log(&format!("ReadyPersonToEdit: {:?}.", person));
                    go_to_page.emit(Some(person.clone()));
                }
            }
            PersonsListMsg::FilterPressed => {
                self.fetching = true;
                let callback = self.link.send_back(
                    move |response: Response<Json<Result<Vec<Person>, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        if meta.status.is_success() {
                            PersonsListMsg::ReadyFilteredPersons(data)
                        } else {
                            PersonsListMsg::Failure("No persons found.".to_string())
                        }
                    },
                );

                let mut request = Request::get(format!(
                    "{}persons?partial_name={}",
                    BACKEND_SITE,
                    url::form_urlencoded::byte_serialize(self.name_portion.as_bytes())
                        .collect::<String>()
                ))
                .body(Nothing)
                .unwrap();

                add_auth(&self.username, &self.password, &mut request);
                self.ft = Some(self.fetch_service.fetch(request, callback));
            }
            PersonsListMsg::ReadyFilteredPersons(response) => {
                self.fetching = false;
                self.filtered_persons = response.unwrap_or_else(|_| vec![]);
                self.console.log(&format!(
                    "ReadyFilteredPersons: {:?}.",
                    self.filtered_persons
                ));
            }
            PersonsListMsg::Failure(msg) => {
                self.fetching = false;
                self.console.log(&format!("Failure: {:?}.", msg));
                self.dialog.alert(&msg);
                return false;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.can_write = props.can_write;
        self.go_to_one_person_page = props.go_to_one_person_page;
        self.username = props.username;
        self.password = props.password;
        self.update(PersonsListMsg::FilterPressed);
        true
    }
}

impl Renderable<PersonsListModel> for PersonsListModel {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <label>{ "Id: " }</label>
                    <input
                        type="number",
                        oninput=|e| PersonsListMsg::IdChanged(e.value),/>
                    { " " }
                    <button
                        onclick=|_| PersonsListMsg::FindPressed,>
                        { "Find" }
                    </button>
                </div>
                <div>
                    <label>{ "Name portion: " }</label>
                    <input
                        type="text",
                        oninput=|e| PersonsListMsg::PartialNameChanged(e.value),
                    />
                    { " " }
                    <button
                        onclick=|_| PersonsListMsg::FilterPressed,
                    >
                        { "Filter" }
                    </button>
                </div>
                <button
                    onclick=|_| PersonsListMsg::DeletePressed,
                    disabled=!self.can_write,
                >
                    { "Delete Selected Persons" }
                    </button>
                { " " }
                <button
                    onclick=|_| PersonsListMsg::AddPressed,
                    disabled=!self.can_write,
                >
                    { "Add New Person" }
                    </button>

                {
                    if !self.filtered_persons.is_empty() {
                        html! {
                            <table>
                                <thead>
                                    <th></th>
                                    <th></th>
                                    <th>{ "Id" }</th>
                                    <th>{ "Name" }</th>
                                </thead>
                                <tbody>
                                    {
                                        for self.filtered_persons.iter().map(|p| {
                                            let id = p.id;
                                            let name = p.name.clone();
                                            html! {
                                                <tr>
                                                    <td><input
                                                        type="checkbox",
                                                        oninput=|_| PersonsListMsg::SelectionToggled(id),
                                                        checked=self.selected_ids.contains(&id),
                                                        /></td>
                                                    <td><button
                                                        onclick=|_| PersonsListMsg::EditPressed(id),>{ "Edit" }</button></td>
                                                    <td>{ id }</td>
                                                    <td>{ name }</td>
                                                </tr>
                                            }
                                        })
                                    }
                                </tbody>
                            </table>
                        }
                    }
                    else {
                        html! {
                            <p>{ "No persons." }</p>
                        }
                    }
                }
            </div>
        }
    }
}
