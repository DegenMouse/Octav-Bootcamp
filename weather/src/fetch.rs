use std::env;
use dotenv::dotenv;
use reqwest;
use crate::error::WeatherResult;
use anyhow;
use crate::weather::{NowWeather, ForecastResponse, HourlyForecast};
use chrono::Utc;
pub async fn get_weather(location: &str) -> WeatherResult<NowWeather> {
    dotenv().ok();

    let api_key = env::var("WEATHER_API_KEY")?;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.weatherapi.com/v1/current.json?key={}&q={}", api_key, location))
        .send()
        .await?;

    if response.status() == reqwest::StatusCode::BAD_REQUEST {
        return Err(anyhow::anyhow!("Unknown location: {}", location));
    }

    let response = response.json().await?;
    Ok(response)
}

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

pub async fn get_weather_hourly(location: &str) -> WeatherResult<HourlyForecast> {
    dotenv().ok();

    let api_key = env::var("WEATHER_API_KEY")?;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=1", api_key, location))
        .send()
        .await?;

    if response.status() == reqwest::StatusCode::BAD_REQUEST {
        return Err(anyhow::anyhow!("Unknown location: {}", location));
    }

    let response = response.json().await?;
    Ok(response)
}






