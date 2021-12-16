use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use askama::Template;

use crate::config::Config;

pub mod api_key;
pub mod api_token;
pub mod check;
pub mod sync;

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
struct FrontendIndexTmpl<'a> {
    userid: &'a str,
}

#[get("/")]
pub async fn frontend(
    req: HttpRequest,
    _pool: web::Data<sqlx::PgPool>,
    config: web::Data<Config>,
) -> actix_web::Result<impl Responder, Box<dyn std::error::Error>> {
    let hdrs = req.headers();
    match hdrs.get(&config.kubeflow.userid_header) {
        Some(h) => Ok(HttpResponse::Ok().body(
            FrontendIndexTmpl {
                userid: h.to_str().unwrap(),
            }
            .render()
            .unwrap(),
        )),
        None => Ok(HttpResponse::Forbidden().finish()),
    }
}
