use std::net::TcpListener;
use newsletter::startup::*;
use newsletter::configuration::*;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to load configurations");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let connection_string = configuration.database.connection_string();
    let connection = PgPool::connect(&connection_string).await.expect("Failed to connect to database");
    run(listener, connection)?.await
}
