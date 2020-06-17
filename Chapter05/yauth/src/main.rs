#![recursion_limit = "128"]
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod db_access;
use crate::login::LoginModel;
use crate::one_person::OnePersonModel;
use crate::persons_list::PersonsListModel;
use db_access::{DbConnection, DbPrivilege, Person, User};

mod login;
mod one_person;
mod persons_list;

enum Page {
    Login,
    PersonsList,
    OnePerson(Option<Person>),
}

pub struct MainModel {
    page: Page,
    current_user: Option<String>,
    can_write: bool,
    db_connection: std::rc::Rc<std::cell::RefCell<DbConnection>>,
}

#[derive(Debug)]
pub enum MainMsg {
    LoggedIn(User),
    ChangeUserPressed,
    GoToOnePersonPage(Option<Person>),
    GoToPersonsListPage,
}

impl Component for MainModel {
    type Message = MainMsg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        MainModel {
            page: Page::Login,
            current_user: None,
            can_write: false,
            db_connection: std::rc::Rc::new(std::cell::RefCell::new(DbConnection::new())),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MainMsg::LoggedIn(user) => {
                self.page = Page::PersonsList;
                self.current_user = Some(user.username);
                self.can_write = user.privileges.contains(&DbPrivilege::CanWrite);
            }
            MainMsg::ChangeUserPressed => self.page = Page::Login,
            MainMsg::GoToOnePersonPage(person) => self.page = Page::OnePerson(person),
            MainMsg::GoToPersonsListPage => self.page = Page::PersonsList,
        }
        true
    }
}

impl Renderable<MainModel> for MainModel {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <style>
                { "
                    .current-user { color: #0000C0}
                " }
                </style>
                <header>
                    <h2>{ "Persons management" }</h2>
                    <p>
                        { "Current user: " }
                        <span class="current-user", >
                        {
                            if let Some(user) = &self.current_user {
                                user
                            }
                            else {
                                "---"
                            }
                        }
                        </span>
                        {
                            match self.page {
                                Page::Login => html! { <div/> },
                                _ => html! {
                                    <span>
                                        { " " }
                                        <button
                                            onclick=|_| MainMsg::ChangeUserPressed,>
                                            { "Change User" }
                                        </button>
                                    </span>
                                },
                            }
                        }
                    </p>
                    <hr/>
                </header>
                {
                    match &self.page {
                        Page::Login => html! {
                            <LoginModel:
                                current_username=&self.current_user,
                                when_logged_in=MainMsg::LoggedIn,
                                db_connection=Some(self.db_connection.clone()),
                            />
                        },
                        Page::PersonsList => html! {
                            <PersonsListModel:
                                can_write=self.can_write,
                                go_to_one_person_page=MainMsg::GoToOnePersonPage,
                                db_connection=Some(self.db_connection.clone()),
                            />
                        },
                        Page::OnePerson(person) => html! {
                            <OnePersonModel:
                                id=match person { Some(p) => Some(p.id), None => None },
                                name=match person { Some(p) => p.name.clone(), None => "".to_string() },
                                can_write=self.can_write,
                                go_to_persons_list_page=|_| MainMsg::GoToPersonsListPage,
                                db_connection=Some(self.db_connection.clone()),
                            />
                        },
                    }
                }
                <footer>
                    <hr/>
                    { "\u{A9} Carlo Milanesi - Developed using Yew" }
                </footer>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<MainModel>();
}
