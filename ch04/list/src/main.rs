use actix_web::{http, server, App, Binary, HttpRequest, HttpResponse, Responder};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

mod db_access;

struct AppState {
    db_conn: Arc<Mutex<db_access::DbConnection>>,
}

fn get_main(_req: &HttpRequest<AppState>) -> impl Responder {
    let context = tera::Context::new();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("main.html", &context).unwrap())
}

fn get_page_persons(req: &HttpRequest<AppState>) -> impl Responder {
    let partial_name = req
        .query()
        .get("partial_name")
        .unwrap_or(&"".to_string())
        .clone();
    let db_conn = req.state().db_conn.lock().unwrap();
    let person_list = db_conn.get_persons_by_partial_name(&partial_name);
    let mut context = tera::Context::new();
    context.insert("partial_name", &partial_name);
    context.insert("persons", &person_list);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("persons.html", &context).unwrap())
}

fn get_favicon(_req: &HttpRequest<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/x-icon")
        .body(Binary::from_slice(include_bytes!("favicon.ico")))
}

fn invalid_resource(_req: &HttpRequest<AppState>) -> impl Responder {
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(Binary::from("<h2>Invalid request.</h2>"))
}

lazy_static! {
    pub static ref TERA: tera::Tera = { tera::compile_templates!("templates/**/*") };
}

fn main() {
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {}", server_address);
    let db_conn = Arc::new(Mutex::new(db_access::DbConnection::new()));
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
            r.method(http::Method::GET).f(get_page_persons);
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
