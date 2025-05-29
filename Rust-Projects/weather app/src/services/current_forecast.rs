use std::env;
use dotenv::dotenv;
use crate::error::WeatherResult;
use crate::weather::{NowWeather, HourlyForecast, CurrentWeatherData};
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

pub async fn get_current_weather_data(location: &str) -> WeatherResult<CurrentWeatherData> {
    let now_weather = get_weather(location).await?;
    let hourly_weather = get_weather_hourly(location).await?;
    Ok(CurrentWeatherData {
        current: now_weather,
        hourly: hourly_weather,
        time: Utc::now().timestamp().to_string(),
    })
}