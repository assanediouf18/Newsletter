use actix_web::{App, HttpResponse, HttpServer, Responder, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String
}

pub async fn subscriptions(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}