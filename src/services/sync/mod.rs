use actix_web::{post, web, Responder};

use crate::{
    config::Config, utils,
};

#[post("/api/reconcile")]
async fn reconcile(
    pool: web::Data<sqlx::PgPool>,
    _config: web::Data<Config>,
) -> actix_web::Result<impl Responder, Box<dyn std::error::Error>> {
    // let profiles = utils::get_resource(config, Resource::Profile).await?;
    let profiles = utils::all_kf_users()?;
    let res = sqlx::query!(
        "DELETE FROM api_token WHERE email != ALL($1)",
        &profiles[..]
    )
    .execute(&**pool)
    .await?;

    Ok(format!("{}", res.rows_affected()))
}
