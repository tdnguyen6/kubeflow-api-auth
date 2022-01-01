use actix_web::{post, web, Responder};

use crate::{
    config::Config, utils,
};

#[post("/api/reconcile")]
async fn reconcile(
    pool: web::Data<sqlx::PgPool>,
    config: web::Data<Config>,
) -> actix_web::Result<impl Responder, Box<dyn std::error::Error>> {
    let profiles = utils::all_kf_users(&config)?;
    let res = sqlx::query!(
        "DELETE FROM api_token WHERE email != ALL($1)",
        &profiles[..]
    )
    .execute(&**pool)
    .await?;

    Ok(format!("{}", res.rows_affected()))
}
