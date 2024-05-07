use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod data;
mod data_json;
mod db;
mod index;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::get_db_pool().await;
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("0.0.0.0:{}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                // Allow all origins
                origin.as_bytes().is_empty() || true
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .route("/", web::get().to(index::index))
            .route("/stations", web::get().to(data::get_stations))
            .route("/schedules", web::get().to(data::get_schedules))
            .route("/api/stations", web::get().to(data_json::get_stations_json))
            .route(
                "/api/schedules",
                web::get().to(data_json::get_schedules_json),
            )
    })
    .bind(&server_address)?
    .run()
    .await
}
