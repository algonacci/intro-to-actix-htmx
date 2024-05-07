use crate::models::{Schedule, Station};
use actix_web::{web, Responder};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct ApiResponse<T> {
    status: Status,
    data: Option<T>,
}

#[derive(Serialize)]
struct Status {
    code: u16,
    message: String,
}

pub async fn get_stations_json(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Station>(
        "SELECT id, name, daop, fg_enable, have_schedule, updated_at
        FROM station",
    )
    .fetch_all(pool.get_ref())
    .await;

    let data = match result {
        Ok(stations) => Some(stations),
        Err(e) => {
            eprintln!("Failed to fetch stations: {}", e);
            None
        }
    };

    let response = ApiResponse {
        status: Status {
            code: 200,
            message: "Success get all stations".to_string(),
        },
        data,
    };
    web::Json(response)
}

pub async fn get_schedules_json(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Schedule>(
        "SELECT id, station_id, train_id, line, route, color, destination,
        updated_at FROM schedule",
    )
    .fetch_all(pool.get_ref())
    .await;

    let data = match result {
        Ok(schedules) => Some(schedules),
        Err(e) => {
            eprintln!("Failed to fetch schedules: {}", e);
            None
        }
    };

    let response = ApiResponse {
        status: Status {
            code: 200,
            message: "Success get all stations".to_string(),
        },
        data,
    };
    web::Json(response)
}
