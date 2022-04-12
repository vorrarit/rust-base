use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgPool;

mod routes;

use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing::subscriber::set_global_default;
use tracing_actix_web::TracingLogger;

pub mod configuration;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("rust_20220328".into(), std::io::stdout);
    let subscriber = Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(routes::hello)
            .service(routes::echo)
            .route("/hey", web::get().to(routes::manual_hello))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}