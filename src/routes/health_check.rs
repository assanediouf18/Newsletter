use actix_web::{App, HttpResponse, HttpServer, Responder, web};
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}