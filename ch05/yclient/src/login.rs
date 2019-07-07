use yew::services::DialogService;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

use crate::db_access::{DbConnection, User};

pub struct LoginModel {
    dialog: DialogService,
    username: String,
    password: String,
    when_logged_in: Option<Callback<User>>,
    db_connection: std::rc::Rc<std::cell::RefCell<DbConnection>>,
}

#[derive(Debug)]
pub enum LoginMsg {
    UsernameChanged(String),
    PasswordChanged(String),
    LoginPressed,
}

#[derive(PartialEq, Clone)]
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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        LoginModel {
            dialog: DialogService::new(),
            username: props.current_username.unwrap_or_default(),
            password: String::new(),
            when_logged_in: props.when_logged_in,
            db_connection: props.db_connection.unwrap(),
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
                        self.dialog
                            .alert("Invalid password for the specified user.");
                    }
                } else {
                    self.dialog.alert("User not found.");
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
}

impl Renderable<LoginModel> for LoginModel {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <label>{ "User name: " }</label>
                    <input
                        type="text",
                        value=&self.username,
                        oninput=|e| LoginMsg::UsernameChanged(e.value),
                    />
                </div>
                <div>
                    <label>{ "Password: " }</label>
                    <input
                        type="password",
                        oninput=|e| LoginMsg::PasswordChanged(e.value),
                    />
                </div>
                <button
                    onclick=|_| LoginMsg::LoginPressed,>
                    { "Log in" }
                </button>
            </div>
        }
    }
}
