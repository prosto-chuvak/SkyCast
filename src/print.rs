use crate::WeatherResponse;

pub fn print_data(weather_data: &WeatherResponse) {
    let weather_code = weather_data.current_weather.weather_code;

    match weather_code {
        0 => {
            println!("             ");
            println!(
                "    \\   /    \tTemperature: {}'C",
                weather_data.current_weather.temperature_2m
            );
            println!(
                "     .-.     \tHumidity: {}%",
                weather_data.current_weather.relative_humidity_2m
            );
            println!(
                "  ― (   ) ―  \tPrecipitation: {}mm",
                weather_data.current_weather.precipitation
            );
            println!(
                "     `-’     \tWind speed: {}km/h",
                weather_data.current_weather.wind_speed_10m
            );
            println!(
                "    /   \\    \tWind direction: {}°",
                weather_data.current_weather.wind_direction_10m
            );
            println!("             ");
        }
        1 => {
            println!("             ");
            println!(
                "   \\  /      \tTemperature: {}'C",
                weather_data.current_weather.temperature_2m
            );
            println!(
                " _ /\"\".-.    \tHumidity: {}%",
                weather_data.current_weather.relative_humidity_2m
            );
            println!(
                "   \\_(   ).  \tPrecipitation: {}mm",
                weather_data.current_weather.precipitation
            );
            println!(
                "   /(___(__) \tWind speed: {}km/h",
                weather_data.current_weather.wind_speed_10m
            );
            println!(
                "             \tWind direction: {}°",
                weather_data.current_weather.wind_direction_10m
            );
            println!("             ");
        }
        2 => {
            println!("             ");
            println!(
                "             \tTemperature: {}'C",
                weather_data.current_weather.temperature_2m
            );
            println!(
                "     .--.    \tHumidity: {}%",
                weather_data.current_weather.relative_humidity_2m
            );
            println!(
                "  .-(    ).  \tPrecipitation: {}mm",
                weather_data.current_weather.precipitation
            );
            println!(
                " (___.__)__) \tWind speed: {}km/h",
                weather_data.current_weather.wind_speed_10m
            );
            println!(
                "             \tWind direction: {}°",
                weather_data.current_weather.wind_direction_10m
            );
            println!("             ");
        }
        3 => {
            println!("         .-. ");
            println!(
                "        (   )\tTemperature: {}'C",
                weather_data.current_weather.temperature_2m
            );
            println!(
                "     .--.`-' \tHumidity: {}%",
                weather_data.current_weather.relative_humidity_2m
            );
            println!(
                "  .-(    ).  \tPrecipitation: {}mm",
                weather_data.current_weather.precipitation
            );
            println!(
                " (___.__)__) \tWind speed: {}km/h",
                weather_data.current_weather.wind_speed_10m
            );
            println!(
                "             \tWind direction: {}°",
                weather_data.current_weather.wind_direction_10m
            );
            println!("             ");
        }
        45 | 48 => {
            println!("             ");
            println!(
                "             \tTemperature: {}'C",
                weather_data.current_weather.temperature_2m
            );
            println!(
                " _ - _ - _ - \tHumidity: {}%",
                weather_data.current_weather.relative_humidity_2m
            );
            println!(
                "  _ - _ - _  \tPrecipitation: {}mm",
                weather_data.current_weather.precipitation
            );
            println!(
                " _ - _ - _ - \tWind speed: {}km/h",
                weather_data.current_weather.wind_speed_10m
            );
            println!(
                "             \tWind direction: {}°",
                weather_data.current_weather.wind_direction_10m
            );
            println!("             ");
        }
        51 | 53 | 55 => {
            println!("             ");
            println!(
                "     .--.     \tTemperature: {}'C",
                weather_data.current_weather.temperature_2m
            );
            println!(
                "  .-(    ).   \tHumidity: {}%",
                weather_data.current_weather.relative_humidity_2m
            );
            println!(
                " (___.__)__)  \tPrecipitation: {}mm",
                weather_data.current_weather.precipitation
            );
            println!(
                "   , '   '    \tWind speed: {}km/h",
                weather_data.current_weather.wind_speed_10m
            );
            println!(
                "  ' , '       \tWind direction: {}°",
                weather_data.current_weather.wind_direction_10m
            );
            println!("             ");
        }
        61 | 63 | 65 => {
            println!("             ");
            println!(
                "     .--.     \tTemperature: {}'C",
                weather_data.current_weather.temperature_2m
            );
            println!(
                "  .-(    ).   \tHumidity: {}%",
                weather_data.current_weather.relative_humidity_2m
            );
            println!(
                " (___.__)__)  \tPrecipitation: {}mm",
                weather_data.current_weather.precipitation
            );
            println!(
                "   ' ' ' '    \tWind speed: {}km/h",
                weather_data.current_weather.wind_speed_10m
            );
            println!(
                "  ' ' ' '     \tWind direction: {}°",
                weather_data.current_weather.wind_direction_10m
            );
            println!("             ");
        }
        71 | 73 | 75 => {
            println!("             ");
            println!(
                "     .-.     \tTemperature: {}'C",
                weather_data.current_weather.temperature_2m
            );
            println!(
                "    (   ).   \tHumidity: {}%",
                weather_data.current_weather.relative_humidity_2m
            );
            println!(
                "   (___(__)  \tPrecipitation: {}mm",
                weather_data.current_weather.precipitation
            );
            println!(
                "    *  *  *  \tWind speed: {}km/h",
                weather_data.current_weather.wind_speed_10m
            );
            println!(
                "   *  *  *   \tWind direction: {}°",
                weather_data.current_weather.wind_direction_10m
            );
            println!("             ");
        }
        80 | 81 | 82 => {
            println!("             ");
            println!(
                "     .--.     \tTemperature: {}'C",
                weather_data.current_weather.temperature_2m
            );
            println!(
                "  .-(    ).   \tHumidity: {}%",
                weather_data.current_weather.relative_humidity_2m
            );
            println!(
                " (___.__)__)  \tPrecipitation: {}mm",
                weather_data.current_weather.precipitation
            );
            println!(
                "  ,',',','    \tWind speed: {}km/h",
                weather_data.current_weather.wind_speed_10m
            );
            println!(
                " ,',',','     \tWind direction: {}°",
                weather_data.current_weather.wind_direction_10m
            );
            println!("             ");
        }
        95 | 96 | 99 => {
            println!("             ");
            println!(
                "     .-.     \tTemperature: {}'C",
                weather_data.current_weather.temperature_2m
            );
            println!(
                "    (   ).   \tHumidity: {}%",
                weather_data.current_weather.relative_humidity_2m
            );
            println!(
                "   (___(__)  \tPrecipitation: {}mm",
                weather_data.current_weather.precipitation
            );
            println!(
                "    Z\"\"Z\"\" \tWind speed: {}km/h",
                weather_data.current_weather.wind_speed_10m
            );
            println!(
                "  ‚'‚'‚'‚'   \tWind direction: {}°",
                weather_data.current_weather.wind_direction_10m
            );
            println!("             ");
        }
        _ => {
            println!("             ");
            println!("    .-.      ");
            println!("     __)     ");
            println!("    (        ");
            println!("     `-’     ");
            println!("      •      ");
            println!("             ");
        }
    }
}
