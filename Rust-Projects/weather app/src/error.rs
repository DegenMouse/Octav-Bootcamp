use anyhow::Result;
use serde::Serialize;

pub type WeatherResult<T> = Result<T, anyhow::Error>;

#[derive(Debug, Serialize)]
pub struct JsonError {
    pub message: String,
}