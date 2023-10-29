mod configuration;
mod routes;
mod startup;

use actix_web::{App, HttpServer, Responder, web};
use actix_web::dev::Server;
use std::net::TcpListener;
use routes::*;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
    })
        .listen(listener)?
        .run();
    Ok(server)
}