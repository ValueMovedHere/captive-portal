use actix_files::Files;
use actix_web::{App, HttpServer, Responder, get, web};
use clap::Parser;
use colored::Colorize;

mod cli;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    let port = args.port;
    println!("Listening on port {port}".green().bold());
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(Files::new("/login", "./pages"))
    })
    .bind(format!("localhost:{port}"))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    web::Redirect::to("/login")
}
