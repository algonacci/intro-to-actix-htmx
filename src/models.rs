// use chrono::NaiveTime;
use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub daop: i32,
    pub fg_enable: i32,
    pub have_schedule: Option<bool>,
    pub updated_at: String,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Schedule {
    pub id: String,
    pub station_id: String,
    pub train_id: String,
    pub line: String,
    pub route: String,
    pub color: String,
    pub destination: String,
    // pub time_estimated: NaiveTime,
    // pub destination_time: NaiveTime,
    pub updated_at: String,
}
