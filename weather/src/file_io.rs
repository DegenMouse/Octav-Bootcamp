use crate::weather::CurrentWeatherData;
use crate::weather::ForecastResponse;
use crate::error::WeatherResult;

use std::fs::File;
use std::io::{self, Write, BufWriter, BufReader, Read};
use hex;
use chrono::Utc;

pub fn save_weather_data_current(location: &str, data: CurrentWeatherData) -> WeatherResult<()> {
    let lowcase_location = location.to_lowercase();
    let json_data = serde_json::to_string(&data)?;
    let hex_data = hex::encode(json_data.as_bytes());
    let file_name = format!("weather_data/{}_now.hex", lowcase_location);
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(hex_data.as_bytes())?;
    Ok(())
}

pub fn save_weather_data_forecast(location: &str, data: ForecastResponse) -> WeatherResult<()> {
    let lowcase_location = location.to_lowercase();
    let json_data = serde_json::to_string(&data)?;
    let hex_data = hex::encode(json_data.as_bytes());
    let file_name = format!("weather_data/{}_forecast.hex", lowcase_location);
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(hex_data.as_bytes())?;
    Ok(())
}

pub fn search_weather_offline_actual(location: &str) -> WeatherResult<CurrentWeatherData> {
    let lowcase_location = location.to_lowercase();
    let file_name = format!("weather_data/{}_now.hex", lowcase_location);
    let file = File::open(&file_name).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            anyhow::anyhow!("Location '{}' does not have any offline data", location)
        } else {
            anyhow::anyhow!("Error reading weather data: {}", err)
        }
    })?;
    let mut reader = BufReader::new(file);
    let mut hex_data = Vec::new();
    reader.read_to_end(&mut hex_data)?;
    let json_data = hex::decode(hex_data)?;
    let data: CurrentWeatherData = serde_json::from_slice(&json_data)?;
    Ok(data)
}

pub fn search_weather_offline_forecast(location: &str) -> WeatherResult<ForecastResponse> {
    let lowcase_location = location.to_lowercase();
    let file_name = format!("weather_data/{}_forecast.hex", lowcase_location);
    let file = File::open(&file_name).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            anyhow::anyhow!("Location '{}' does not have any offline data", location)
        } else {
            anyhow::anyhow!("Error reading weather data: {}", err)
        }
    })?;
    let mut reader = BufReader::new(file);
    let mut hex_data = Vec::new();
    reader.read_to_end(&mut hex_data)?;
    let json_data = hex::decode(hex_data)?;
    let data: ForecastResponse = serde_json::from_slice(&json_data)?;
    Ok(data)
}

pub fn check_time_for_file_now(location: &str) -> WeatherResult<bool> {
    let lowcase_location = location.to_lowercase();
    match search_weather_offline_actual(&lowcase_location) {
        Ok(data) => {
            let current_time = Utc::now().timestamp();
            match data.time.parse::<i64>() {
                Ok(file_time) => Ok(current_time - file_time < 1800),
                Err(_) => Ok(false)
            }
        }
        Err(_) => Ok(false)
    }
}

pub fn check_time_for_file_forecast(location: &str) -> WeatherResult<bool> {
    let lowcase_location = location.to_lowercase();
    match search_weather_offline_forecast(&lowcase_location) {
        Ok(data) => {
            let current_time = Utc::now().timestamp();
            match data.time.parse::<i64>() {
                Ok(file_time) => Ok(current_time - file_time < 1800),
                Err(_) => Ok(false)
            }
        }
        Err(_) => Ok(false)
    }
}

pub fn export_weather_data_now(location: &str, data: CurrentWeatherData) -> WeatherResult<()> {
    let lowcase_location = location.to_lowercase();
    let file_name = format!("weather_exports/{}_now.json", lowcase_location);
    let file = File::create(file_name)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &data)?;
    Ok(())
}

pub fn export_weather_data_forecast(location: &str, data: ForecastResponse) -> WeatherResult<()> {
    let lowcase_location = location.to_lowercase();
    let file_name = format!("weather_exports/{}_forecast.json", lowcase_location);
    let file = File::create(file_name)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &data)?;
    Ok(())
}