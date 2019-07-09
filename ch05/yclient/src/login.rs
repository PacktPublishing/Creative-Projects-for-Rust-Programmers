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
    pub username: String,
    pub password: String,
    pub when_logged_in: Option<Callback<User>>,
    pub db_connection: Option<std::rc::Rc<std::cell::RefCell<DbConnection>>>,
}

impl Default for LoginProps {
    fn default() -> Self {
        LoginProps {
            username: "".to_string(),
            password: "".to_string(),
            when_logged_in: None,
            db_connection: None,
        }
    }
}

//#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DbPrivilege {
    CanRead,
    CanWrite,
}

impl Component for LoginModel {
    type Message = LoginMsg;
    type Properties = LoginProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        LoginModel {
            dialog: DialogService::new(),
            username: props.username,
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
                if self.username.is_empty() {
                    self.dialog.alert("User not specified.");
                    return false;
                }
                /*
                self.fetching = true;
                self.ft = Some({
                    let callback = self.link.send_back(
                        move |response: Response<Json<Result<Vec<Privileges>, Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            if meta.status.is_success() {
                                PersonsListMsg::ReadyFilteredPersons(data)
                            } else {
                                PersonsListMsg::Failure(
                                    format!("No persons found."))
                            }
                        },
                    );

                    let request = Request::get(
                        "http://localhost:8080/persons?partial_name=".to_string() +
                        &url::form_urlencoded::byte_serialize(
                            self.name_portion.as_bytes()).collect::<String>()
                        ).body(Nothing).unwrap();
                    
                    self.fetch_service.fetch(request, callback)
                });
                */



                //TODO fetch

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
                    self.dialog.alert("User not specified.");
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.username = props.username;
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
