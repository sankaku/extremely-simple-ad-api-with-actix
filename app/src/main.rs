use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bb8::Pool;
use bb8_redis::{redis::AsyncCommands, RedisConnectionManager};
use redis::RedisError;

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
async fn deliver(
    params: web::Query<DeliveryParams>,
    pool: web::Data<Pool<RedisConnectionManager>>,
) -> impl Responder {
    let num = params.num.unwrap_or(5);

    let ads = create_ads(num);
    record_delivered(&ads, &pool).await;

    let res = DeliveryResponse {
        success: true,
        errors: vec!["".to_string()],
        message: DeliveryMessage { ads: ads },
    };

    HttpResponse::Ok().json(res)
}

async fn record_delivered(ads: &Vec<Ad>, pool: &Pool<RedisConnectionManager>) -> () {
    let ids: Vec<Uuid> = ads.iter().map(|ad| ad.id).collect();
    let mut conn = pool.get().await.unwrap();
    for id in ids {
        let key = format!("id:{}", id);
        let _: () = conn.hset_multiple(key, &[("cv", "false")]).await.unwrap();
    }
}

async fn record_cv(id: Uuid, pool: &Pool<RedisConnectionManager>) -> () {
    let mut conn = pool.get().await.unwrap();
    let key = format!("id:{}", id);
    let _: () = conn.hset_multiple(key, &[("cv", "true")]).await.unwrap();
}

#[post("cv")]
async fn cv(
    params: web::Query<CvParams>,
    pool: web::Data<Pool<RedisConnectionManager>>,
) -> impl Responder {
    let id = params.id;
    let res = CvResponse {
        success: true,
        errors: vec!["".to_string()],
        message: CvMessage { id: id },
    };
    record_cv(id, &pool).await;
    HttpResponse::Ok().json(res)
}

fn create_ad() -> Ad {
    Ad { id: Uuid::new_v4() }
}

fn create_ads(num: u64) -> Vec<Ad> {
    (0..num).map(|_| create_ad()).collect()
}

async fn create_pool() -> Result<Pool<RedisConnectionManager>, RedisError> {
    let manager = RedisConnectionManager::new("redis://127.0.0.1/")?;
    let pool = Pool::builder().max_size(20).build(manager).await.unwrap();
    Ok(pool)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool().await.expect("Failed to create pool");

    HttpServer::new(move || App::new().data(pool.clone()).service(deliver).service(cv))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
