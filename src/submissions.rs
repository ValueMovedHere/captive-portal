use actix_web::{HttpResponse, http::header, post, web};
use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InfoForm {
    phone: String,
    name: String,
    email: String,
    agree: String,
}

#[post("/auth/login")]
pub async fn submit(form: web::Form<InfoForm>) -> HttpResponse {
    println!("{}", "[!] New submission".blue().bold());
    print!(
        "\tName: {}\nPhone: {}\nEmail: {}",
        form.name, form.phone, form.email
    );
    HttpResponse::Found()
        .insert_header((header::LOCATION, "/success.html"))
        .finish()
}
