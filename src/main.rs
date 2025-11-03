use clap::Parser;
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
mod icons;
mod utils;

#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: i32,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    #[serde(rename = "current_units", default)]
    current_weather_units: Option<CurrentWeatherUnits>,
    #[serde(rename = "current")]
    current_weather: CurrentWeather,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentWeatherUnits {
    time: String,
    interval: String,
    temperature_2m: String,
    relative_humidity_2m: String,
    wind_speed_10m: String,
    weather_code: String,
    precipitation: String,
    wind_direction_10m: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentWeather {
    time: String,
    interval: i32,
    temperature_2m: f64,
    relative_humidity_2m: i32,
    wind_speed_10m: f64,
    weather_code: i32,
    precipitation: f64,
    wind_direction_10m: i32,
}

#[derive(Parser)]
#[command(version, about = "Console utility for displaying weather")]
struct Args {
    #[arg(short = 'c', long = "city", default_value = "Cairo")]
    city: String,
}

fn print_data(weather_data: WeatherResponse) {
    let weather = match weather_data.current_weather.weather_code {
        0 => "Clear".to_string(),
        1 => "Partly cloudy".to_string(),
        2 => "Cloudy".to_string(),
        3 => "Overcast".to_string(),
        45 | 48 => "Fog".to_string(),
        51 | 53 | 55 => "Drizzle".to_string(),
        61 | 63 | 65 => "Rain".to_string(),
        71 | 73 | 75 => "Snow".to_string(),
        80 | 81 | 82 => "Downpour".to_string(),
        95 | 96 | 99 => "Thunderstorm".to_string(),
        _ => "Unknown weather".to_string(),
    };

    println!("Weather: {}", &weather);
    println!(
        "Temperature: {}'C",
        weather_data.current_weather.temperature_2m
    );
    println!(
        "Humidity: {}%",
        weather_data.current_weather.relative_humidity_2m
    );
    println!(
        "Precipitation: {} mm",
        weather_data.current_weather.precipitation
    );
    println!(
        "Wind speed: {} km/h",
        weather_data.current_weather.wind_speed_10m
    );
    println!(
        "Wind direction: {}°",
        weather_data.current_weather.wind_direction_10m
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let (lat, lon) = utils::get_coordinates(&args.city).await?;
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,wind_speed_10m,weather_code,precipitation,wind_direction_10m&forecast_days=1",
        lat, lon
    );

    let response = reqwest::get(&url).await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let weather_data: WeatherResponse = response.json().await?;
            print_data(weather_data);
        }
        status => {
            eprintln!("The request ended with an error: {}", status);
            let error_text = response.text().await?;
            eprintln!("Error body: {}", error_text);
        }
    }

    Ok(())
}
