fn main() {
    let mut tera_engine = tera::Tera::default();

    tera_engine
        .add_raw_template("id_template", "Identifier: {{id}}.")
        .unwrap();

    let mut numeric_id = tera::Context::new();
    numeric_id.insert("id", &7362);
    println!(
        "id_template with numeric_id: [{}]",
        tera_engine
            .render("id_template", &numeric_id.clone())
            .unwrap()
    );

    let mut textual_id = tera::Context::new();
    textual_id.insert("id", &"ABCD");
    println!(
        "id_template with textual_id: [{}]",
        tera_engine.render("id_template", &textual_id).unwrap()
    );

    tera_engine
        .add_raw_template("person_id_template", "Person id: {{person.id}}")
        .unwrap();

    #[derive(serde_derive::Serialize)]
    struct Person {
        id: u32,
        name: String,
    }

    let mut one_person = tera::Context::new();
    one_person.insert(
        "person",
        &Person {
            id: 534,
            name: "Mary".to_string(),
        },
    );
    println!(
        "person_id_template with one_person: [{}]",
        tera_engine
            .render("person_id_template", &one_person.clone())
            .unwrap()
    );

    tera_engine
        .add_raw_template(
            "possible_person_id_template",
            "{%if person%}Id: {{person.id}}\
             {%else%}No person\
             {%endif%}",
        )
        .unwrap();

    println!(
        "possible_person_id_template with one_person: [{}]",
        tera_engine
            .render("possible_person_id_template", &one_person)
            .unwrap()
    );

    println!(
        "possible_person_id_template with empty context: [{}]",
        tera_engine
            .render("possible_person_id_template", &tera::Context::new())
            .unwrap()
    );

    tera_engine
        .add_raw_template(
            "multiple_person_id_template",
            "{%for p in persons%}\
             Id: {{p.id}};\n\
             {%endfor%}",
        )
        .unwrap();

    let mut three_persons = tera::Context::new();
    three_persons.insert(
        "persons",
        &vec![
            Person {
                id: 534,
                name: "Mary".to_string(),
            },
            Person {
                id: 298,
                name: "Joe".to_string(),
            },
            Person {
                id: 820,
                name: "Ann".to_string(),
            },
        ],
    );
    println!(
        "multiple_person_id_template with three_persons: [{}]",
        tera_engine
            .render("multiple_person_id_template", &three_persons.clone())
            .unwrap()
    );

    tera_engine
        .add_template_file("templates/templ_id.txt", Some("id_file_template"))
        .unwrap();

    println!(
        "id_file_template with numeric_id: [{}]",
        tera_engine
            .render("id_file_template", &numeric_id.clone())
            .unwrap()
    );

    tera_engine
        .add_template_file("templates/templ_id.txt", None)
        .unwrap();

    println!(
        "templates/templ_id.txt with numeric_id: [{}]",
        tera_engine
            .render("templates/templ_id.txt", &numeric_id)
            .unwrap()
    );

    println!(
        "templates/templ_names.txt with numeric_id: [{}]",
        TERA.render("templ_names.txt", &three_persons).unwrap()
    );
}

lazy_static::lazy_static! {
    pub static ref TERA: tera::Tera =
        tera::Tera::new("templates/**/*").unwrap();
}
