use serde_derive::Deserialize;
use yew::services::fetch::Request;

pub const BACKEND_SITE: &str = "http://localhost:8080/";

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Copy, PartialEq, Debug, Deserialize)]
pub enum DbPrivilege {
    CanRead,
    CanWrite,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub privileges: Vec<DbPrivilege>,
}

pub fn add_auth<T>(username: &str, password: &str, request: &mut Request<T>) {
    let mut auth_string = "Basic ".to_string();
    base64::encode_config_buf(
        format!("{}:{}", username, password).as_bytes(),
        base64::STANDARD,
        &mut auth_string,
    );
    request
        .headers_mut()
        .append("authorization", auth_string.parse().unwrap());
}
