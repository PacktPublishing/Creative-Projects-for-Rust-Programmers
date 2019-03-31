use actix_web::{http, server, App, HttpRequest, HttpResponse,
    Responder, Binary, FromRequest};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};

mod db_access;
use db_access::Person;
use db_access::DbPrivilege;

struct AppState {
    db_conn: Arc<Mutex<db_access::DbConnection>>,
}

fn check_credentials(req: &HttpRequest<AppState>,
    required_privilege: DbPrivilege)
    -> Result<Vec<DbPrivilege>, String>
{
    let mut config = Config::default();
    config.realm("PersonsApp");
    if let Ok(credentials) = BasicAuth::from_request(&req, &config) {
        let db_conn = req.state().db_conn.lock().unwrap();
        if let Some(user) = db_conn.get_user_by_username(
            credentials.username()) {
            if user.password == credentials.password().unwrap_or("") {
                if user.privileges.contains(&required_privilege) {
                    Ok(user.privileges.clone())
                }
                else {
                    Err(format!(
                        "Insufficient privileges for user \"{}\".", user.username))
                }
            }
            else {
                Err(format!("Invalid password for user \"{}\".", user.username))
            }
        }
        else {
            Err(format!("User \"{}\" not found.", credentials.username()))
        }
    }
    else {
        Err(format!("Credentials missing."))
    }
}

fn get_main(_req: &HttpRequest<AppState>) -> impl Responder {
    let context = tera::Context::new();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("main.html", &context).unwrap())
}

fn get_page_login(req: &HttpRequest<AppState>) -> HttpResponse {
    get_page_login_with_message(req, "")
}

fn get_page_login_with_message(_req: &HttpRequest<AppState>,
    error_message: &str) -> HttpResponse {
    let mut context = tera::Context::new();
    context.insert("error_message", error_message);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("login.html", &context).unwrap())
}

fn get_page_persons(req: &HttpRequest<AppState>) -> HttpResponse {
    match check_credentials(req, DbPrivilege::CanRead) {
        Ok(privileges) =>  {
            let partial_name = req.query().get("partial_name")
                .unwrap_or(&"".to_string()).clone();
            let db_conn = req.state().db_conn.lock().unwrap();
            let person_list = db_conn.get_persons_by_partial_name(&partial_name);
            let mut context = tera::Context::new();
            context.insert("id_error", &"");
            context.insert("partial_name", &partial_name);
            context.insert("persons", &person_list);
            context.insert("can_write",
                &privileges.contains(&DbPrivilege::CanWrite));
            HttpResponse::Ok()
                .content_type("text/html")
                .body(TERA.render("persons.html", &context).unwrap())
        },
        Err(msg) => get_page_login_with_message(req, &msg)
    }
}

fn delete_persons(req: &HttpRequest<AppState>) -> HttpResponse {
    match check_credentials(req, DbPrivilege::CanWrite) {
        Ok(_) =>  {
            let mut db_conn = req.state().db_conn.lock().unwrap();
            let mut deleted_count = 0;
            req
                .query()
                .get("id_list")
                .unwrap_or(&"".to_string())
                .split_terminator(',')
                .for_each(|id| {
                    deleted_count += if db_conn.delete_by_id(id.parse::<u32>().unwrap()) {
                        1
                    }
                    else {
                        0
                    };
                });
            HttpResponse::Ok()
                .content_type("text/plain")
                .body(deleted_count.to_string())
        },
        Err(msg) => get_page_login_with_message(req, &msg)
    }
}

fn get_page_new_person(req: &HttpRequest<AppState>) -> HttpResponse {
    match check_credentials(req, DbPrivilege::CanWrite) {
        Ok(privileges) =>  {
            let mut context = tera::Context::new();
            context.insert("person_id", &"");
            context.insert("person_name", &"");
            context.insert("inserting", &true);
            context.insert("can_write",
                &privileges.contains(&DbPrivilege::CanWrite));
            HttpResponse::Ok()
                .content_type("text/html")
                .body(TERA.render("one_person.html", &context).unwrap())
        },
        Err(msg) => get_page_login_with_message(req, &msg)
    }
}

