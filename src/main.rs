mod config;
mod models;
mod services;
mod utils;

use actix_files::Files;
use actix_web::{get, App, HttpServer, Responder, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[get("/api")]
async fn index() -> actix_web::Result<impl Responder, Box<dyn std::error::Error>> {
    Ok("good")
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
            .service(index)
            .service(services::api_key::get_key)
            .service(Files::new("/", "frontend/dist").index_file("index.html"))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await?)
}
