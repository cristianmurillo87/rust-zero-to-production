use env_logger::Env;
use rust_zero_to_production::{configuration::get_configuration, run};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read_configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let db_connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&db_connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind(&address)?;
    run(listener, connection_pool)?.await
}
