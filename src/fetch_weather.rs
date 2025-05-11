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
    pub lon: f64,
    pub lat: f64,
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
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    #[serde(rename = "temp_min")]
    pub temp_min: f64,
    #[serde(rename = "temp_max")]
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
    #[serde(rename = "sea_level")]
    pub sea_level: Option<i32>, // Added Option in case it's sometimes null
    #[serde(rename = "grnd_level")]
    pub grnd_level: Option<i32>, // Added Option in case it's sometimes null
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: i32,
    pub gust: Option<f64>, // Added Option in case it's sometimes null
}

#[derive(Deserialize, Debug)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: i32, // Renamed 'type' to 'type_field' to avoid Rust keyword
    pub id: i32,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
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
// Get weather condition from weather id
pub fn condition(id: i32) -> String {
    match id {
        199..233 => String::from("thunder"),
        299..321 => String::from("drizzle"),
        499..532 => String::from("rain"),
        599..623 => String::from("snow"),
        700..781 => String::from("mist"),
        800 => String::from("clear"),
        _ => String::from("clouds"),
    }
}
