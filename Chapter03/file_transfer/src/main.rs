// Test it with the following commands:
// curl -X DELETE http://localhost:8080/datafile.txt
// curl -X GET http://localhost:8080/datafile.txt
// curl -X PUT http://localhost:8080/datafile.txt -d "File contents."
// curl -X POST http://localhost:8080/data -d "File contents."
// curl -X GET http://localhost:8080/a/b

use actix_web::Error;
use actix_web::{web, web::Path, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::{
    future::{ok, Future},
    Stream,
};
use rand::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

fn delete_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Deleting file \"{}\" ... ", filename);
    flush_stdout();

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

fn download_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Downloading file \"{}\" ... ", filename);
    flush_stdout();

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

fn upload_specified_file(
    payload: web::Payload,
    info: Path<(String,)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let filename = info.0.clone();

    print!("Uploading file \"{}\" ... ", filename);
    flush_stdout();

    // Get asynchronously from the client
    // the contents to write into the file.
    payload
        .map_err(Error::from)
        .fold(web::BytesMut::new(), move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
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
            ok(HttpResponse::Ok().finish())
        })
}

fn upload_new_file(
    payload: web::Payload,
    info: Path<(String,)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let filename_prefix = info.0.clone();
    print!("Uploading file \"{}*.txt\" ... ", filename_prefix);
    flush_stdout();

    payload
        .map_err(Error::from)
        .fold(web::BytesMut::new(), move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
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
            ok(HttpResponse::Ok().content_type("text/plain").body(filename))
        })
}

fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {} ...", server_address);
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{filename}")
                    .route(web::delete().to(delete_file))
                    .route(web::get().to(download_file))
                    .route(web::put().to_async(upload_specified_file))
                    .route(web::post().to_async(upload_new_file)),
            )
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_address)?
    .run()
}
