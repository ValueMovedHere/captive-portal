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
    let addr = if args.nonlocal {
        "0.0.0.0"
    } else {
        "localhost"
    };
    let port = args.port;
    let pages_dir = args.pages_path;
    let offline = args.offline;
    let msg = format!("Listening on {addr}:{port} in offline mode: ");
    println!(
        "{}{}",
        msg.bold().bright_green(),
        offline.to_string().bright_red().bold()
    );
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Conf { offline: offline }))
            .service(handlers::redirect_login)
            .service(submissions::submit)
            .service(Files::new("/login", &pages_dir).index_file("index.html"))
            .default_service(web::to(handlers::not_found))
    })
    .bind(format!("{addr}:{port}"))?
    .run()
    .await
}

struct Conf {
    offline: bool,
}
