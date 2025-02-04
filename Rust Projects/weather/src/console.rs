use crate::args::{LocationSubcommand, WeatherArgs, WeatherCommand, OfflineSubcommand, ForecastOfflineCommand};
use crate::fetch;
use crate::file_io;
use crate::weather::CurrentWeatherData;

use clap::Parser;
use cli_table::{format::Justify, Cell, Style, Table};
use chrono::NaiveDate;
use chrono::Utc;
use colored::*;


pub async fn flow() {
    let args = WeatherArgs::parse();
    let unit = match args.unit {
        Some(u) => match u.as_str() {
            "imperial" => String::from("imperial"),
            _ => String::from("metric"),
        },
        None => String::from("metric"),
    };
    let export = args.export.unwrap_or(false);
    match args.command {
        WeatherCommand::Now(now_command) => match now_command.command {
            LocationSubcommand::City { city } => {
                display_current_weather(&city, &unit, now_command.compare.as_deref(), export).await;
            }
            LocationSubcommand::Ip { ip } => {
                display_current_weather(&ip, &unit, now_command.compare.as_deref(), export).await;
            }
            LocationSubcommand::LatLong { lat, long } => {
                display_current_weather(format!("{}&{}", lat, long).as_str(), &unit, now_command.compare.as_deref(), export).await;
            }
        },
        WeatherCommand::Forecast(forecast_command) => match forecast_command.command {
            LocationSubcommand::City { city } => {
                display_forecast_weather_interval(&city, &unit, forecast_command.compare.as_deref(), export).await;
            }
            LocationSubcommand::Ip { ip } => {
                display_forecast_weather_interval(&ip, &unit, forecast_command.compare.as_deref(), export).await;
            }
            LocationSubcommand::LatLong { lat, long } => {
                display_forecast_weather_interval(format!("{}&{}", lat, long).as_str(), &unit, forecast_command.compare.as_deref(), export).await;
            }
        },
        WeatherCommand::Offline(offline_command) => match offline_command.command {
            OfflineSubcommand::Latest(location_cmd) => match location_cmd {
                LocationSubcommand::City { city } => {
                    weather_offline_current(&city, &unit, export).await;
                }
                LocationSubcommand::Ip { ip } => {
                    weather_offline_current(&ip, &unit, export).await;
                }
                LocationSubcommand::LatLong { lat, long } => {
                    weather_offline_current(format!("{}&{}", lat, long).as_str(), &unit, export).await;
                }
            },
            OfflineSubcommand::Forecast(forecast_cmd) => match forecast_cmd {
                ForecastOfflineCommand::Location(location_cmd) => match location_cmd {
                    LocationSubcommand::City { city } => {
                        weather_offline_forecast(&city, &unit, export).await;
                    }
                    LocationSubcommand::Ip { ip } => {
                        weather_offline_forecast(&ip, &unit, export).await;
                    }
                    LocationSubcommand::LatLong { lat, long } => {
                        weather_offline_forecast(format!("{}&{}", lat, long).as_str(), &unit, export).await;
                    }
                },
            },
        },
    }

}

