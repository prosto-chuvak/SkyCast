// mod icons;
mod print;
mod utils;
mod weather;

use clap::Parser;
use reqwest;
use std::error::Error;
use weather::WeatherResponse;

#[derive(Parser)]
#[command(version, about = "Console utility for displaying weather")]
struct Args {
    #[arg(short = 'c', long = "city", default_value = "Cairo")]
    city: String,
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
            let mut weather_data: WeatherResponse = response.json().await?;
            print::print_data(&mut weather_data);
        }
        status => {
            eprintln!("The request ended with an error: {}", status);
            let error_text = response.text().await?;
            eprintln!("Error body: {}", error_text);
        }
    }

    Ok(())
}
