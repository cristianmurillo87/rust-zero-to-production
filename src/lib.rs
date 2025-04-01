use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn check_health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// web::Form<T> extract urls encoded form data
async fn subcribe_to_newsletter(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(check_health))
            .route("/subscriptions", web::post().to(subcribe_to_newsletter))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
