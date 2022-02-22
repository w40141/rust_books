use actix_web::{web, App, HttpRequest, HttpServer};
use anyhow::Result;

const SERVER_ADDR: &str = "127.0.0.1:8888";

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("[SERVER] http://{SERVER_ADDR}/");
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(SERVER_ADDR)?
        .run()
        .await
}

async fn index(req: HttpRequest) -> &'static str {
    println!("request: {req:?}");
    "Hello World"
}