async fn display_current_weather(location: &str, unit: &str, compare: Option<&str>, export: bool) {
    match file_io::check_time_for_file_now(location) {
        Ok(true) => {
            weather_offline_current(location, unit, export).await;
            println!("{}", "Using cached data".green());
            
            // Handle comparison for cached data
            if let Some(compare_location) = compare {
                match file_io::check_time_for_file_now(compare_location) {
                    Ok(true) => {
                        weather_offline_current(compare_location, unit, export).await;
                        println!("{}", "Using cached data for comparison".green());
                    },
                    _ => {
                        match fetch::get_weather(compare_location).await {
                            Ok(compare_weather) => {
                                let compare_hourly = match fetch::get_weather_hourly(compare_location).await {
                                    Ok(weather) => weather,
                                    Err(e) => {
                                        println!("Error fetching hourly forecast for comparison: {:?}", e);
                                        return;
                                    }
                                };
                
                                let compare_data = CurrentWeatherData {
                                    current: compare_weather,
                                    hourly: compare_hourly,
                                    time: Utc::now().timestamp().to_string(),
                                };
                                display_weather_actual(compare_location, &compare_data, unit.to_string(), export);
                                
                                // Save comparison data
                                match file_io::save_weather_data_current(compare_location, compare_data) {
                                    Ok(_) => println!("{}", "Comparison weather data saved successfully.".green()),
                                    Err(e) => println!("Error saving comparison weather data: {:?}", e),
                                }
                            }
                            Err(e) => println!("Error fetching comparison weather: {:?}", e),
                        }
                    }
                }
            }
        }
        _ => {
            let current_weather = match fetch::get_weather(location).await {
                Ok(weather) => weather,
                Err(e) => {
                    println!("{}", format!("Error: {:?}", e).red());
                    return;
                }
            };
        
            let hourly_weather = match fetch::get_weather_hourly(location).await {
                Ok(weather) => weather,
                Err(e) => {
                    println!("{}", format!("Error fetching hourly forecast: {:?}", e).red());
                    return;
                }
            };
        
            let weather_data = CurrentWeatherData {
                current: current_weather,
                hourly: hourly_weather,
                time: Utc::now().timestamp().to_string(),
            };
            
            display_weather_actual(location, &weather_data, unit.to_string(), export);
        
            if let Some(compare_location) = compare {
                match fetch::get_weather(compare_location).await {
                    Ok(compare_weather) => {
                        let compare_hourly = match fetch::get_weather_hourly(compare_location).await {
                            Ok(weather) => weather,
                            Err(e) => {
                                println!("Error fetching hourly forecast for comparison: {:?}", e);
                                return;
                            }
                        };
        
                        let compare_data = CurrentWeatherData {
                            current: compare_weather,
                            hourly: compare_hourly,
                            time: Utc::now().timestamp().to_string(),
                        };
                        display_weather_actual(compare_location, &compare_data, unit.to_string(), export);
                        
                        // Save comparison data
                        match file_io::save_weather_data_current(compare_location, compare_data) {
                            Ok(_) => println!("{}", "Comparison weather data saved successfully.".green()),
                            Err(e) => println!("{}", format!("Error: {:?}", e).red()),
                        }
                    }
                    Err(e) => println!("{}", format!("Error: {:?}", e).red()),
                }
            }
        
            match file_io::save_weather_data_current(location, weather_data) {
                Ok(_) => println!("{}", "Weather data saved successfully.".green()),
                Err(e) => println!("{}", format!("Error: {:?}", e).red()),
            }
        }
    }
}

async fn display_forecast_weather_interval(location: &str, unit: &str, compare: Option<&str>, export: bool) {
    display_forecast_weather(location, unit, export).await;
    if let Some(compare_location) = compare {
        display_forecast_weather(compare_location, unit, export).await;
    }   
}

