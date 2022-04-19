use std::fmt::Debug;

use actix_web::{route, http::header::ContentType, web, HttpRequest, HttpResponse, Responder};
use askama::Template;

use crate::{config::Config, models::api_token::TokenData, rules};

#[derive(Template)]
#[template(path = "401.html", escape = "none")]
struct Error401Template<'a> {
    msg: &'a str,
    location: &'a str,
}

#[route(
    "/check/{upstream_url:.*}",
    method = "GET",
    method = "POST",
    method = "PUT",
    method = "PATCH",
    method = "HEAD",
    method = "DELETE",
    method = "OPTIONS",
    method = "TRACE",
    method = "CONNECT",
)]
async fn check(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    config: web::Data<Config>,
) -> actix_web::Result<impl Responder, Box<dyn std::error::Error>> {
    let hdrs = req.headers();
    let kf_endpoint = config.kubeflow.interactive_endpoint.clone();
    let auth_res = match hdrs.get("Authorization") {
        Some(_h) => token_auth(req, pool, config.clone()).await?,
        None => key_auth(req, pool).await?,
    };

    let userid_header = config.kubeflow.userid_header.as_str();

    if auth_res.success {
        Ok(HttpResponse::Ok()
            .append_header((userid_header, auth_res.email))
            .body(auth_res.msg))
    } else {
        Ok(HttpResponse::Unauthorized()
            .append_header(ContentType::html())
            .body(
                Error401Template {
                    msg: &auth_res.msg,
                    location: &kf_endpoint,
                }
                .render()
                .unwrap(),
            ))
    }
}

struct AuthRes {
    msg: String,
    success: bool,
    email: String,
}

async fn key_auth(req: HttpRequest, pool: web::Data<sqlx::PgPool>) -> anyhow::Result<AuthRes> {
    let hdrs = req.headers();
    let email;
    let key;

    if let Some(h) = hdrs.get("X-Auth-Key") {
        key = h.to_str()?;
    } else {
        return Ok(AuthRes {
            success: false,
            msg: String::from("no auth key"),
            email: String::default(),
        });
    };

    if let Some(h) = hdrs.get("X-Auth-Email") {
        email = h.to_str()?;
    } else {
        return Ok(AuthRes {
            success: false,
            msg: String::from("no auth email"),
            email: String::default(),
        });
    };

    let rec = sqlx::query!(
        "SELECT * FROM api_key WHERE email = $1 AND content = $2",
        &email,
        &key
    )
    .fetch_one(&**pool)
    .await;

    match rec {
        Ok(_r) => Ok(AuthRes {
            success: true,
            msg: String::default(),
            email: String::from(email),
        }),
        Err(_) => Ok(AuthRes {
            success: false,
            msg: String::from("Wrong credentials"),
            email: String::default(),
        }),
    }
}

async fn token_auth(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    config: web::Data<Config>,
) -> anyhow::Result<AuthRes> {
    let auth_header = req.headers().get("Authorization").unwrap().to_str()?;
    let token_opt = auth_header.split("Bearer ").last();
    if let Some(token) = token_opt {
        let token_data_res = jsonwebtoken::decode::<TokenData>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(config.jwt_secret.as_ref()),
            &jsonwebtoken::Validation {
                validate_exp: false,
                ..jsonwebtoken::Validation::default()
            },
        );

        if let Ok(token_data) = token_data_res {
            let today = chrono::Utc::now().date().naive_utc();

            if chrono::NaiveDate::parse_from_str(
                token_data.claims.core.end_date.as_str(),
                "%Y-%m-%d",
            )? < today || chrono::NaiveDate::parse_from_str(
                token_data.claims.core.start_date.as_str(),
                "%Y-%m-%d",
            )? > today
            {
                return Ok(AuthRes {
                    success: false,
                    msg: String::from("token expired"),
                    email: String::default(),
                });
            }

            sqlx::query!(
                "UPDATE api_token SET last_used = $1 WHERE id = $2",
                &today,
                &token_data.claims.id,
            )
            .execute(&**pool)
            .await?;

            let res = sqlx::query!(
                "SELECT email FROM api_token WHERE id = $1",
                &token_data.claims.id,
            )
            .fetch_one(&**pool)
            .await?;

            let upstream_url = req.match_info().query("upstream_url");

            if rules::check(upstream_url, &token_data.claims.core.rules).await? {
                return Ok(AuthRes {
                    success: true,
                    msg: String::default(),
                    email: String::from(res.email.unwrap()),
                });
            }
        }
    }

    Ok(AuthRes {
        success: false,
        msg: String::from("error verifying token"),
        email: String::default(),
    })
}
