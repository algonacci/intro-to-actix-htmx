use crate::models::{Schedule, Station};
use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "stations_data.html")]
struct StationsDataTemplate {
    stations: Vec<Station>,
}

pub async fn get_stations(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Station>(
        "SELECT id, name, daop, fg_enable, have_schedule, updated_at
        FROM station",
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(stations) => {
            let html = StationsDataTemplate { stations };
            HttpResponse::Ok()
                .content_type("text/html")
                .body(html.render().unwrap())
        }
        Err(e) => {
            eprintln!("Failed to fetch stations: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Template)]
#[template(path = "schedule_data.html")]
struct SchedulesDataTemplate {
    schedules: Vec<Schedule>,
}

pub async fn get_schedules(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Schedule>(
        "SELECT id, station_id, train_id, line, route, color, destination,
        updated_at FROM schedule LIMIT 10",
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(schedules) => {
            let html = SchedulesDataTemplate { schedules };
            HttpResponse::Ok()
                .content_type("text/html")
                .body(html.render().unwrap())
        }
        Err(e) => {
            eprintln!("Failed to fetch schedules: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
