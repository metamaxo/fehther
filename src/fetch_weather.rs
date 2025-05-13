#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}

#[derive(Deserialize, Debug)]
pub struct Coord {
    pub lon: Option<f64>,
    pub lat: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: Option<f64>,
    #[serde(rename = "feels_like")]
    pub feels_like: Option<f64>,
    #[serde(rename = "temp_min")]
    pub temp_min: Option<f64>,
    #[serde(rename = "temp_max")]
    pub temp_max: Option<f64>,
    pub pressure: Option<i32>,
    pub humidity: Option<i32>,
    #[serde(rename = "sea_level")]
    pub sea_level: Option<i32>,
    #[serde(rename = "grnd_level")]
    pub grnd_level: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: Option<f64>,
    pub deg: Option<i32>,
    pub gust: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Clouds {
    pub all: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: Option<i32>,
    pub id: i32,
    pub country: Option<String>,
    pub sunrise: i32,
    pub sunset: i32,
}

// Get API response from openweathermap
pub async fn openweathermap(
    key: &str,
    city: &str,
    country_code: &str,
) -> Result<WeatherResponse, anyhow::Error> {
    let body = reqwest::get(format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, key
    ))
    .await?
    .text()
    .await?;
    Ok(serde_json::from_str(&body)?)
}
