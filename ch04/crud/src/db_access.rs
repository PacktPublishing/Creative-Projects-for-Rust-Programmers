use serde_derive::Serialize;

#[derive(Serialize, Clone, Debug)]
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

    pub fn get_person_by_id(&self, id: u32) -> Option<&Person> {
        if let Some(p) = self.persons.iter().find(|p| p.id == id) {
            Some(p)
        } else {
            None
        }
    }

    pub fn get_persons_by_partial_name<'a>(
        &'a self,
        subname: &'a str,
    ) -> impl Iterator<Item = &Person> + 'a {
        self.persons
            .iter()
            .filter(move |p| p.name.contains(subname))
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
