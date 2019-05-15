use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod db_access;
use db_access::{DbConnection, User, DbPrivilege};
use crate::login::{LoginModel};

mod login;

enum Page {
    Login,
    PersonsList,
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
            },
            MainMsg::ChangeUserPressed =>
                self.page = Page::Login,
            MainMsg::GoToPersonsListPage =>
                self.page = Page::PersonsList,            
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
                                when_logged_in=|u| MainMsg::LoggedIn(u),
                                db_connection=Some(self.db_connection.clone()),
                            />
                        },
                        Page::PersonsList => html! {
                            <h2>{ "Page to be implemented" }</h2>
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
