use yew::services::DialogService;
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender, Properties};
use yew::events::InputData;

use crate::db_access::{DbConnection, User};

pub struct LoginModel {
    username: String,
    password: String,
    when_logged_in: Option<Callback<User>>,
    db_connection: std::rc::Rc<std::cell::RefCell<DbConnection>>,
    link: ComponentLink<Self>,
}

#[derive(Debug)]
pub enum LoginMsg {
    UsernameChanged(String),
    PasswordChanged(String),
    LoginPressed,
}

#[derive(PartialEq, Clone, Properties)]
pub struct LoginProps {
    pub current_username: Option<String>,
    pub when_logged_in: Option<Callback<User>>,
    pub db_connection: Option<std::rc::Rc<std::cell::RefCell<DbConnection>>>,
}

impl Default for LoginProps {
    fn default() -> Self {
        LoginProps {
            current_username: None,
            when_logged_in: None,
            db_connection: None,
        }
    }
}

impl Component for LoginModel {
    type Message = LoginMsg;
    type Properties = LoginProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        LoginModel {
            username: props.current_username.unwrap_or_default(),
            password: String::new(),
            when_logged_in: props.when_logged_in,
            db_connection: props.db_connection.unwrap(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LoginMsg::UsernameChanged(username) => self.username = username,
            LoginMsg::PasswordChanged(password) => self.password = password,
            LoginMsg::LoginPressed => {
                if let Some(user) = self
                    .db_connection
                    .borrow()
                    .get_user_by_username(&self.username)
                {
                    if user.password == self.password {
                        if let Some(ref go_to_page) = self.when_logged_in {
                            go_to_page.emit(user.clone());
                        }
                    } else {
                        DialogService::alert("Invalid password for the specified user.");
                    }
                } else {
                    DialogService::alert("User not found.");
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.username = props.current_username.unwrap_or_default();
        self.when_logged_in = props.when_logged_in;
        self.db_connection = props.db_connection.unwrap();
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <label>{ "User name: " }</label>
                    <input
                        type="text",
                        value=&self.username,
                        oninput=self.link.callback(|e: InputData| LoginMsg::UsernameChanged(e.value)),
                    />
                </div>
                <div>
                    <label>{ "Password: " }</label>
                    <input
                        type="password",
                        oninput=self.link.callback(|e: InputData| LoginMsg::PasswordChanged(e.value)),
                    />
                </div>
                <button
                    onclick=self.link.callback(|_| LoginMsg::LoginPressed),>
                    { "Log in" }
                </button>
            </div>
        }
    }
}
