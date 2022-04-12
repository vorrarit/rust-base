use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use rust_base::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read configuration");

    let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(3)
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}