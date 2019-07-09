use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DbPrivilege {
    CanRead,
    CanWrite,
}

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub username: String,
    pub password: String,
    pub privileges: Vec<DbPrivilege>,
}

#[derive(PartialEq, Clone)]
pub struct DbConnection {
    persons: Vec<Person>,
    users: Vec<User>,
}

impl DbConnection {
    pub fn new() -> DbConnection {
        DbConnection {
            persons: vec![],
            // /*
            users: vec![
                User {
                    username: "joe".to_string(),
                    password: "xjoe".to_string(),
                    privileges: vec![DbPrivilege::CanRead],
                },
                User {
                    username: "susan".to_string(),
                    password: "xsusan".to_string(),
                    privileges: vec![DbPrivilege::CanRead, DbPrivilege::CanWrite],
                },
            ],
            // */
        }
    }

// /*
    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        if let Some(u) = self.users.iter().find(|u| u.username == username) {
            Some(u)
        } else {
            None
        }
    }
// */
}
