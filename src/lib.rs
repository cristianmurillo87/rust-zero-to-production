pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use routes::{check_health, subscribe_to_newsletter};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(check_health))
            .route("/subscriptions", web::post().to(subscribe_to_newsletter))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
