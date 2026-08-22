use actix_web::{HttpResponse, Responder, get, http::header, web};

#[get("/")]
pub async fn redirect_login() -> impl Responder {
    web::Redirect::to("/login/login.html")
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::Found()
        .insert_header((header::LOCATION, "/login/not_found.html"))
        .finish()
}
