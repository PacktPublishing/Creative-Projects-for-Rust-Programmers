use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew::events::InputData;

use crate::db_access::{DbConnection, Person};

pub struct OnePersonModel {
    id: Option<u32>,
    name: String,
    can_write: bool,
    is_inserting: bool,
    go_to_persons_list_page: Option<Callback<()>>,
    db_connection: std::rc::Rc<std::cell::RefCell<DbConnection>>,
    link: ComponentLink<Self>,
}

#[derive(Debug)]
pub enum OnePersonMsg {
    NameChanged(String),
    SavePressed,
    CancelPressed,
}

#[derive(PartialEq, Clone, Properties)]
pub struct OnePersonProps {
    pub id: Option<u32>,
    pub name: String,
    pub can_write: bool,
    pub go_to_persons_list_page: Option<Callback<()>>,
    pub db_connection: Option<std::rc::Rc<std::cell::RefCell<DbConnection>>>,
}

impl Default for OnePersonProps {
    fn default() -> Self {
        OnePersonProps {
            id: None,
            name: "".to_string(),
            can_write: false,
            go_to_persons_list_page: None,
            db_connection: None,
        }
    }
}

impl Component for OnePersonModel {
    type Message = OnePersonMsg;
    type Properties = OnePersonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        OnePersonModel {
            id: props.id,
            name: props.name,
            can_write: props.can_write,
            is_inserting: props.id.is_none(),
            go_to_persons_list_page: props.go_to_persons_list_page,
            db_connection: props.db_connection.unwrap(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            OnePersonMsg::NameChanged(name) => self.name = name,
            OnePersonMsg::SavePressed => {
                if self.is_inserting {
                    self.db_connection.borrow_mut().insert_person(Person {
                        id: 0,
                        name: self.name.clone(),
                    });
                } else {
                    self.db_connection.borrow_mut().update_person(Person {
                        id: self.id.unwrap(),
                        name: self.name.clone(),
                    });
                }
                if let Some(ref go_to_page) = self.go_to_persons_list_page {
                    go_to_page.emit(());
                }
            }
            OnePersonMsg::CancelPressed => {
                if let Some(ref go_to_page) = self.go_to_persons_list_page {
                    go_to_page.emit(());
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.id = props.id;
        self.name = props.name;
        self.can_write = props.can_write;
        self.is_inserting = props.id.is_none();
        self.go_to_persons_list_page = props.go_to_persons_list_page;
        self.db_connection = props.db_connection.unwrap();
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <label>{ "Id: " }</label>
                    <input
                        type="number",
                        value=match self.id { Some(id) => format!("{}", id), _ => "".to_string() },
                        disabled=true,
                    />
                </div>
                <div>
                    <label>{ "Name: " }</label>
                    <input
                        type="text",
                        value=&self.name,
                        disabled=!self.can_write,
                        oninput=self.link.callback(|e: InputData| OnePersonMsg::NameChanged(e.value)),
                    />
                </div>
                <div>
                    <button
                        onclick=self.link.callback(|_| OnePersonMsg::SavePressed),
                        disabled=!self.can_write,
                    >
                        { if self.is_inserting { "Insert" } else { "Update" } }
                    </button>
                    { " " }
                    <button
                        onclick=self.link.callback(|_| OnePersonMsg::CancelPressed),
                        disabled=!self.can_write,
                    >
                        { "Cancel" }
                    </button>
                </div>
            </div>
        }
    }
}
