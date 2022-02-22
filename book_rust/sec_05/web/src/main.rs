use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use anyhow::Result;
use serde::{Deserialize, Serialize};

const SERVER_ADDR: &str = "127.0.0.1:8888";

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("[SERVER] http://{SERVER_ADDR}/");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/bmi", web::get().to(bmi))
            .route("/calc", web::get().to(calc))
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}

async fn index(req: HttpRequest) -> &'static str {
    println!("request: {req:?}");
    "Hello World"
}

async fn bmi(_: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "{}{}{}{}{}{}",
            "<html><body><h1>BMI判定</h1>",
            "<form action='calc'>",
            "Height: <input name='height' value='160'><br>",
            "weight: <input name='weight' value='60'><br>",
            "<input type='submit' value='Submit'>",
            "</form></body></html>"
        )))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BMI {
    height: f64,
    weight: f64,
}

async fn calc(q: web::Query<BMI>) -> Result<HttpResponse, Error> {
    println!("{q:?}");
    let h = q.height / 100.0;
    let bmi = q.weight / (h * h);
    let per = (bmi / 22.0) * 100.0;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<h3>BMI={bmi:.1}, 乖離率={per:.0}%</h3>")))
}
