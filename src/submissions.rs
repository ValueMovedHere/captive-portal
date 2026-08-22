use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InfoForm {
    phone: String,
    name: String,
    email: String,
    agree: String,
}

#[post("/auth/login")]
pub async fn submit(form: web::Form<InfoForm>) -> impl Responder {
    println!("Name: {}", form.name);
    HttpResponse::Ok().body("Success")
}
