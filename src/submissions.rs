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
pub async fn submit(web::Form(form): web::Form<InfoForm>) -> impl Responder {
    HttpResponse::Ok().body("Success")
}
