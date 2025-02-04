// DAY
export interface WeatherTodayData {
    current: WeatherDefaultResponse,
    hourly: HourlyForecast[],
    timestamp: number
}

export interface WeatherDefaultResponse {
    temp_c: number,
    temp_f: number,
    wind_kph: number,
    wind_mph: number,
    condition: string,
}

export interface HourlyForecast {
    hour: string,
    weather: WeatherDefaultResponse,
}


// FORECAST
export interface WeeklyForecast {
    day: WeatherDayData[],
    timestamp: number
}

export interface WeatherDayData {
    day: string,
    avgtemp_c: number,
    avgtemp_f: number,
    maxwind_mph: number,
    maxwind_kph: number,
    condition: string,
}