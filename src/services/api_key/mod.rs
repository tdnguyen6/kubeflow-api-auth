use actix_web::{post, web, HttpResponse, Responder, Result};
use serde::Deserialize;
use sqlx::{PgPool};
use uuid::Uuid;

use crate::{config::Config, utils};

#[derive(Deserialize)]
struct Params {
    email: String,
    #[serde(rename = "recaptchaToken")]
    recaptcha_token: String,
}

#[post("/api/key")]
async fn view_content(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    params: web::Query<Params>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    if !utils::verify_recaptcha(&config, &params.recaptcha_token).await? {
        return Ok(HttpResponse::Unauthorized().body("wrong recaptcha"));
    }
    let rec = sqlx::query!("SELECT content FROM api_key WHERE email=$1", &params.email)
        .fetch_one(&**pool)
        .await;

    match rec {
        Ok(row) => Ok(HttpResponse::Ok().body(row.content.expect("Content unwrap error"))),
        Err(_e) => Ok(HttpResponse::Created().body(add_rec(&params.email, pool).await?)),
    }
}

#[post("/api/key/roll")]
async fn roll(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    params: web::Query<Params>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    if !utils::verify_recaptcha(&config, &params.recaptcha_token).await? {
        return Ok(HttpResponse::Unauthorized().body("wrong recaptcha"));
    }

    let res = sqlx::query!(
        "UPDATE api_key SET content = $1 WHERE email = $2 RETURNING content",
        &new_key(),
        &params.email
    )
    .fetch_one(&**pool)
    .await?;

    Ok(HttpResponse::Accepted().body(res.content.unwrap()))
}

async fn add_rec(email: &String, pool: web::Data<PgPool>) -> anyhow::Result<String> {
    let res = sqlx::query!(
        "INSERT INTO api_key VALUES ($1, $2) RETURNING content",
        &email,
        &new_key(),
    )
    .fetch_one(&**pool)
    .await?;

    Ok(res.content.unwrap())
}

#[inline]
fn new_key() -> String {
    Uuid::new_v4().to_simple().to_string()
}
