use actix_web::{web, App, HttpResponse, HttpServer};

async fn check_health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(check_health)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_checking_succeeds() {
        let response = check_health().await;
        assert!(response.status().is_success())
    }
}
