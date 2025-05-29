use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NowWeather {
    pub current: Current,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Current {
    pub temp_c: f64,
    pub temp_f: f64,
    pub wind_kph: f64,
    pub wind_mph: f64,
    pub wind_dir: String,
    pub precip_mm: f64,
    pub precip_in: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentWeatherData {
    pub current: NowWeather,
    pub hourly: HourlyForecast,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastResponse {
    pub forecast: Forecast,
    #[serde(default)]
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Forecast {
    pub forecastday: Vec<DailyForecast>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyForecast {
    pub date: String,
    pub day: DayAverage,
    pub hour: Vec<Hours>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DayAverage {
    pub avgtemp_c: f64,
    pub avgtemp_f: f64,
    pub maxwind_kph: f64,
    pub maxwind_mph: f64,
    pub condition: Condition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Condition {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HourlyForecast {
    pub forecast: Forecast,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hours {
    pub time: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub wind_kph: f64,
    pub wind_mph: f64,
    pub condition: Condition,
}