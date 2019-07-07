mod db_access;

use actix_web::{http, web, web::Path, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_derive::Deserialize;
use serde_json::json;
use std::sync::Mutex;

struct AppState {
    db: db_access::DbConnection,
}

fn get_all_persons_ids(state: web::Data<Mutex<AppState>>) -> impl Responder {
    println!("In get_all_persons_ids");
    let db_conn = &state.lock().unwrap().db;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json!(db_conn.get_all_persons_ids().collect::<Vec<_>>()).to_string())
}

/*
fn get_person_name_by_id(
    state: web::Data<Mutex<AppState>>,
    info: Path<(String,)>,
) -> impl Responder {
    println!("In get_person_name_by_id");
    let id = &info.0;
    let id = id.parse::<u32>();
    if id.is_err() {
        return HttpResponse::NotFound().finish();
    }
    let id = id.unwrap();
    let db_conn = &state.lock().unwrap().db;
    if let Some(name) = db_conn.get_person_name_by_id(id) {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(json!(name).to_string())
    } else {
        HttpResponse::NotFound().finish()
    }
}
*/

fn get_person_by_id(
    state: web::Data<Mutex<AppState>>,
    info: Path<(u32,)>,
) -> impl Responder {
    println!("In get_person_by_id");
    let id = info.0;
    let db_conn = &state.lock().unwrap().db;
    if let Some(person) = db_conn.get_person_by_id(id) {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(json!(person).to_string())
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[derive(Deserialize)]
pub struct Filter {
    partial_name: Option<String>,
}

fn get_persons(
    state: web::Data<Mutex<AppState>>,
    query: web::Query<Filter>
) -> impl Responder {
    println!("In get_persons");
    let db_conn = &state.lock().unwrap().db;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(
            json!(db_conn
                .get_persons_by_partial_name(
                    &query.partial_name.clone().unwrap_or_else(|| "".to_string()),
                )
                .collect::<Vec<_>>())
            .to_string(),
        )
}

#[derive(Deserialize)]
pub struct ToDelete {
    id_list: Option<String>,
}

fn delete_persons(
    state: web::Data<Mutex<AppState>>,
    query: web::Query<ToDelete>
) -> impl Responder {
    println!("In delete_persons: {:?}", query.id_list);
    let db_conn = &mut state.lock().unwrap().db;
    let mut deleted_count = 0;
    query
        .id_list
        .clone()
        .unwrap_or_else(|| "".to_string())
        .split_terminator(',')
        .for_each(|id| {
            deleted_count += if db_conn.delete_by_id(id.parse::<u32>().unwrap()) {
                1
            } else {
                0
            };
        });
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json!(deleted_count).to_string())
}

#[derive(Deserialize)]
pub struct PersonData {
    id: Option<String>,
    name: Option<String>,
}

fn insert_person(
    state: web::Data<Mutex<AppState>>,
    query: web::Query<PersonData>
) -> impl Responder {
    println!("In insert_person");
    let db_conn = &mut state.lock().unwrap().db;
    let name = query.name.clone().unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json!(db_conn.insert_person(db_access::Person { id: 0, name })).to_string())
}

fn update_person(
    state: web::Data<Mutex<AppState>>,
    query: web::Query<PersonData>
) -> impl Responder {
    let db_conn = &mut state.lock().unwrap().db;
    let id = query.id.clone().unwrap();
    let id = str::parse::<u32>(&id).unwrap();
    let name = query.name.clone().unwrap();
    println!("In update_person: id={:?}, name={:?}", id, name);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json!(db_conn.update_person(db_access::Person { id, name })).to_string())
}

fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

fn invalid_method(req: HttpRequest) -> impl Responder {
    println!("Invalid method {} for URI: \"{}\"", req.method(), req.uri());
    HttpResponse::NotFound()
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
            .wrap(
                actix_cors::Cors::new()
                    //.allowed_origin("http://localhost:8000/")
                    //.allowed_origin("*")
                    //.allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_methods(vec!["POST", "PUT", "DELETE"])
                    //.allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    //.allowed_header(http::header::CONTENT_TYPE)
                    //.max_age(3600)
            )

            //curl -X GET http://localhost:8080/persons/ids
            .service(web::resource("/persons/ids")
                .route(web::get().to(get_all_persons_ids))
                .default_service(web::route().to(invalid_method))
            )
            //curl -X GET http://localhost:8080/person/id/1
            /*
            .wrap(
                actix_cors::Cors::new()
                    .allowed_methods(vec!["DELETE"])
            )
            */
            .service(web::resource("/person/id/{id}")
                .route(web::get().to(get_person_by_id))
                .default_service(web::route().to(invalid_method))
            )
            //curl -X GET http://localhost:8080/persons?partial_name=i
            /*
            .wrap(
                actix_cors::Cors::new()
                    .allowed_methods(vec!["DELETE"])
            )
            */
            //DELETE
            //curl -X DELETE http://localhost:8080/persons?ids=1,3
            /*
            .wrap(
                actix_cors::Cors::new()
                    .allowed_methods(vec!["DELETE"])
            )
            */
            .service(web::resource("/persons")
                .route(web::get().to(get_persons))
                //.route(web::head().to(|| HttpResponse::MethodNotAllowed()))
                .route(web::delete().to(delete_persons))
                .default_service(web::route().to(invalid_method))
            )

            /*
            .wrap(
                actix_cors::Cors::new()
                    .allowed_methods(vec!["DELETE"])
            )
            */
            .service(web::resource("/one_person")
                //curl -X POST http://localhost:8080/one_person?name=Juliet
                .route(web::post().to(insert_person))
                //curl -X PUT http://localhost:8080/one_person?id=1&name=Julia
                .route(web::put().to(update_person))
                .default_service(web::route().to(invalid_method))
            )
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_address)?
    .run()
}
/*
    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
    pub fn get_person_by_id(&self, id: u32) -> Option<&Person> {
    pub fn get_persons_by_partial_name(&self, subname: &str) -> Vec<Person> {
    pub fn delete_by_id(&mut self, id: u32) -> bool {
    pub fn insert_person(&mut self, mut person: Person) -> u32 {
    pub fn update_person(&mut self, person: Person) -> bool {

            .service(
                web::resource("/persons")
                    .route(web::delete().to(delete_persons)),
            )
            .service(
                web::resource("/one_person")
                    .route(web::post().to(insert_person))
                    .route(web::put().to(update_person)),
            )
*/
