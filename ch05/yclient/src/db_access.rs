use serde_derive::{Deserialize, Serialize};

pub const BACKEND_SITE: &str = "http://localhost:8080/";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
