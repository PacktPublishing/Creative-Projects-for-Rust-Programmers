#[derive(Clone, Debug, PartialEq)]
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
        }
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        if let Some(u) = self.users.iter().find(|u| u.username == username) {
            Some(u)
        } else {
            None
        }
    }

    pub fn get_person_by_id(&self, id: u32) -> Option<&Person> {
        if let Some(p) = self.persons.iter().find(|p| p.id == id) {
            Some(p)
        } else {
            None
        }
    }

    pub fn get_persons_by_partial_name(&self, subname: &str) -> Vec<Person> {
        self.persons
            .iter()
            .filter(|p| p.name.contains(subname))
            .cloned()
            .collect()
    }

    pub fn delete_by_id(&mut self, id: u32) -> bool {
        if let Some((n, _)) = self.persons.iter().enumerate().find(|(_, p)| p.id == id) {
            self.persons.remove(n);
            true
        } else {
            false
        }
    }

    pub fn insert_person(&mut self, mut person: Person) -> u32 {
        let new_id = if self.persons.is_empty() {
            1
        } else {
            self.persons[self.persons.len() - 1].id + 1
        };
        person.id = new_id;
        self.persons.push(person);
        new_id
    }

    pub fn update_person(&mut self, person: Person) -> bool {
        if let Some((n, _)) = self
            .persons
            .iter()
            .enumerate()
            .find(|(_, p)| p.id == person.id)
        {
            self.persons[n] = person;
            true
        } else {
            false
        }
    }
}
