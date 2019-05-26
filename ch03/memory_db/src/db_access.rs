#[derive(Clone, Debug)]
pub struct Person {
    pub id: u32,
    pub name: String,
}

pub struct DbConnection {
    persons: Vec<Person>,
}

impl DbConnection {
    pub fn new() -> DbConnection {
        DbConnection { persons: vec![] }
    }

    pub fn get_all_persons_ids(&self) -> Vec<u32> {
        self.persons.iter().map(|p| p.id).collect()
    }

    pub fn get_person_name_by_id(&self, id: u32) -> Option<String> {
        Some(self.persons.iter().find(|p| p.id == id)?.name.clone())
    }

    pub fn get_persons_id_and_name_by_partial_name(&self, subname: &str) -> Vec<(u32, String)> {
        self.persons
            .iter()
            .filter(|p| p.name.contains(subname))
            .map(|p| (p.id, p.name.clone()))
            .collect()
    }

    pub fn insert_person(&mut self, name: String) -> u32 {
        let new_id = if self.persons.is_empty() {
            1
        } else {
            self.persons[self.persons.len() - 1].id + 1
        };
        self.persons.push(Person { id: new_id, name });
        new_id
    }
}
