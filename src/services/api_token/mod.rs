use std::collections::HashMap;

use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    config::Config,
    models::api_token::{APIToken, TokenCore, TokenDTO, TokenData},
    utils,
};

#[derive(Deserialize)]
struct Params {
    email: String,
    #[serde(rename = "recaptchaToken")]
    recaptcha_token: String,
}


#[derive(Debug, Deserialize)]
pub struct RecaptchaOnlyParam {
    #[serde(rename = "recaptchaToken")]
    recaptcha_token: String,
}

#[post("/api/token/{id}")]
async fn view_content(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    id: web::Path<i32>,
    params: web::Query<RecaptchaOnlyParam>,
) -> actix_web::Result<impl Responder, Box<dyn std::error::Error>> {
    if !utils::verify_recaptcha(&config, &params.recaptcha_token).await? {
        return Ok(HttpResponse::Unauthorized().body("wrong recaptcha"));
    }
    let rec = sqlx::query!(
        "SELECT content FROM api_token WHERE id=$1",
        &id.into_inner()
    )
    .fetch_one(&**pool)
    .await;

    match rec {
        Ok(row) => Ok(HttpResponse::Ok().body(row.content.expect("Content unwrap error"))),
        Err(_e) => Ok(HttpResponse::NotFound().body("id not found")),
    }
}

#[delete("/api/token/{id}")]
async fn delete(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    id: web::Path<i32>,
    params: web::Query<RecaptchaOnlyParam>,
) -> actix_web::Result<impl Responder, Box<dyn std::error::Error>> {
    if !utils::verify_recaptcha(&config, &params.recaptcha_token).await? {
        return Ok(HttpResponse::Unauthorized().body("wrong recaptcha"));
    }
    let res = sqlx::query!("DELETE FROM api_token WHERE id=$1", &id.into_inner())
        .execute(&**pool)
        .await;

    if let Ok(r) = res {
        if r.rows_affected() == 1 {
            return Ok(HttpResponse::Ok().finish());
        }
        return Ok(HttpResponse::InternalServerError().finish());
    }

    Ok(HttpResponse::InternalServerError().finish())
}

#[post("/api/token")]
async fn create(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    params: web::Query<Params>,
    token_core: web::Json<TokenCore>,
) -> actix_web::Result<impl Responder, Box<dyn std::error::Error>> {
    if !utils::verify_recaptcha(&config, &params.recaptcha_token).await? {
        return Ok(HttpResponse::Unauthorized().body("wrong recaptcha"));
    }

    let rec = sqlx::query!(
        "INSERT INTO api_token (email) VALUES ($1) RETURNING id",
        &params.email,
    )
    .fetch_one(&**pool)
    .await?;

    let token_data = TokenData {
        id: rec.id,
        core: token_core.into_inner(),
    };

    let encoded_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &token_data,
        &jsonwebtoken::EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .expect("jwt encode error");

    let res = sqlx::query!(
        "UPDATE api_token SET content = $1 WHERE id = $2",
        &encoded_token,
        &rec.id
    )
    .execute(&**pool)
    .await;

    if let Ok(r) = res {
        if r.rows_affected() == 1 {
            return Ok(HttpResponse::Ok().body(format!("{}", rec.id)));
        }
        return Ok(HttpResponse::InternalServerError().finish());
    }
    Ok(HttpResponse::InternalServerError().finish())
}

#[derive(Debug, Deserialize)]
pub struct EmailOnlyParam {
    email: String,
}

#[get("/api/token/list")]
async fn list(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    params: web::Query<EmailOnlyParam>,
) -> actix_web::Result<impl Responder, Box<dyn std::error::Error>> {
    let records = sqlx::query!("SELECT * FROM api_token WHERE email = $1", &params.email)
        .fetch_all(&**pool)
        .await?;

    let dto = TokenDTO {
        tokens: records
            .iter()
            .map(|r| APIToken {
                last_used: r.last_used.format("%Y-%m-%d").to_string(),
                data: match &r.content.as_ref() {
                    Some(c) => {
                        jsonwebtoken::decode::<TokenData>(
                            c,
                            &jsonwebtoken::DecodingKey::from_secret(config.jwt_secret.as_ref()),
                            &jsonwebtoken::Validation {
                                validate_exp: false,
                                ..jsonwebtoken::Validation::default()
                            },
                        )
                        .expect("jwt decode error")
                        .claims
                    }
                    None => TokenData::default(),
                },
            })
            .collect(),
        template: TokenCore::default()
        // .better_default(&config).await.expect("template error"),
    };

    Ok(HttpResponse::Ok().json(dto))
}
