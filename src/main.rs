mod config;
mod models;
mod rules;
mod services;
mod utils;

use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[get("/health")]
async fn health() -> impl Responder {
    String::from("good")
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = config::Config::from_env()?;
    let config2 = config.clone();

    let pool = PgPoolOptions::new()
        .max_connections(config.database_maxcon)
        .connect(config.database_url.as_str())
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config2.clone()))
            .service(health)
            .service(services::api_key::view_content)
            .service(services::api_key::roll)
            .service(services::api_token::view_content)
            .service(services::api_token::delete)
            .service(services::api_token::list)
            .service(services::api_token::create)
            .service(services::check::check)
            .service(services::sync::reconcile)
            .service(services::frontend)
            .service(Files::new("/", "frontend/dist").index_file("index.html"))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await?)
}
