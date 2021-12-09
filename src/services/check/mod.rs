use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::config::Config;

#[get("/check/{_:.*}")]
async fn check(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    config: web::Data<Config>,
) -> actix_web::Result<impl Responder, Box<dyn std::error::Error>> {
    let hdrs = req.headers();
    let kf_endpoint = config.kubeflow.interactive_endpoint.clone();
    let auth_res = match hdrs.get("Authorization") {
        Some(_h) => token_auth(req, pool, config).await?,
        None => key_auth(req, pool).await?,
    };

    if auth_res.success {
        Ok(HttpResponse::Ok().body(auth_res.msg))
    } else {
        Ok(HttpResponse::SeeOther()
            .append_header(("Location", kf_endpoint))
            .body(auth_res.msg))
    }
}

struct AuthRes {
    msg: String,
    success: bool,
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
        });
    };

    if let Some(h) = hdrs.get("X-Auth-Email") {
        email = h.to_str()?;
    } else {
        return Ok(AuthRes {
            success: false,
            msg: String::from("no auth email"),
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
        }),
        Err(e) => Ok(AuthRes {
            success: false,
            msg: e.to_string(),
        }),
    }
}

async fn token_auth(
    req: HttpRequest,
    pool: web::Data<sqlx::PgPool>,
    config: web::Data<Config>,
) -> anyhow::Result<AuthRes> {
    Ok(AuthRes {
        success: true,
        msg: String::default(),
    })
}
