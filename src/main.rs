use actix_web::{App, HttpServer, Responder, get};
use clap::Parser;

mod cli;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    let port = args.port;
    println!("Listening on port {port}");
    HttpServer::new(|| App::new().service(index))
        .bind(format!("localhost:{port}"))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World"
}
