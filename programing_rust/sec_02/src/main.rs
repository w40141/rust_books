pub mod gcd;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use gcd::gcd;
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(get_gcd).service(post_gcd))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello World!")
}

#[get("/gcd")]
async fn get_gcd() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd/answer" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
            </form>
        "#,
    )
}

#[derive(Debug, Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[post("/gcd/answer")]
async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    let response = format!(
        "The greatest common divisor of the number {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );
    HttpResponse::Ok().content_type("text/html").body(response)
}
