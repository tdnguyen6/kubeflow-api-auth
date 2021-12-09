use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
use sqlx::{Executor, PgPool};
use uuid::Uuid;

use crate::{config::Config, utils};

#[derive(Deserialize)]

struct Params {
    email: String,
    recaptchaToken: String,
}

#[post("/api/key")]
async fn get_key(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    params: web::Query<Params>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    if !utils::verify_recaptcha(&config, &params.recaptchaToken).await? {
        return Ok(HttpResponse::Unauthorized().body("wrong recaptcha"));
    }
    let rec = sqlx::query!("SELECT content FROM api_key WHERE email=$1", &params.email)
        .fetch_one(&**pool)
        .await;

    match rec {
        Ok(row) => Ok(HttpResponse::Ok().body(row.content.unwrap())),
        Err(_e) => Ok(HttpResponse::Accepted().body(create_key())),
    }
}

#[post("/api/key/roll")]
async fn roll_key(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<Params>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let rec = sqlx::query!("SELECT FROM api_key WHERE email=$1", &params.email)
        .fetch_one(&**pool)
        .await?;
    Ok("hello")
}

#[inline]
fn create_key() -> String {
    return Uuid::new_v4().to_simple().to_string();
}
