use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_check_test() {
        let resp = health_check().await;
        assert!(resp.status().is_success())
    }
}
