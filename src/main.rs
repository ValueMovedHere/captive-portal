use actix_files::Files;
use actix_web::{App, HttpServer, web};
use clap::Parser;
use colored::Colorize;

mod cli;
mod handlers;
mod submissions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    let port = args.port;
    let pages_dir = args.pages_path;
    let msg = format!("Listening on port {port}");
    println!("{}", msg.bold().bright_green());
    HttpServer::new(move || {
        App::new()
            .service(handlers::redirect_login)
            .service(submissions::submit)
            .service(Files::new("/login", &pages_dir))
            .default_service(web::to(handlers::not_found))
    })
    .bind(format!("localhost:{port}"))?
    .run()
    .await
}
