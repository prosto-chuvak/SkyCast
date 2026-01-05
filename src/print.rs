use crate::icon;
use crate::utils;
use crate::WeatherResponse;

pub fn print_data(weather_data: &WeatherResponse) {
    let weather_code = weather_data.current_weather.weather_code;
    let mut art = icon::get_weather_art(weather_code as i32);
    art[1][0].push_str("\t\tTemperature: ");
    art[1][0].push_str(&weather_data.current_weather.temperature_2m.to_string());
    art[1][0].push_str("'C");
    
    art[2][0].push_str("\t\tHumidity: ");
    art[2][0].push_str(&weather_data.current_weather.relative_humidity_2m.to_string());
    art[2][0].push_str("%");
    
    art[3][0].push_str("\t\tPrecipitation: ");
    art[3][0].push_str(&weather_data.current_weather.precipitation.to_string());
    art[3][0].push_str(" mm");
    
    art[4][0].push_str("\t\tWind speed: ");
    art[4][0].push_str(&utils::kmh_to_ms(weather_data.current_weather.wind_speed_10m));
    art[4][0].push_str(" m/s");
    
    art[5][0].push_str("\t\tWind direction: ");
    art[5][0].push_str(&weather_data.current_weather.wind_direction_10m.to_string());
    art[5][0].push_str("°");
    
    for line in &art {
      println!("{}", line.join(""));
    }
}

