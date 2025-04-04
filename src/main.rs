use rust_zero_to_production::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "Rust Zero to Production".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read_configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let db_connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&db_connection_string.expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind(&address)?;
    let _ = run(listener, connection_pool)?.await;
    Ok(())
}
