use actix_web::{http, server, App, HttpRequest, HttpResponse, Responder, Binary};
//use serde_json::json;
use std::sync::{Arc, Mutex};
use actix_web::HttpMessage;
use lazy_static::lazy_static;

mod db_access;
use db_access::Person;

struct AppState {
    db_conn: Arc<Mutex<db_access::DbConnection>>,
}

fn get_main(req: &HttpRequest<AppState>) -> impl Responder {
    println!("### get_main");
    println!("Path: \"{}\"", req.path());
    println!("Method: \"{}\"", req.method());
    println!("Content type: \"{}\"", req.content_type());
    println!("Headers: \"{:?}\"", req.headers());
    //println!("Payload: \"{:?}\"", req.payload());
    println!("Query string: \"{}\"\n", req.query_string());
    //let db_conn = req.state().db_conn.lock().unwrap();
    let context = tera::Context::new();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("main.html", &context).unwrap())
}

fn get_page_persons(req: &HttpRequest<AppState>) -> impl Responder {
    println!("### get_persons");
    println!("Path: \"{}\"", req.path());
    println!("Method: \"{}\"", req.method());
    println!("Content type: \"{}\"", req.content_type());
    println!("Headers: \"{:?}\"", req.headers());
    //println!("Payload: \"{:?}\"", req.payload());
    println!("Query string: \"{}\"\n", req.query_string());
    
    let partial_name = req.query().get("partial_name")
        .or(Some(&"".to_string())).unwrap().clone();
    let db_conn = req.state().db_conn.lock().unwrap();
    let person_list = db_conn.get_persons_by_partial_name(&partial_name);
    println!("partial_name: {}, person_list: {:?}.", partial_name, person_list);

    let mut context = tera::Context::new();
    context.insert("id_error", &"");
    context.insert("partial_name", &partial_name);
    context.insert("persons", &person_list);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("persons.html", &context).unwrap())
}

fn delete_persons(req: &HttpRequest<AppState>) -> impl Responder {
    println!("### delete_persons");
    println!("Path: \"{}\"", req.path());
    println!("Method: \"{}\"", req.method());
    println!("Content type: \"{}\"", req.content_type());
    println!("Headers: \"{:?}\"", req.headers());
    //println!("Payload: \"{:?}\"", req.payload());
    println!("Query string: \"{}\"\n", req.query_string());
    let mut db_conn = req.state().db_conn.lock().unwrap();
    let mut deleted_count = 0;
    req
        .query()
        .get("id_list")
        .or(Some(&"".to_string()))
        .unwrap()
        .split_terminator(',')
        .for_each(|id| {
            println!("Deleting: '{}'", id);
            deleted_count += if db_conn.delete_by_id(id.parse::<u32>().unwrap()) {
                1
            }
            else {
                0
            };
        });
    deleted_count.to_string()
}

fn get_page_new_person(req: &HttpRequest<AppState>) -> impl Responder {
    println!("### get_person_new");
    println!("Path: \"{}\"", req.path());
    println!("Method: \"{}\"", req.method());
    println!("Content type: \"{}\"", req.content_type());
    println!("Headers: \"{:?}\"", req.headers());
    //println!("Payload: \"{:?}\"", req.payload());
    println!("Query string: \"{}\"\n", req.query_string());

    let mut context = tera::Context::new();
    context.insert("person_id", &"");
    context.insert("person_name", &"");
    context.insert("inserting", &true);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("one_person.html", &context).unwrap())
}

fn get_page_edit_person(req: &HttpRequest<AppState>) -> impl Responder {
    let info = req.match_info();
    let id = info.get_decoded("id").unwrap();
    println!("### get_person_by_id: {}", id);
    println!("Path: \"{}\"", req.path());
    println!("Method: \"{}\"", req.method());
    println!("Content type: \"{}\"", req.content_type());
    println!("Headers: \"{:?}\"", req.headers());
    //println!("Payload: \"{:?}\"", req.payload());
    println!("Query string: \"{}\"\n", req.query_string());
    let db_conn = req.state().db_conn.lock().unwrap();
    let mut context = tera::Context::new();
    if let Ok(id_n) = id.parse::<u32>() {
        if let Some(person) = db_conn.get_person_by_id(id_n) {
            context.insert("person_id", &id);
            context.insert("person_name", &person.name);
            context.insert("inserting", &false);
            return HttpResponse::Ok()
                .content_type("text/html")
                .body(TERA.render("one_person.html", &context).unwrap());
        }
    }
    let person_list = db_conn.get_persons_by_partial_name(&"");

    context.insert("id_error", &"Person id not found");
    context.insert("partial_name", &"");
    context.insert("persons", &person_list);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("persons.html", &context).unwrap())
}

fn insert_person(req: &HttpRequest<AppState>) -> impl Responder {
    println!("### insert_person");
    println!("Path: \"{}\"", req.path());
    println!("Method: \"{}\"", req.method());
    println!("Content type: \"{}\"", req.content_type());
    println!("Headers: \"{:?}\"", req.headers());
    println!("Payload: \"{:?}\"", req.payload());
    println!("Query string: \"{}\"\n", req.query_string());
    let mut db_conn = req.state().db_conn.lock().unwrap();
    let mut inserted_count = 0;
    if let Some(name) = req.query().get("name") {
        inserted_count += db_conn.insert_person(
            Person { id: 0, name: name.clone() });
    }
    inserted_count.to_string()
}

