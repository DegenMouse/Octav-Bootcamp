use dotenv::dotenv;
use std::env;
use chrono::Utc;
use crate::weather::{ForecastResponse};
use crate::error::WeatherResult;

pub async fn get_weather_forecast(location: &str) -> WeatherResult<ForecastResponse> {
    dotenv().ok();

    let api_key = env::var("WEATHER_API_KEY")?;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=7", api_key, location))
        .send()
        .await?;

    if response.status() == reqwest::StatusCode::BAD_REQUEST {
        return Err(anyhow::anyhow!("Unknown location: {}", location));
    }

    let mut response: ForecastResponse = response.json().await?;
    response.time = Utc::now().timestamp().to_string();
    Ok(response)
}