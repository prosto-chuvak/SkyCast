use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Place {
    #[serde(rename = "lat")]
    latitude: String,
    #[serde(rename = "lon")]
    longitude: String,
}

pub async fn get_coordinates(client: &reqwest::Client, city: &str) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search?city={}&format=json&limit=1",
        city
    );

    let places: Vec<Place> = client
        .get(&url)
        .header("User-Agent", "my-rust-app")
        .send()
        .await?
        .json()
        .await?;

    if let Some(first) = places.first() {
        let lat: f64 = first.latitude.parse()
            .map_err(|_| format!("Failed to parse latitude: {}", first.latitude))?;
        let lon: f64 = first.longitude.parse()
            .map_err(|_| format!("Failed to parse longitude: {}", first.longitude))?;
        Ok((lat, lon))
    } else {
        Err("City not found!".into())
    }
}
