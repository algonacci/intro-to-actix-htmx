use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod index;
mod db;
mod data;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::get_db_pool().await;
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index::index))
            .route("/stations", web::get().to(data::get_stations))
            .route("/schedules", web::get().to(data::get_schedules))
    })
    .bind(&server_address)?
    .run()
    .await
}