async fn display_forecast_weather(location: &str, unit: &str, export: bool) {
    match file_io::check_time_for_file_forecast(location) {
        Ok(true) => {
            weather_offline_forecast(location, unit, export).await;
            println!("{}", "Using cached data".green());
        }
        _ => {
            match fetch::get_weather_forecast(location).await {
                Ok(weather) => {
                    println!("\nForecast for {}:", location);
                    
                    let table = vec![
                        vec!["Temperature".cell().bold(true)]
                            .into_iter()
                            .chain(weather.forecast.forecastday.iter().map(|day| {
                                if unit == "imperial" {
                                    format!("{}°F", day.day.avgtemp_f).cell().justify(Justify::Center)
                                } else {
                                    format!("{}°C", day.day.avgtemp_c).cell().justify(Justify::Center)
                                }
                            }))
                            .collect::<Vec<_>>(),
                        vec!["Condition".cell().bold(true)]
                            .into_iter()
                            .chain(weather.forecast.forecastday.iter().map(|day| {
                                get_emoji_for_time(&day.day.condition.text, &day.date).cell().justify(Justify::Center)
                            }))
                            .collect::<Vec<_>>(),
                        vec!["Wind".cell().bold(true)]
                            .into_iter()
                            .chain(weather.forecast.forecastday.iter().map(|day| {
                                if unit == "imperial" {
                                    format!("{} mph", day.day.maxwind_mph).cell().justify(Justify::Center)
                                } else {
                                    format!("{} kph", day.day.maxwind_kph).cell().justify(Justify::Center)
                                }
                            }))
                            .collect::<Vec<_>>(),
                    ]
                    .table()
                    .title(
                        vec!["Day".cell().bold(true)]
                            .into_iter()
                            .chain(weather.forecast.forecastday.iter().map(|day| {
                                let date = NaiveDate::parse_from_str(&day.date, "%Y-%m-%d")
                                    .map(|d| d.format("%A").to_string())
                                    .unwrap_or(day.date.clone());
                                date.cell().bold(true)
                            }))
                            .collect::<Vec<_>>()
                    )
                    .bold(true);
        
                    let table_display = table.display().unwrap();
                    println!("{}", table_display);
                    match file_io::save_weather_data_forecast(location, weather) {
                        Ok(_) => println!("\n{}", "Weather data saved successfully.".purple()),
                        Err(e) => println!("{}", format!("Error: {:?}", e).red()),
                    }
                }
                Err(e) => println!("{}", format!("Error: {:?}", e).red()),
            }
        }
    }
}

    

async fn weather_offline_current(location: &str, unit: &str, export: bool) {
    match file_io::search_weather_offline_actual(location) {
        Ok(weather_data) => {
            display_weather_actual(location, &weather_data, unit.to_string(), export);
        },  
        Err(e) => println!("Error reading offline weather data: {:?}", e),
    }
}

async fn weather_offline_forecast(location: &str, unit: &str, export: bool) {
    match file_io::search_weather_offline_forecast(location) {
        Ok(weather_data) => {
            println!("\nOffline forecast for {}:", location);
            
            let table = vec![
                vec!["Temperature".cell().bold(true)]
                    .into_iter()
                    .chain(weather_data.forecast.forecastday.iter().map(|day| {
                        if unit == "imperial" {
                            format!("{}°F", day.day.avgtemp_f).cell().justify(Justify::Center)
                        } else {
                            format!("{}°C", day.day.avgtemp_c).cell().justify(Justify::Center)
                        }
                    }))
                    .collect::<Vec<_>>(),
                vec!["Condition".cell().bold(true)]
                    .into_iter()
                    .chain(weather_data.forecast.forecastday.iter().map(|day| {
                        get_emoji_for_time(&day.day.condition.text, &day.date).cell().justify(Justify::Center)
                    }))
                    .collect::<Vec<_>>(),
                vec!["Wind".cell().bold(true)]
                    .into_iter()
                    .chain(weather_data.forecast.forecastday.iter().map(|day| {
                        if unit == "imperial" {
                            format!("{} mph", day.day.maxwind_mph).cell().justify(Justify::Center)
                        } else {
                            format!("{} kph", day.day.maxwind_kph).cell().justify(Justify::Center)
                        }
                    }))
                    .collect::<Vec<_>>(),
            ]
            .table()
            .title(
                vec!["Day".cell().bold(true)]
                    .into_iter()
                    .chain(weather_data.forecast.forecastday.iter().map(|day| {
                        let date = NaiveDate::parse_from_str(&day.date, "%Y-%m-%d")
                            .map(|d| d.format("%A").to_string())
                            .unwrap_or(day.date.clone());
                        date.cell().bold(true)
                    }))
                    .collect::<Vec<_>>()
            )
            .bold(true);

            let table_display = table.display().unwrap();
            println!("{}", table_display);

            if export {
                match file_io::export_weather_data_forecast(location, weather_data) {
                    Ok(_) => println!("{}", "Weather data exported as json.".green()),
                    Err(e) => println!("{}", format!("Error: {:?}", e).red()),
                }
            }
        }
        Err(e) => println!("Error reading offline weather data: {:?}", e),
    }
}

