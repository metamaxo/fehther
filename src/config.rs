use crate::Daytime;
use crate::Mode;
use crate::PathBuf;
use crate::Settings;
use crate::WeatherType;
use anyhow::anyhow;
use anyhow::{Context, Result};
use ini::{Ini, Properties}; // Import Ini and Properties
use std::collections::HashMap;

fn get_mode(mode: &str) -> Mode {
    match mode.to_lowercase().as_str() {
        "on" => Mode::On,
        _ => Mode::Off,
    }
}

fn get_interval(interval: &str) -> i32 {
    interval.parse::<i32>().unwrap() + 1
}

// Find path to home directory for reading config file
fn fetch_config_path() -> Result<PathBuf, anyhow::Error> {
    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => anyhow::bail!("Could not determine user's home directory"),
    };
    let config_path = home_dir
        .join(".config")
        .join("wallpaper-manager")
        .join("config.ini");
    Ok(config_path)
}

fn get_folder(config: Option<String>, default: &str) -> String {
    match config {
        Some(map) => map.to_string(),
        None => default.to_string(),
    }
}

// Read config file and load configuration
pub fn fetch_config() -> Result<Settings, anyhow::Error> {
    let config_path = fetch_config_path()?;
    let config = Ini::load_from_file(&config_path)
        .with_context(|| format!("Failed to load config file: {}", config_path.display()))?;

    let default_weather_types: Vec<&str> = vec![
        "clear", "clouds", "drizzle", "mist", "rain", "snow", "thunder",
    ];
    fn get_weathertype(weathertype: &str) -> Result<WeatherType, anyhow::Error> {
        match weathertype.to_lowercase().as_str() {
            "clear" => Ok(WeatherType::Clear),
            "clouds" => Ok(WeatherType::Clouds),
            "drizzle" => Ok(WeatherType::Drizzle),
            "mist" => Ok(WeatherType::Mist),
            "rain" => Ok(WeatherType::Rain),
            "snow" => Ok(WeatherType::Snow),
            "thunder" => Ok(WeatherType::Thunder),
            _ => Err(anyhow!("not a known weathertype")),
        }
    }

    let mut settings = Settings {
        current_loop: Mode::On,
        key: config
            .get_from(Some("settings"), "key")
            .unwrap_or("")
            .to_string(),
        city: config
            .get_from(Some("settings"), "city")
            .unwrap_or("")
            .to_string(),
        country: config
            .get_from(Some("settings"), "country")
            .unwrap_or("")
            .to_string(),
        path: config
            .get_from(Some("settings"), "path")
            .unwrap_or("")
            .to_string(),
        daytime_mode: get_mode(
            config
                .get_from(Some("modes"), "daytime-mode")
                .unwrap_or("off"),
        ),
        custom_folder_names: get_mode(
            config
                .get_from(Some("folders"), "custom-folder-names")
                .unwrap_or("off"),
        ),
        daytime_folder_name: "day".to_string(),     // Default
        nighttime_folder_name: "night".to_string(), // Default
        sunrise_folder_name: "sunrise".to_string(), // Default
        sunset_folder_name: "sunset".to_string(),   // Default
        weather_mode: get_mode(
            config
                .get_from(Some("modes"), "weather-mode")
                .unwrap_or("off"),
        ),
        weather_groups_mode: get_mode(
            config
                .get_from(Some("weather-groups"), "weather-groups")
                .unwrap_or("off"),
        ),
        weather_groups: {
            let mut groups = HashMap::new();
            if let Some(weather_groups_section) = config.section(Some("weather-groups")) {
                for (folder_name, weather_types_str) in weather_groups_section.iter() {
                    let weather_types: Vec<WeatherType> = weather_types_str
                        .split_whitespace()
                        .filter(|weather| default_weather_types.contains(weather))
                        .map(|weather| get_weathertype(weather).unwrap())
                        .collect();
                    if !weather_types.is_empty() {
                        groups.insert(folder_name.to_string(), weather_types);
                    }
                }
            }
            groups
        },
        daytime_weather_mode: get_mode(
            config
                .get_from(Some("modes"), "daytime-weather-mode")
                .unwrap_or("off"),
        ),
        nighttime_weather_mode: get_mode(
            config
                .get_from(Some("modes"), "nighttime-weather-mode")
                .unwrap_or("off"),
        ),
        sunset_weather_mode: get_mode(
            config
                .get_from(Some("modes"), "sunset-weather-mode")
                .unwrap_or("off"),
        ),
        sunrise_weather_mode: get_mode(
            config
                .get_from(Some("modes"), "sunrise-weather-mode")
                .unwrap_or("off"),
        ),
        sunset_mode: get_mode(
            config
                .get_from(Some("modes"), "sunset-mode")
                .unwrap_or("off"),
        ),
        sunset_time: get_interval(
            config
                .get_from(Some("modes"), "sunset-time")
                .unwrap_or("60"),
        ), //default 60
        cycle_mode: get_mode(
            config
                .get_from(Some("modes"), "cycle-mode")
                .unwrap_or("off"),
        ),
        interval: get_interval(config.get_from(Some("modes"), "interval").unwrap_or("60")), //default 60
        daytime: Daytime::Day,       // Default
        weather: WeatherType::Clear, // Default
        feh_mode: format!(
            "--bg-{}",
            config.get_from(Some("modes"), "feh-mode").unwrap_or("fill")
        ), // Default
        timer: 0,                    // Default
    };

    // Apply custom folder names if enabled
    if settings.custom_folder_names == Mode::On {
        let assign_folder_name = |config: &Ini, key: &str, target: &mut String| {
            if let Some(folder_name) = config.get_from(Some("folders"), key) {
                *target = folder_name.to_string();
            }
        };
        assign_folder_name(
            &config,
            "daytime-folder-name",
            &mut settings.daytime_folder_name,
        );
        assign_folder_name(
            &config,
            "nighttime-folder-name",
            &mut settings.nighttime_folder_name,
        );
        assign_folder_name(
            &config,
            "sunrise-folder-name",
            &mut settings.sunrise_folder_name,
        );
        assign_folder_name(
            &config,
            "sunset-folder-name",
            &mut settings.sunset_folder_name,
        );
    }

    Ok(settings)
}
