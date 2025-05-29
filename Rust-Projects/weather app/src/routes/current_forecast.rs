use rocket::serde::json::{Json};
use rocket::get;

use crate::services;
use crate::weather::CurrentWeatherData;
use crate::error::JsonError;

#[get("/current/<location>")]
pub async fn get_current_weather(location: &str) -> Json<Result<CurrentWeatherData, JsonError>> {
    let result = services::current_forecast::get_current_weather_data(location).await;

    Json(result.map_err(|e| JsonError {
        message: e.to_string(),
    }))
}

#[get("/")]
pub fn basic() -> String {
    format!("Hops")
}