fn update_person(req: &HttpRequest<AppState>) -> impl Responder {
    println!("### update_person");
    println!("Path: \"{}\"", req.path());
    println!("Method: \"{}\"", req.method());
    println!("Content type: \"{}\"", req.content_type());
    println!("Headers: \"{:?}\"", req.headers());
    //println!("Payload: \"{:?}\"", req.payload());
    println!("Query string: \"{}\"\n", req.query_string());
    let mut db_conn = req.state().db_conn.lock().unwrap();
    let mut updated_count = 0;
    let id = req.query().get("id").or(Some(&"0".to_string()))
        .unwrap().parse::<u32>().unwrap();
    let name = req.query().get("name").or(Some(&"".to_string())).unwrap().clone();
    updated_count += if db_conn.update_person(
        Person { id: id, name: name }) {
            1
        }
        else {
            0
        };
    updated_count.to_string()
}

fn get_favicon(_req: &HttpRequest<AppState>) -> impl Responder {
    println!("### get_favicon");
    HttpResponse::Ok()
        .content_type("image/x-icon")
        .body(Binary::from_slice(include_bytes!("favicon.ico")))
}

fn invalid_resource(req: &HttpRequest<AppState>) -> impl Responder {
    use actix_web::HttpMessage;
    println!("### invalid_resource: \"{}\"", req.uri());
    println!("Path: \"{}\"", req.path());
    println!("Method: \"{}\"", req.method());
    println!("Content type: \"{}\"", req.content_type());
    println!("Headers: \"{:?}\"", req.headers());
    //println!("Payload: \"{:?}\"", req.payload());
    println!("Query string: \"{}\"\n", req.query_string());

    let db_conn = req.state().db_conn.lock().unwrap();    
    let mut context = tera::Context::new();
    context.insert("id_error", &"Invalid request.");
    context.insert("partial_name", &"");
    let person_list = db_conn.get_persons_by_partial_name(&"");
    context.insert("persons", &person_list);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("persons.html", &context).unwrap())
}

lazy_static! {
    pub static ref TERA: tera::Tera = {
        tera::compile_templates!("src/*.html")
    };
}

fn main() {
    /*
    use sodiumoxide::crypto::secretbox;
    sodiumoxide::init();
    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();
    println!("{} {}", std::mem::size_of_val(&key), std::mem::size_of_val(&nonce));
    let plaintext = "1234567890:carlo.milanesi";
    println!("[{}]", plaintext);
    let ciphertext = secretbox::seal(plaintext.as_bytes(), &nonce, &key);
    println!("[{}]", base64::encode(&ciphertext));
    let their_plaintext = std::str::from_utf8(&secretbox::open(&ciphertext, &nonce, &key).unwrap()).unwrap().to_string();
    println!("[{}]", their_plaintext);
    assert!(plaintext == &their_plaintext[..]);
    return;
    */

    //use http::Method;
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {}", server_address);
    let db_conn = Arc::new(Mutex::new(
        db_access::DbConnection::new()
    ));
    // GET    /persons            Go to the persons page, with the persons filtered
    //        query: partial_name
    // GET    /new_person        Go to the one-person page, to insert a new person
    //        query: ---
    // GET    /edit_person/{id}    Go to the one-person page, to show/edit a specifid existing person
    //        query: ---
    // POST    /one_person         Insert a person, and go to persons page
    //        query: ---
    // PUT     /one_person         Update a person, and go to persons page
    //        query: partial_name
    // DELETE  /persons            Delete all the selected persons, and go to persons page
    //        query: partial_name
    server::new(move || {
        App::with_state(AppState {
            db_conn: db_conn.clone(),
        })
        .resource("/", |r| {
            // Get the frame of the page to manage the persons.
            // Such frame should request its body.
            r.method(http::Method::GET).f(get_main);
        })
        .resource("/page/persons", |r| {
            // Get the page to manage the persons,
            // showing all the persons whose name contains
            // a string specified in the query argument "partial_name".
            r.method(http::Method::GET).f(get_page_persons);//TODO
        })
        .resource("/persons", |r| {
            // Delete the persons specified in the query argument
            // "id_to_delete" as a comma-separated list of numbers,
            // and return the number of persons deleted.
            r.method(http::Method::DELETE).f(delete_persons);//TODO
        })
        .resource("/page/new_person", |r| {
            // Get the page to insert a new person.
            r.method(http::Method::GET).f(get_page_new_person);//TODO
        })
        .resource("/page/edit_person/{id}", |r| {
            // Get the page to show and edit the existing person
            // having the specified id,
            // or a NotFound state if does not exist.
            r.method(http::Method::GET).f(get_page_edit_person);//TODO
        })
        .resource("/one_person", |r| {
            // Insert a person having as name the string specified
            // in the query argument "name",
            // and return the number of persons inserted (1 or 0).
            r.method(http::Method::POST).f(insert_person);//TODO
            // Save the person having the id specified in the path
            // setting as its name the string specified
            // in the query argument "name",
            // and return the number of persons updated (1 or 0).
            r.method(http::Method::PUT).f(update_person);//TODO
        })
        .resource("/favicon.ico", |r| {
            // Get the App icon.
            r.method(http::Method::GET).f(get_favicon);
        })
        .default_resource(|r| r.f(invalid_resource))
    })
    .bind(server_address)
    .unwrap()
    .run();
}
