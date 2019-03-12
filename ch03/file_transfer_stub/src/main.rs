// Test it with the following commands:
// curl -X DELETE http://localhost:8080/datafile.txt
// curl -X GET http://localhost:8080/datafile.txt
// curl -X PUT http://localhost:8080/datafile.txt -d "File contents."
// curl -X POST http://localhost:8080/data -d "File contents."
// curl -X GET http://localhost:8080/a/b
//
// after running the second command, the client should have printed:
// Contents of the file.
//
// After running all five commands, the server should have printed:
// Listening at address 127.0.0.1:8080 ...
// Deleting file "datafile.txt" ... Deleted file "datafile.txt"
// Downloading file "datafile.txt" ... Downloaded file "datafile.txt"
// Uploading file "datafile.txt" ... Uploaded file "datafile.txt"
// Uploading file "data*.txt" ... Uploaded file "data917.txt"
// Invalid URI: "/a/b"

use actix_web::{http, server, App, HttpRequest, HttpResponse, Responder};

fn delete_file(req: &HttpRequest) -> impl Responder {
    let info = req.match_info();
    let filename = info.get_decoded("filename").unwrap();
    print!("Deleting file \"{}\" ... ", filename);

    // TODO: Delete the file.

    println!("Deleted file \"{}\"", filename);
    HttpResponse::Ok()
}

fn download_file(req: &HttpRequest) -> impl Responder {
    let info = req.match_info();
    let filename = info.get_decoded("filename").unwrap();
    print!("Downloading file \"{}\" ... ", filename);

    // TODO: Read the contents of the file.
    let contents = "Contents of the file.\n".to_string();

    println!("Downloaded file \"{}\"", filename);
    HttpResponse::Ok().content_type("text/plain").body(contents)
}

fn upload_specified_file(req: &HttpRequest) -> impl Responder {
    let info = req.match_info();
    let filename = info.get_decoded("filename").unwrap();
    print!("Uploading file \"{}\" ... ", filename);

    // TODO: Get from the client the contents to write into the file.
    let _contents = "Contents of the file.\n".to_string();

    // TODO: Create the file and write the contents into it.

    println!("Uploaded file \"{}\"", filename);
    HttpResponse::Ok()
}

fn upload_new_file(req: &HttpRequest) -> impl Responder {
    let info = req.match_info();
    let filename_prefix = info.get_decoded("filename").unwrap();
    print!("Uploading file \"{}_*.txt\" ... ", filename_prefix);

    // TODO: Get from the client the contents to write into the file.
    let _contents = "Contents of the file.\n".to_string();

    // TODO: Generate new filename and create that file.
    let file_id = 17;

    let filename = format!("{}_{}.txt", filename_prefix, file_id);

    // TODO: Write the contents into the file.

    println!("Uploaded file \"{}\"", filename);
    HttpResponse::Ok()
}

fn invalid_resource(req: &HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

fn main() {
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {} ...", server_address);
    server::new(move || {
        App::new()
            .resource("/{filename}", |r| {
                r.method(http::Method::DELETE).f(delete_file);
                r.method(http::Method::GET).f(download_file);
                r.method(http::Method::PUT).f(upload_specified_file);
                r.method(http::Method::POST).f(upload_new_file);
            })
            .default_resource(|r| r.f(invalid_resource))
    })
    .bind(server_address)
    .unwrap()
    .run();
}