fn get_emoji_for_time(condition: &str, time: &str) -> String {
    // For daily forecast, time will be just the date (YYYY-MM-DD)
    // For hourly forecast, time will include hours (YYYY-MM-DD HH:MM)
    let hour: u32 = if time.len() > 10 {
        time[11..13].parse().unwrap_or(12) // Use 12 as default for hourly
    } else {
        12 // Use 12 (noon) as default for daily forecasts
    };

    match condition.trim() {
        "Cloudy" => {
            if (7..=17).contains(&hour) {
                "🌥️".to_string() // Day
            } else {
                "☁️".to_string() // Night
            }
        }
        "Sunny" => "☀️".to_string(),
        "Clear" => {
            if (7..=17).contains(&hour) {
                "🌞".to_string() // Day
            } else {
                "✨".to_string() // Night
            }
        }
        "Patchy rain possible" | "Patchy light drizzle" | "Light drizzle" | "Freezing drizzle" | "Mist" => "🌧️".to_string(),
        "Patchy snow possible" | "Patchy sleet possible" | "Patchy freezing drizzle possible" | "Blowing snow" | "Blizzard" | "Patchy snow nearby" | "Patchy sleet nearby" | "Patchy freezing drizzle nearby" => "🌨️".to_string(),
        "Thundery outbreaks possible" | "Moderate or heavy rain with thunder" => "⛈️".to_string(),
        "Fog" | "Freezing fog" => "🌫️".to_string(),
        "Partly Cloudy" | "Partly cloudy" => "🌤️".to_string(),
        "Overcast" => "☁️".to_string(),
        s if s.to_lowercase().contains("snow") => "🌨️".to_string(),
        s if s.to_lowercase().contains("rain") => "🌧️".to_string(),
        _ => condition.to_string(),
    }
}

fn display_weather_actual(location: &str, weather_data: &CurrentWeatherData, unit: String, export: bool) {
    println!("\n{}", format!("Current temperature in {}: {}°C, wind: {} kph with {} precipitation", location, weather_data.current.current.temp_c, weather_data.current.current.wind_kph, weather_data.current.current.precip_mm).blue());
    println!("{}", format!("Hourly forecast for {}", location).magenta());
    
    let mut hourly_weather = weather_data.hourly.forecast.forecastday[0].hour.clone();
    for hour in hourly_weather.iter_mut() {
        hour.condition.text = get_emoji_for_time(&hour.condition.text, &hour.time);
    }

    let table = vec![
        vec!["Temperature".cell().bold(true)]
            .into_iter()
            .chain(hourly_weather.iter().map(|hour| {
                if unit == "imperial" {
                    format!("{}°F", hour.temp_f).cell().justify(Justify::Center)
                } else {
                    format!("{}°C", hour.temp_c).cell().justify(Justify::Center)
                }
            }))
            .collect::<Vec<_>>(),
        vec!["Weather".cell().bold(true)]
            .into_iter()
            .chain(hourly_weather.iter().map(|hour| {
                hour.condition.text.clone().cell().justify(Justify::Center)
            }))
            .collect::<Vec<_>>(),
        vec!["Wind".cell().bold(true)]
            .into_iter()
            .chain(hourly_weather.iter().map(|hour| {
                if unit == "imperial" {
                    format!("{} mph", hour.wind_mph).cell().justify(Justify::Center)
                } else {
                    format!("{} kph", hour.wind_kph).cell().justify(Justify::Center)
                }
            }))
            .collect::<Vec<_>>(),
    ]
    .table()
    .title(
        vec!["Hour".cell().bold(true)]
            .into_iter()
            .chain((0..24).map(|hour| {
                format!("{:02}:00", hour).cell().bold(true)
            }))
            .collect::<Vec<_>>()
    )
    .bold(true);

    let table_display = table.display().unwrap();
    println!("{}", table_display);

    if export {
        match file_io::export_weather_data_now(location, weather_data.clone()) {
            Ok(_) => println!("{}", "Weather data exported as json.".green()),
            Err(e) => println!("{}", format!("Error: {:?}", e).red()),
        }
    }
}