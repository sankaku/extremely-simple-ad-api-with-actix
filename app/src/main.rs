use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: How to merge DeliveryResponse and CvResponse ?
#[derive(Debug, Serialize, Deserialize)]
struct DeliveryResponse {
    success: bool,
    errors: Vec<String>,
    message: DeliveryMessage,
}

#[derive(Debug, Serialize, Deserialize)]
struct CvResponse {
    success: bool,
    errors: Vec<String>,
    message: CvMessage,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeliveryMessage {
    ads: Vec<Ad>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CvMessage {
    id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ad {
    id: Uuid,
}

#[derive(Deserialize)]
struct DeliveryParams {
    num: Option<u64>,
}

#[derive(Deserialize)]
struct CvParams {
    id: Uuid,
}

#[get("/deliver")]
async fn deliver(params: web::Query<DeliveryParams>) -> impl Responder {
    let num = params.num.unwrap_or(5);
    let ads = create_ads(num);
    let res = DeliveryResponse {
        success: true,
        errors: vec!["".to_string()],
        message: DeliveryMessage { ads: ads },
    };
    HttpResponse::Ok().json(res)
}

#[post("cv")]
async fn cv(params: web::Query<CvParams>) -> impl Responder {
    let id = params.id;
    let res = CvResponse {
        success: true,
        errors: vec!["".to_string()],
        message: CvMessage { id: id },
    };
    HttpResponse::Ok().json(res)
}

fn create_ad() -> Ad {
    Ad { id: Uuid::new_v4() }
}

fn create_ads(num: u64) -> Vec<Ad> {
    (0..num).map(|_| create_ad()).collect()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(deliver).service(cv))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
