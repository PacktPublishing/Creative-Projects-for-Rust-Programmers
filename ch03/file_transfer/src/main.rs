// Test it with the following commands:
// curl -X DELETE http://localhost:8080/datafile.txt
// curl -X GET http://localhost:8080/datafile.txt
// curl -X PUT http://localhost:8080/datafile.txt -d "File contents."
// curl -X POST http://localhost:8080/data -d "File contents."
// curl -X GET http://localhost:8080/a/b

use actix_web::{http, server, App, HttpRequest, HttpResponse, Responder};
use actix_web::{AsyncResponder, Error, HttpMessage};
use futures::future::{ok, Future};
use rand::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn delete_file(req: &HttpRequest) -> impl Responder {
    let info = req.match_info();
    let filename = info.get_decoded("filename").unwrap();
    print!("Deleting file \"{}\" ... ", filename);

    // Delete the file.
    match std::fs::remove_file(&filename) {
        Ok(_) => {
            println!("Deleted file \"{}\"", filename);
            HttpResponse::Ok()
        }
        Err(error) => {
            println!("Failed to delete file \"{}\": {}", filename, error);
            HttpResponse::NotFound()
        }
    }
}

fn download_file(req: &HttpRequest) -> impl Responder {
    let info = req.match_info();
    let filename = info.get_decoded("filename").unwrap();
    print!("Downloading file \"{}\" ... ", filename);

    fn read_file_contents(filename: &str) -> std::io::Result<String> {
        use std::io::Read;
        let mut contents = String::new();
        File::open(filename)?.read_to_string(&mut contents)?;
        Ok(contents)
    }

    match read_file_contents(&filename) {
        Ok(contents) => {
            println!("Downloaded file \"{}\"", filename);
            HttpResponse::Ok().content_type("text/plain").body(contents)
        }
        Err(error) => {
            println!("Failed to read file \"{}\": {}", filename, error);
            HttpResponse::NotFound().finish()
        }
    }
}

fn upload_specified_file(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let info = req.match_info();
    let filename = info.get_decoded("filename").unwrap();
    print!("Uploading file \"{}\" ... ", filename);

    // Get asynchronously from the client
    // the contents to write into the file.
    req.body()
        .from_err()
        .and_then(move |contents| {
            // Create the file.
            let f = File::create(&filename);
            if f.is_err() {
                println!("Failed to create file \"{}\"", filename);
                return ok(HttpResponse::NotFound().into());
            }

            // Write the contents into it.
            if f.unwrap().write_all(&contents).is_err() {
                println!("Failed to write file \"{}\"", filename);
                return ok(HttpResponse::NotFound().into());
            }

            println!("Uploaded file \"{}\"", filename);
            ok(HttpResponse::Ok().into())
        })
        .responder()
}

fn upload_new_file(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let info = req.match_info();
    let filename_prefix = info.get_decoded("filename").unwrap();
    print!("Uploading file \"{}*.txt\" ... ", filename_prefix);

    // Get asynchronously from the client
    // the contents to write into the file.
    req.body()
        .from_err()
        .and_then(move |contents| {
            let mut rng = rand::thread_rng();
            let mut attempts = 0;
            let mut file;
            let mut filename;
            const MAX_ATTEMPTS: u32 = 100;

            loop {
                attempts += 1;
                if attempts > MAX_ATTEMPTS {
                    println!(
                        "Failed to create new file with prefix \"{}\", \
                         after {} attempts.",
                        filename_prefix, MAX_ATTEMPTS
                    );
                    return ok(HttpResponse::NotFound().into());
                }

                // Generate a 3-digit pseudo-random number.
                // and use it to create a file name.
                filename = format!("{}{:03}.txt", filename_prefix, rng.gen_range(0, 1000));

                // Create a not-yet-existing file.
                file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&filename);

                // If it was created, exit the loop.
                if file.is_ok() {
                    break;
                }
            }

            // Write the contents into it synchronously.
            if file.unwrap().write_all(&contents).is_err() {
                println!("Failed to write file \"{}\"", filename);
                return ok(HttpResponse::NotFound().into());
            }
            println!("Uploaded file \"{}\"", filename);

            ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body(filename)
                .into())
        })
        .responder()
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