fn get_page_edit_person(req: &HttpRequest<AppState>) -> HttpResponse {
    match check_credentials(req, DbPrivilege::CanRead) {
        Ok(privileges) =>  {
            let info = req.match_info();
            let id = info.get_decoded("id").unwrap();
            let db_conn = req.state().db_conn.lock().unwrap();
            let mut context = tera::Context::new();
            if let Ok(id_n) = id.parse::<u32>() {
                if let Some(person) = db_conn.get_person_by_id(id_n) {
                    context.insert("person_id", &id);
                    context.insert("person_name", &person.name);
                    context.insert("inserting", &false);
                    context.insert("can_write",
                        &privileges.contains(&DbPrivilege::CanWrite));
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
        },
        Err(msg) => get_page_login_with_message(req, &msg)
    }
}

fn insert_person(req: &HttpRequest<AppState>) -> HttpResponse {
    match check_credentials(req, DbPrivilege::CanWrite) {
        Ok(_) =>  {
            let mut db_conn = req.state().db_conn.lock().unwrap();
            let mut inserted_count = 0;
            if let Some(name) = req.query().get("name") {
                inserted_count += db_conn.insert_person(
                    Person { id: 0, name: name.clone() });
            }
            HttpResponse::Ok()
                .content_type("text/plain")
                .body(inserted_count.to_string())
        },
        Err(msg) => get_page_login_with_message(req, &msg)
    }
}

fn update_person(req: &HttpRequest<AppState>) -> HttpResponse {
    match check_credentials(req, DbPrivilege::CanWrite) {
        Ok(_) =>  {
            let mut db_conn = req.state().db_conn.lock().unwrap();
            let mut updated_count = 0;
            let id = req.query().get("id").unwrap_or(&"0".to_string())
                .parse::<u32>().unwrap();
            let name = req.query().get("name").unwrap_or(&"".to_string()).clone();
            updated_count += if db_conn.update_person(
                Person { id: id, name: name }) {
                    1
                }
                else {
                    0
                };
            HttpResponse::Ok()
                .content_type("text/plain")
                .body(updated_count.to_string())
        },
        Err(msg) => get_page_login_with_message(req, &msg)
    }
}

fn get_favicon(_req: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/x-icon")
        .body(Binary::from_slice(include_bytes!("favicon.ico")))
}

fn invalid_resource(_req: &HttpRequest<AppState>) -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("id_error", &"Invalid request.");
    context.insert("partial_name", &"");
    context.insert("persons", &Vec::<Person>::new());
    context.insert("can_write", &false);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("persons.html", &context).unwrap())
}

lazy_static! {
    pub static ref TERA: tera::Tera = {
        tera::compile_templates!("templates/**/*")
    };
}

fn main() {
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {}", server_address);
    let db_conn = Arc::new(Mutex::new(
        db_access::DbConnection::new()
    ));
    server::new(move || {
        App::with_state(AppState {
            db_conn: db_conn.clone(),
        })
        .resource("/", |r| {
            // Get the frame of the page to manage the persons.
            // Such frame should request its body.
            r.method(http::Method::GET).f(get_main);
        })
        .resource("/page/login", |r| {
            // Get the page to login into the Web app.
            r.method(http::Method::GET).f(get_page_login);
        })
        .resource("/page/persons", |r| {
            // Get the page to manage the persons,
            // showing all the persons whose name contains
            // a string specified in the query argument "partial_name".
            r.method(http::Method::GET).f(get_page_persons);
        })
        .resource("/persons", |r| {
            // Delete the persons specified in the query argument
            // "id_to_delete" as a comma-separated list of numbers,
            // and return the number of persons deleted.
            r.method(http::Method::DELETE).f(delete_persons);
        })
        .resource("/page/new_person", |r| {
            // Get the page to insert a new person.
            r.method(http::Method::GET).f(get_page_new_person);
        })
        .resource("/page/edit_person/{id}", |r| {
            // Get the page to show and edit the existing person
            // having the specified id,
            // or a NotFound state if does not exist.
            r.method(http::Method::GET).f(get_page_edit_person);
        })
        .resource("/one_person", |r| {
            // Insert a person having as name the string specified
            // in the query argument "name",
            // and return the number of persons inserted (1 or 0).
            r.method(http::Method::POST).f(insert_person);
            // Save the person having the id specified in the path
            // setting as its name the string specified
            // in the query argument "name",
            // and return the number of persons updated (1 or 0).
            r.method(http::Method::PUT).f(update_person);
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
