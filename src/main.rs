use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running");

    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| index()))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

async fn index() -> impl Responder {
    let html = fs::read_to_string("../wasm/index.html")
        .expect("Something went wrong reading the file");

    HttpResponse::Ok().body(html)
}