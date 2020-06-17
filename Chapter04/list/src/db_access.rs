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
        DbConnection {
            persons: vec![
                Person {
                    id: 2,
                    name: "Hamlet".to_string(),
                },
                Person {
                    id: 4,
                    name: "Macbeth".to_string(),
                },
                Person {
                    id: 7,
                    name: "Othello".to_string(),
                },
            ],
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
}
