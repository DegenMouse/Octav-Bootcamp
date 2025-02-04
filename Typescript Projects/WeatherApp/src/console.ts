import { getTodayWeatherData, getForecastWeatherData } from "./fetch";
import Table from 'cli-table3';
import * as fs from "fs";
import { WeatherTodayData, WeeklyForecast } from "./weather";

export async function displayTodaysWeather(location: string, imperial: boolean, weatherData: WeatherTodayData) {
    try {
        fs.writeFileSync(`offline_data/day/${location}.json`, JSON.stringify(weatherData, null, 2));
        
        console.log('\nCurrent Weather in ' + location + ':');
        console.log(weatherData.current.condition);
        console.log((imperial ? weatherData.current.temp_f : weatherData.current.temp_c) + 
            (imperial ? ' °F' : ' °C'));
        console.log((imperial ? weatherData.current.wind_mph : weatherData.current.wind_kph) + 
            (imperial ? ' mph' : ' kph'));
        console.log(); 

        var table = new Table({
            head: ['Hour', 'Condition', 'Temperature', 'Wind'],
            colWidths: [7, 15, 10, 10],
            colAligns: ['center', 'center', 'center', 'center']
        });
        
        for(let i = 0; i < 24; i++) {
            const hourStr = i.toString().padStart(2, '0') + ':00';
            table.push([
                hourStr,
                weatherData.hourly[i].weather.condition,
                (imperial ? weatherData.hourly[i].weather.temp_f : weatherData.hourly[i].weather.temp_c) + 
                    (imperial ? ' °F' : ' °C'),
                (imperial ? weatherData.hourly[i].weather.wind_mph : weatherData.hourly[i].weather.wind_kph) + 
                    (imperial ? ' mph' : ' kph')
            ]);
        }
        
        console.log('Hourly Forecast:');
        console.log(table.toString());

    } catch (error) {
        console.error('Error fetching weather data:', error);
    }
}

export async function displayForecastWeather(location: string, imperial: boolean, weatherData: WeeklyForecast) {
    try {
        fs.writeFileSync(`offline_data/forecast/${location}.json`, JSON.stringify(weatherData, null, 2));
        
        var table = new Table({
            head: ['Day', 'Condition', 'Temperature', 'Wind'],
            colWidths: [12, 15, 10, 10],
            colAligns: ['center', 'center', 'center', 'center']
        });

        weatherData.day.forEach(day => {
            table.push([
                day.day,
                day.condition,
                (imperial ? day.avgtemp_f : day.avgtemp_c) + 
                    (imperial ? ' °F' : ' °C'),
                (imperial ? day.maxwind_mph : day.maxwind_kph) + 
                    (imperial ? ' mph' : ' kph')
            ]);
        });

        console.log('\n3-Day Forecast for ' + location);
        console.log(table.toString());

    } catch (error) {
        console.error('Error fetching weather data:', error);
    }
}