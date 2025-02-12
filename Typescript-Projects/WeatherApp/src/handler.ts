import { Command } from "commander";
import { getTodayWeatherData, getForecastWeatherData } from './fetch';
import { displayTodaysWeather, displayForecastWeather } from "./console";
import { existsSync } from 'fs';
import * as fs from "fs";
import { WeatherTodayData, WeeklyForecast } from "./weather";
import chalk from 'chalk';

const program = new Command();

program
  .name("WeatherApp")
  .description("A CLI weather app tool with nested commands")
  .version("1.0.0");

const today = new Command("today")
  .description("Get todays forecast, by hour")
  .alias("t")
  .argument("<location>", "Location")
  .option("-i, --imperial", "Displays the data in the imperial sistem")
  .option("--offline", "Use only cached data without API calls")
  .action(async (location, option) => {
    const path = `offline_data/day/${location}.json`;

    if (option.offline) {
      if (!existsSync(path)) {
        console.error(chalk.red(`No cached data found for ${location}`));
        return;
      }
      console.log(chalk.green("Using cached memory"));
      const data: WeatherTodayData = JSON.parse(fs.readFileSync(path, "utf-8"));
      return displayTodaysWeather(location, option.imperial, data);
    }

    const now = Math.floor(Date.now() / 1000);
    const rawData = existsSync(path) ? fs.readFileSync(path, "utf-8") : null;
    if (rawData) {
        const data: WeatherTodayData = JSON.parse(rawData);
        if (now - data.timestamp <= 1800) {
            console.log(chalk.green("Using cached memory"));
            return displayTodaysWeather(location, option.imperial, data);
        }
    }
    const data = await getTodayWeatherData(location);
    if (!data) {
        return;
    }
    console.log(chalk.green("New location saved"));
    return displayTodaysWeather(location, option.imperial, data);
});

const forecast = new Command("forecast")
  .description("Get forecast for the next 3 days")
  .alias("f")
  .argument("<location>", "Location")
  .option("-i, --imperial", "Displays the data in the imperial system")
  .option("--offline", "Use only cached data without API calls")
  .action(async (location, option) => {
    const path = `offline_data/forecast/${location}.json`;

    if (option.offline) {
      if (!existsSync(path)) {
        console.error(chalk.red(`No cached data found for ${location}`));
        return;
      }
      console.log(chalk.green("Using cached memory"));
      const data: WeeklyForecast = JSON.parse(fs.readFileSync(path, "utf-8"));
      return displayForecastWeather(location, option.imperial, data);
    }

    const now = Math.floor(Date.now() / 1000);
    if (existsSync(path)) {
        const rawData = fs.readFileSync(path, "utf-8");
        const data: WeeklyForecast = JSON.parse(rawData);
        if (now - data.timestamp <= 1800) {
            console.log(chalk.green("Using cached memory"));
            return displayForecastWeather(location, option.imperial, data);
        }
    }
    const newData = await getForecastWeatherData(location);
    if (!newData) {
        return;
    }
    console.log(chalk.green("New location saved"));
    displayForecastWeather(location, option.imperial, newData);
  });

program.addCommand(today);
program.addCommand(forecast);

// Export the program
export const weatherProgram = program;
