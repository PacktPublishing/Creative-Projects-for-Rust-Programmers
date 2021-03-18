#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod db_access;
use crate::login::LoginModel;
use db_access::{DbConnection, DbPrivilege, User};

mod login;

enum Page {
    Login,
    PersonsList,
}

struct MainModel {
    page: Page,
    current_user: Option<String>,
    can_write: bool,
    db_connection: std::rc::Rc<std::cell::RefCell<DbConnection>>,
    link: ComponentLink<Self>,
}

enum MainMsg {
    LoggedIn(User),
    ChangeUserPressed,
}

impl Component for MainModel {
    type Message = MainMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MainModel {
            page: Page::Login,
            current_user: None,
            can_write: false,
            db_connection: std::rc::Rc::new(std::cell::RefCell::new(DbConnection::new())),
            link,
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
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
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
                                            onclick=self.link.callback(|_| MainMsg::ChangeUserPressed),>
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
                                when_logged_in=Some(self.link.callback(|user: User| MainMsg::LoggedIn(user))),
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

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<MainModel>::new().mount_to_body();
}
