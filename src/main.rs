use actix_web::{App, HttpServer, Responder, get};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("localhost:8080")?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World"
}
