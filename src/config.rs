use crate::Daytime;
use crate::Mode;
use crate::PathBuf;
use crate::Settings;
use crate::WeatherType;
use anyhow::anyhow;
use anyhow::{Context, Result};
use ini::Ini;
use std::collections::HashMap;

fn get_interval(interval: &str) -> i32 {
    interval.parse::<i32>().unwrap() + 1
}

// Find path to home directory for reading config file
fn fetch_config_path() -> Result<PathBuf> {
    Ok(home::home_dir()
        .ok_or_else(|| anyhow!("Could not determine user's home directory"))?
        .join(".config")
        .join("fehther")
        .join("config.ini"))
}

// Read config file and load configuration
pub fn fetch_config() -> Result<Settings> {
    let config_path = fetch_config_path()?;
    let config = Ini::load_from_file(&config_path)
        .with_context(|| format!("Failed to load config file: {}", config_path.display()))?;

    let default_weather_types = [
        "clear",
        "few-clouds",
        "drizzle",
        "mist",
        "rain",
        "snow",
        "thunder",
        "scattered-clouds",
        "overcast-clouds",
        "broken-clouds",
    ];

    fn get_weathertype(weathertype: &str) -> Result<WeatherType> {
        match weathertype.to_lowercase().as_str() {
            "clear" => Ok(WeatherType::Clear),
            "few-clouds" => Ok(WeatherType::FewClouds),
            "scattered-clouds" => Ok(WeatherType::ScatteredClouds),
            "broken-clouds" => Ok(WeatherType::BrokenClouds),
            "overcast-clouds" => Ok(WeatherType::OvercastClouds),
            "drizzle" => Ok(WeatherType::Drizzle),
            "mist" => Ok(WeatherType::Mist),
            "rain" => Ok(WeatherType::Rain),
            "snow" => Ok(WeatherType::Snow),
            "thunder" => Ok(WeatherType::Thunder),
            _ => Err(anyhow!("not a known weathertype: {}", weathertype)),
        }
    }

    let key = config
        .get_from(Some("settings"), "key")
        .unwrap_or_default()
        .to_string();
    let city = config
        .get_from(Some("settings"), "city")
        .unwrap_or_default()
        .to_string();
    let country = config
        .get_from(Some("settings"), "country")
        .unwrap_or_default()
        .to_string();
    let path = config
        .get_from(Some("settings"), "path")
        .unwrap_or_default()
        .to_string();

    let modes = config
        .section(Some("modes"))
        .map(|mode_section| {
            mode_section
                .iter()
                .filter_map(|(mode, active)| {
                    if Mode::to_string_vec().contains(&mode.to_string()) && active == "true" {
                        Mode::from_string(mode).ok()
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let custom_folder_names = config
        .get_from(Some("folders"), "custom-folder-names")
        .unwrap_or("false")
        == "true";

    let custom_weather_groups = config
        .get_from(Some("weather-groups"), "weather-groups")
        .unwrap_or("false")
        == "true";

    let mut weather_groups = HashMap::new();
    if custom_weather_groups {
        if let Some(weather_groups_section) = config.section(Some("weather-groups")) {
            for (folder_name, weather_types_str) in weather_groups_section.iter() {
                let weather_types: Vec<WeatherType> = weather_types_str
                    .split_whitespace()
                    .filter(|weather| default_weather_types.contains(weather))
                    .filter_map(|weather| get_weathertype(weather).ok())
                    .collect();
                if !weather_types.is_empty() {
                    weather_groups.insert(folder_name.to_string(), weather_types);
                }
            }
        }
    }

    let sunset_time = get_interval(
        config
            .get_from(Some("modes"), "sunset-time")
            .unwrap_or("60"),
    );
    let interval = get_interval(
        config
            .get_from(Some("modes"), "cycle-timer")
            .unwrap_or("60"),
    );

    let mut disabled_daytimes: Vec<Daytime> = Vec::new();

    for daytime in config
        .get_from(Some("modes"), "disabled-weather-modes")
        .unwrap_or("")
        .to_lowercase()
        .split_whitespace()
    {
        match daytime {
            "sunrise" => disabled_daytimes.push(Daytime::Sunrise),
            "day" => disabled_daytimes.push(Daytime::Day),
            "sunset" => disabled_daytimes.push(Daytime::Sunset),
            _ => disabled_daytimes.push(Daytime::Night),
        }
    }
    let mut folder_names_map = HashMap::new();

    Ok(Settings {
        current_loop: true,
        key,
        city,
        country,
        path,
        modes,
        disabled_daytimes: Some(disabled_daytimes),
        folder_names: if custom_folder_names {
            if let Some(folder_name) = config.get_from(Some("folders"), "daytime-folder-name") {
                folder_names_map
                    .entry(Daytime::Day)
                    .or_insert(folder_name.to_string());
            } else {
                folder_names_map.insert(Daytime::Day, "day".to_string());
            }
            if let Some(folder_name) = config.get_from(Some("folders"), "sunset-folder-name") {
                folder_names_map
                    .entry(Daytime::Sunset)
                    .or_insert(folder_name.to_string());
            } else {
                folder_names_map.insert(Daytime::Sunset, "sunset".to_string());
            }
            if let Some(folder_name) = config.get_from(Some("folders"), "sunrise-folder-name") {
                folder_names_map
                    .entry(Daytime::Sunrise)
                    .or_insert(folder_name.to_string());
            } else {
                folder_names_map.insert(Daytime::Sunrise, "sunrise".to_string());
            }
            if let Some(folder_name) = config.get_from(Some("folders"), "nighttime-folder-name") {
                folder_names_map
                    .entry(Daytime::Night)
                    .or_insert(folder_name.to_string());
            } else {
                folder_names_map.insert(Daytime::Night, "night".to_string());
            }
            folder_names_map
        } else {
            folder_names_map.insert(Daytime::Day, "day".to_string());
            folder_names_map.insert(Daytime::Night, "night".to_string());
            folder_names_map.insert(Daytime::Sunrise, "sunrise".to_string());
            folder_names_map.insert(Daytime::Sunset, "sunset".to_string());
            folder_names_map
        },
        custom_weather_groups,
        weather_groups,
        sunset_time,
        interval,
        daytime: Daytime::Day,
        weather: WeatherType::Clear,
        feh_mode: format!(
            "--bg-{}",
            config.get_from(Some("modes"), "feh-mode").unwrap_or("fill")
        ),
        timer: 0,
    })
}
