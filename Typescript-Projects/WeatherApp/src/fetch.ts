import chalk from "chalk";
import { WeatherTodayData, WeatherDefaultResponse, HourlyForecast, WeeklyForecast, WeatherDayData } from "./weather";

export async function getTodayWeatherData(location: string): Promise<WeatherTodayData | null> {
    try {
        const url = `https://api.weatherapi.com/v1/forecast.json?key=8a8c7fd6149341a8ba6135014251101&q=${location}&days=1`;
        const response = await fetch(url);
        const data = await response.json();

        if (!data.current || !data.forecast?.forecastday?.[0]?.hour) {
            console.log(chalk.red("Location not found or invalid API response"));
            return null;
        }

        const current: WeatherDefaultResponse = {
            temp_c: data.current.temp_c,
            temp_f: data.current.temp_f,
            wind_kph: data.current.wind_kph,
            wind_mph: data.current.wind_mph,
            condition: data.current.condition.text,
        };

        const hourlyForecasts: HourlyForecast[] = new Array(24);
        const hours = data.forecast.forecastday[0].hour;

        for (let i = 0; i < 24; i++) {
            const hour = hours[i];
            hourlyForecasts[i] = {
                hour: hour.time.split(' ')[1],
                weather: {
                    temp_c: hour.temp_c,
                    temp_f: hour.temp_f,
                    wind_kph: hour.wind_kph,
                    wind_mph: hour.wind_mph,
                    condition: hour.condition.text,
                }
            };
        }

        return {
            current,
            hourly: hourlyForecasts,
            timestamp: Math.floor(Date.now() / 1000)
        };
    } catch (error) {
        console.log(chalk.red("Failed to fetch weather data"));
        return null;
    }
}

export async function getForecastWeatherData(location: string): Promise<WeeklyForecast | null> {
    try {
        const url = `https://api.weatherapi.com/v1/forecast.json?key=8a8c7fd6149341a8ba6135014251101&q=${location}&days=3`;
        const response = await fetch(url);
        const data = await response.json();

        if (!data.forecast?.forecastday) {
            console.log(chalk.red("Location not found or invalid API response"));
            return null;
        }

        const dailyForecasts: WeatherDayData[] = data.forecast.forecastday.map((forecastDay: any) => {
            const date = new Date(forecastDay.date);
            const dayName = date.toLocaleDateString('en-US', { weekday: 'long' });

            return {
                day: dayName,
                avgtemp_c: forecastDay.day.avgtemp_c,
                avgtemp_f: forecastDay.day.avgtemp_f,
                maxwind_mph: forecastDay.day.maxwind_mph,
                maxwind_kph: forecastDay.day.maxwind_kph,
                condition: forecastDay.day.condition.text,
            };
        });

        return {
            day: dailyForecasts,
            timestamp: Math.floor(Date.now() / 1000)
        };
    } catch (error) {
        console.log(chalk.red("Failed to fetch weather data"));
        return null;
    }
}