use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    #[serde(rename = "current_units", default)]
    pub current_weather_units: Option<CurrentWeatherUnits>,
    #[serde(rename = "current")]
    pub current_weather: CurrentWeather,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentWeatherUnits {
    pub time: String,
    pub interval: String,
    pub temperature_2m: String,
    pub relative_humidity_2m: String,
    pub wind_speed_10m: String,
    pub weather_code: String,
    pub precipitation: String,
    pub wind_direction_10m: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentWeather {
    pub time: String,
    pub interval: i32,
    pub temperature_2m: f64,
    pub relative_humidity_2m: i32,
    pub wind_speed_10m: f64,
    pub weather_code: i32,
    pub precipitation: f64,
    pub wind_direction_10m: i32,
}
