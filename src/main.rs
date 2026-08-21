use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use clap::Parser;
use colored::Colorize;

mod cli;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    let port = args.port;
    let msg = format!("Listening on port {port}");
    println!("{}", msg.bold().bright_green());
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(Files::new("/login", "./pages"))
            .default_service(web::to(fallback))
    })
    .bind(format!("localhost:{port}"))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    web::Redirect::to("/login/login.html")
}

async fn fallback() -> HttpResponse {
    HttpResponse::Found()
        .insert_header(("Location", "/not_found.html"))
        .finish()
}
