use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::sync::Mutex;

mod db_access;

struct AppState {
    db: db_access::DbConnection,
}

fn get_main() -> impl Responder {
    let context = tera::Context::new();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("main.html", &context).unwrap())
}

#[derive(Deserialize)]
pub struct Filter {
    partial_name: Option<String>,
}

fn get_page_persons(
    query: web::Query<Filter>,
    state: web::Data<Mutex<AppState>>,
) -> impl Responder {
    let partial_name = &query.partial_name.clone().unwrap_or_else(|| "".to_string());
    let db_conn = &state.lock().unwrap().db;
    let person_list = db_conn.get_persons_by_partial_name(&partial_name);
    let mut context = tera::Context::new();
    context.insert("partial_name", &partial_name);
    context.insert("persons", &person_list.collect::<Vec<_>>());
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("persons.html", &context).unwrap())
}

fn get_favicon() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/x-icon")
        .body(include_bytes!("favicon.ico") as &[u8])
}

fn invalid_resource() -> impl Responder {
    HttpResponse::NotFound()
        .content_type("text/html")
        .body("<h2>Invalid request.</h2>")
}

lazy_static! {
    pub static ref TERA: tera::Tera = tera::Tera::new("templates/**/*").unwrap();
}

fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {}", server_address);
    let db_conn = web::Data::new(Mutex::new(AppState {
        db: db_access::DbConnection::new(),
    }));
    HttpServer::new(move || {
        App::new()
            .register_data(db_conn.clone())
            .service(
                web::resource("/")
                    // Get the frame of the page to manage the persons.
                    // Such frame should request its body.
                    .route(web::get().to(get_main)),
            )
            .service(
                web::resource("/page/persons")
                    // Get the page to manage the persons,
                    // showing all the persons.
                    .route(web::get().to(get_page_persons)),
            )
            .service(
                web::resource("/favicon.ico")
                    // Get the App icon.
                    .route(web::get().to(get_favicon)),
            )
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_address)?
    .run()
}
