use crate::WeatherResponse;

pub fn print_data(weather_data: &mut WeatherResponse) {
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
        "Precipitation: {}mm",
        weather_data.current_weather.precipitation
    );
    println!(
        "Wind speed: {}km/h",
        weather_data.current_weather.wind_speed_10m
    );
    println!(
        "Wind direction: {}°",
        weather_data.current_weather.wind_direction_10m
    );
}
