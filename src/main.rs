mod models;

use actix_web::{web, App, HttpServer, Responder};
use models::Status;
use std::io;

const HOST: &str = "127.0.0.1:8080";

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Starting Server at http://{}", HOST);
    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind(HOST)?
        .run()
        .await
}
