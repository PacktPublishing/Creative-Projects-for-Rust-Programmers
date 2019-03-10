mod db_access;

struct AppState {
    db_conn: Arc<Mutex<db_access::DbConnection>>,
}

use actix_web::{http, App, server, Responder, HttpRequest, HttpResponse};
use std::sync::{Arc, Mutex};
use serde_json::json;

fn get_all_persons_ids(req: &HttpRequest<AppState>) -> impl Responder {
    println!("In get_all_persons_ids");
    let db_conn = req.state().db_conn.lock().unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(
            json!(db_conn
                .get_all_persons_ids()
                .iter()
                .map(|id|id)
                .collect::<Vec<_>>())
            .to_string())
}

fn get_person_name_by_id(req: &HttpRequest<AppState>) -> impl Responder {
    println!("In get_person_name_by_id");
    let info = req.match_info();
    let id = info.get_decoded("id").unwrap();
    let id = id.parse::<u32>();
    if id.is_err() { return HttpResponse::NotFound().finish(); }
    let id = id.unwrap();
    let db_conn = req.state().db_conn.lock().unwrap();
    if let Some(name) = db_conn.get_person_name_by_id(id) {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(json!(name).to_string())
    }
    else 
    {
        HttpResponse::NotFound().finish()
    }
}

fn get_persons_by_partial_name(req: &HttpRequest<AppState>) -> impl Responder {
    println!("In get_persons_by_partial_name");
    let info = req.match_info();
    let partial_name = info.get_decoded("partial_name").unwrap();
    let db_conn = req.state().db_conn.lock().unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json!(
            db_conn
                .get_persons_by_partial_name(&partial_name)
                .iter()
                .collect::<Vec<_>>()
            ).to_string())
}

fn insert_person(req: &HttpRequest<AppState>) -> impl Responder {
    println!("In insert_person");
    let info = req.match_info();
    let name = info.get_decoded("name").unwrap();
    let mut db_conn = req.state().db_conn.lock().unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json!(
            db_conn.insert_person(name)
            ).to_string())
}

fn invalid_resource(req: &HttpRequest<AppState>) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

fn main() {
    use http::Method;
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {}", server_address);
    let db_conn = Arc::new(Mutex::new(db_access::DbConnection::new()));
    server::new(
        move || App::with_state(AppState {
            db_conn: db_conn.clone()
        })
        .resource("/persons/ids",
            |r| r
                .method(Method::GET)
                .f(get_all_persons_ids))
        .resource("/person/name_by_id/{id}",
            |r| r
                .method(Method::GET)
                .f(get_person_name_by_id))
        .resource("/persons/by_partial_name/{partial_name}",
            |r| r
                .method(Method::GET)
                .f(get_persons_by_partial_name)
        )
        .resource("/person/{name}",
            |r| r
                .method(Method::POST)
                .f(insert_person))
        .default_resource(|r| r.f(invalid_resource))
    )
    .bind(server_address)
    .unwrap()
    .run();
}
