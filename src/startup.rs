use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgPool;
use crate::routes::*;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
            .app_data(connection.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}