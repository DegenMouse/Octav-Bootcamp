use rocket::serde::json::{Json};
use rocket::get;

use crate::services;
use crate::weather::ForecastResponse;
use crate::error::JsonError;

#[get("/weekly/<location>")]
pub async fn get_weekly_weather(location: &str) -> Json<Result<ForecastResponse, JsonError>> {
    let result = services::weekly_forecast::get_weather_forecast(location).await;

    Json(result.map_err(|e| JsonError {
        message: e.to_string(),
    }))
}