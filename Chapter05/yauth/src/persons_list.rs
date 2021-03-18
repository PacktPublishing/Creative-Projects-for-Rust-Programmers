use yew::services::DialogService;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew::events::InputData;

use crate::db_access::{DbConnection, Person};

pub struct PersonsListModel {
    id_to_find: Option<u32>,
    name_portion: String,
    filtered_persons: Vec<Person>,
    selected_ids: std::collections::HashSet<u32>,
    can_write: bool,
    go_to_one_person_page: Option<Callback<Option<Person>>>,
    db_connection: std::rc::Rc<std::cell::RefCell<DbConnection>>,
    link: ComponentLink<Self>,
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
}

#[derive(PartialEq, Clone, Properties)]
pub struct PersonsListProps {
    pub can_write: bool,
    pub go_to_one_person_page: Option<Callback<Option<Person>>>,
    pub db_connection: Option<std::rc::Rc<std::cell::RefCell<DbConnection>>>,
}

impl Default for PersonsListProps {
    fn default() -> Self {
        PersonsListProps {
            can_write: false,
            go_to_one_person_page: None,
            db_connection: None,
        }
    }
}

impl Component for PersonsListModel {
    type Message = PersonsListMsg;
    type Properties = PersonsListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = PersonsListModel {
            id_to_find: None,
            name_portion: "".to_string(),
            filtered_persons: Vec::<Person>::new(),
            selected_ids: std::collections::HashSet::<u32>::new(),
            can_write: props.can_write,
            go_to_one_person_page: props.go_to_one_person_page,
            db_connection: props.db_connection.unwrap(),
            link,
        };
        model.filtered_persons = model.db_connection.borrow().get_persons_by_partial_name("");
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
                    DialogService::alert("No id specified.");
                }
            },
            PersonsListMsg::PartialNameChanged(s) => self.name_portion = s,
            PersonsListMsg::FilterPressed => {
                self.filtered_persons = self
                    .db_connection
                    .borrow()
                    .get_persons_by_partial_name(&self.name_portion);
            }
            PersonsListMsg::DeletePressed => {
                if DialogService::confirm("Do you confirm to delete the selected persons?")
                {
                    {
                        let mut db = self.db_connection.borrow_mut();
                        for id in &self.selected_ids {
                            db.delete_by_id(*id);
                        }
                    }
                    self.update(PersonsListMsg::FilterPressed);
                    DialogService::alert("Deleted.");
                }
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
                match self.db_connection.borrow().get_person_by_id(id) {
                    Some(person) => {
                        if let Some(ref go_to_page) = self.go_to_one_person_page {
                            go_to_page.emit(Some(person.clone()));
                        }
                    }
                    None => DialogService::alert("No person found with the indicated id."),
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.can_write = props.can_write;
        self.go_to_one_person_page = props.go_to_one_person_page;
        self.db_connection = props.db_connection.unwrap();
        self.filtered_persons = self.db_connection.borrow().get_persons_by_partial_name("");
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <label>{ "Id: " }</label>
                    <input
                        type="number",
                        oninput=self.link.callback(|e: InputData| PersonsListMsg::IdChanged(e.value)),/>
                    { " " }
                    <button
                        onclick=self.link.callback(|_| PersonsListMsg::FindPressed),>
                        { "Find" }
                    </button>
                </div>
                <div>
                    <label>{ "Name portion: " }</label>
                    <input
                        type="text",
                        oninput=self.link.callback(|e: InputData| PersonsListMsg::PartialNameChanged(e.value)),
                    />
                    { " " }
                    <button
                        onclick=self.link.callback(|_| PersonsListMsg::FilterPressed),
                    >
                        { "Filter" }
                    </button>
                </div>
                <button
                    onclick=self.link.callback(|_| PersonsListMsg::DeletePressed),
                    disabled=!self.can_write,
                >
                    { "Delete Selected Persons" }
                    </button>
                { " " }
                <button
                    onclick=self.link.callback(|_| PersonsListMsg::AddPressed),
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
                                            let id = p.id.clone();
                                            let name = p.name.clone();
                                            html! {
                                                <tr>
                                                    <td><input
                                                        type="checkbox",
                                                        oninput=self.link.callback(move |_| PersonsListMsg::SelectionToggled(id)),
                                                        checked=self.selected_ids.contains(&id),
                                                        /></td>
                                                    <td><button
                                                        onclick=self.link.callback(move |_| PersonsListMsg::EditPressed(id)),>{ "Edit" }</button></td>
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
