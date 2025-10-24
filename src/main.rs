use serde::{Deserialize, Serialize}; // Для десериализации JSON в структуры

// Определяем структуры, соответствующие JSON
#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: i32,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    current_weather_units: CurrentWeatherUnits,
    current_weather: CurrentWeather,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentWeatherUnits {
    time: String,
    interval: String,
    temperature: String,
    windspeed: String,
    winddirection: String,
    is_day: String,
    weathercode: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentWeather {
    time: String,
    interval: i32,
    temperature: f64,
    windspeed: f64,
    winddirection: f64,
    is_day: i32,
    weathercode: i32,
}

fn print_data(weather_data: WeatherResponse) {
    println!("\n--- Данные о погоде ---");
    println!("Широта: {}", weather_data.latitude);
    println!("Долгота: {}", weather_data.longitude);
    println!("Время генерации (мс): {}", weather_data.generationtime_ms);
    println!("Смещение UTC (сек): {}", weather_data.utc_offset_seconds);
    println!("Часовой пояс: {}", weather_data.timezone);
    println!("Аббревиатура часового пояса: {}", weather_data.timezone_abbreviation);
    println!("Высота над уровнем моря (м): {}", weather_data.elevation);

    println!("\n--- Единицы измерения ---");
    println!("Время: {}", weather_data.current_weather_units.time);
    println!("Интервал: {}", weather_data.current_weather_units.interval);
    println!("Температура: {}", weather_data.current_weather_units.temperature);
    println!("Скорость ветра: {}", weather_data.current_weather_units.windspeed);
    println!("Направление ветра: {}", weather_data.current_weather_units.winddirection);
    println!("Код погоды: {}", weather_data.current_weather_units.weathercode);

    println!("\n--- Текущая погода ---");
    println!("Время: {}", weather_data.current_weather.time);
    println!("Интервал (с): {}", weather_data.current_weather.interval);
    println!("Температура: {}°C", weather_data.current_weather.temperature);
    println!("Скорость ветра: {} км/ч", weather_data.current_weather.windspeed);
    println!("Направление ветра: {}°", weather_data.current_weather.winddirection);
    // is_day: 0 - ночь, 1 - день
    println!("День (1) или Ночь (0): {}", weather_data.current_weather.is_day);
    println!("Код погоды (WMO): {}", weather_data.current_weather.weathercode);

    if weather_data.current_weather.is_day != 0 {
        println!("Сейчас день.");
    } else {
        println!("Сейчас ночь.");
    }

    match weather_data.current_weather.weathercode {
        0 => println!("Ясно."),
        1 => println!("Малооблачно"),
        2 => println!("Облачно"),
        3 => println!("Пасмурно"),
        45 | 48 => println!("Туман."),
        51 | 53 | 55 => println!("Морось."),
        61 | 63 | 65 => println!("Дождь."),
        71 | 73 | 75 => println!("Снег."),
        80 | 81 | 82 => println!("Ливень."),
        95 | 96 | 99 => println!("Гроза."),
        _ => println!("Неизвестный код погоды WMO: {}", weather_data.current_weather.weathercode),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lat = 56.13222;
    let lon = 47.25194;
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true&timezone=GMT", lat, lon);

    println!("Отправляем GET-запрос на {}", url);

    let response = ureq::get(&url)
        .call()?;

    let status = response.status();
    println!("Статус ответа: {}", status);

    if status == 200 {
        let mut response_body = response.into_body();
        let response_text = response_body.read_to_string()?;

        print_data(serde_json::from_str(&response_text)?);
    } else {
        eprintln!("Запрос завершился с ошибкой: {}", status);
        // Попробуем прочитать тело ошибки для отладки
        let mut response_body = response.into_body();
        let error_text = response_body.read_to_string()?;
        eprintln!("Тело ошибки: {}", error_text);
    }

    Ok(())
}