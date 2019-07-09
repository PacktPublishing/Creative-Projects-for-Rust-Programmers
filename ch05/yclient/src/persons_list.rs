use failure::Error;
use yew::format::{Json, Nothing};
use yew::services::{DialogService, ConsoleService};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

use crate::db_access::{DbConnection, Person};

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
    db_connection: std::rc::Rc<std::cell::RefCell<DbConnection>>,
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
    pub db_connection: Option<std::rc::Rc<std::cell::RefCell<DbConnection>>>,
    pub username: String,
    pub password: String,
}

impl Default for PersonsListProps {
    fn default() -> Self {
        PersonsListProps {
            can_write: false,
            go_to_one_person_page: None,
            db_connection: None,
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
            db_connection: props.db_connection.unwrap(),
            console: ConsoleService::new(),
            username: "".to_string(),
            password: "".to_string(),
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
            }
            PersonsListMsg::PartialNameChanged(s) => self.name_portion = s,
            PersonsListMsg::DeletePressed => {
                if self
                    .dialog
                    .confirm("Do you confirm to delete the selected persons?")
                {
                    //http://localhost:8080/one_person?id_list=1,3
                    self.fetching = true;
                    self.ft = Some({
                        let callback = self.link.send_back(
                            move |response: Response<Json<Result<u32, Error>>>| {
                                let (meta, Json(data)) = response.into_parts();
                                if meta.status.is_success() {
                                    PersonsListMsg::ReadyDeletedPersons(data)
                                } else {
                                    PersonsListMsg::Failure(
                                        format!("No persons deleted."))
                                }
                            },
                        );

                        let mut request = Request::delete(
                            &format!("http://localhost:8080/persons?id_list={}",
                                self.selected_ids.iter()
                                .map(|id| id.to_string())
                                .collect::<Vec<_>>()
                                .join(","))
                            ).body(Nothing).unwrap();
                        
                        let mut auth_string = "Basic ".to_string();
                        base64::encode_config_buf(
                            format!("{}:{}",
                                self.username, self.password).as_bytes(),
                            base64::STANDARD,
                            &mut auth_string);
                        request.headers_mut().append(
                            "authorization",
                            auth_string.parse().unwrap()
                            );
                        self.console.log(&format!("request.headers: {:?}.", request.headers()));

                        self.fetch_service.fetch(request, callback)
                    });
                }
            }
            PersonsListMsg::ReadyDeletedPersons(response) => {
                self.fetching = false;
                let num_deleted = response.unwrap_or(0);
                self.console.log(&format!("ReadyDeletedPersons: {}.",
                    num_deleted));

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
                self.ft = Some({
                    let callback = self.link.send_back(
                        move |response: Response<Json<Result<Person, Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            if meta.status.is_success() {
                                PersonsListMsg::ReadyPersonToEdit(data)
                            } else {
                                PersonsListMsg::Failure(
                                    format!("No person found with the indicated id"))
                            }
                        },
                    );

                    let mut request = Request::get(
                        format!("http://localhost:8080/person/id/{}", id))
                        .body(Nothing).unwrap();
                    
                    let mut auth_string = "Basic ".to_string();
                    base64::encode_config_buf(
                        &base64::encode(format!("{}:{}",
                            self.username, self.password).as_bytes()),
                        base64::STANDARD,
                        &mut auth_string);
                    request.headers_mut().append(
                        "authorization",
                        auth_string.parse().unwrap()
                        );
                    self.console.log(&format!("request.headers: {:?}.", request.headers()));

                    self.fetch_service.fetch(request, callback)
                });
            }
            PersonsListMsg::ReadyPersonToEdit(person) => {
                self.fetching = false;
                let person = person.unwrap_or(Person { id: 0, name: "".to_string() });
                if let Some(ref go_to_page) = self.go_to_one_person_page {
                    self.console.log(&format!("ReadyPersonToEdit: {:?}.",
                        person));
                    go_to_page.emit(Some(person.clone()));
                }
            }
            PersonsListMsg::FilterPressed => {
                self.fetching = true;
                self.ft = Some({
                    let callback = self.link.send_back(
                        move |response: Response<Json<Result<Vec<Person>, Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            if meta.status.is_success() {
                                PersonsListMsg::ReadyFilteredPersons(data)
                            } else {
                                PersonsListMsg::Failure(
                                    format!("No persons found."))
                            }
                        },
                    );

                    let mut request = Request::get(
                        "http://localhost:8080/persons?partial_name=".to_string() +
                        &url::form_urlencoded::byte_serialize(
                            self.name_portion.as_bytes()).collect::<String>()
                        ).body(Nothing).unwrap();
                    
                    let mut auth_string = "Basic ".to_string();
                    base64::encode_config_buf(
                        &base64::encode(("username".to_string() + ":" + "password").as_bytes()),
                        base64::STANDARD,
                        &mut auth_string);
                    request.headers_mut().append(
                        "authorization",
                        auth_string.parse().unwrap()
                        );
                    self.console.log(&format!("request.headers: {:?}.", request.headers()));

                    self.fetch_service.fetch(request, callback)
                });
            }
            PersonsListMsg::ReadyFilteredPersons(response) => {
                self.fetching = false;
                self.filtered_persons = response.unwrap_or(vec![]);
                self.console.log(&format!("ReadyFilteredPersons: {:?}.",
                    self.filtered_persons));
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
        self.db_connection = props.db_connection.unwrap();

        //self.filtered_persons = self.db_connection.borrow().get_persons_by_partial_name("");
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
