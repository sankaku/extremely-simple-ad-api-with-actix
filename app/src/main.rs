use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    errors: Vec<String>,
    ads: Vec<Ad>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ad {
    id: Uuid,
}

#[get("/deliver")]
async fn deliver() -> impl Responder {
    let ads = create_ads(3);
    let res = ApiResponse {
        success: true,
        errors: vec!["".to_string()],
        ads: ads,
    };
    HttpResponse::Ok().json(res)
}

fn create_ad() -> Ad {
    Ad { id: Uuid::new_v4() }
}

fn create_ads(num: usize) -> Vec<Ad> {
    (0..num).map(|_| create_ad()).collect()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(deliver))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
