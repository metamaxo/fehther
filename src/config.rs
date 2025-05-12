extern crate ini;
use crate::Mode;
use crate::PathBuf;
use crate::Settings;
use std::collections::HashMap;

fn get_mode(mode: &str) -> Mode {
    match mode.to_lowercase().as_str() {
        "on" => Mode::On,
        _ => Mode::Off,
    }
}

fn get_interval(interval: &str) -> i64 {
    interval.parse::<i64>().unwrap() + 1
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
    let default_weather_types: Vec<&str> = vec![
        "clear", "clouds", "drizzle", "mist", "rain", "snow", "thunder",
    ];
    let config_path = fetch_config_path()?;
    let config = ini::ini!(config_path.to_str().expect("test"));

    let mut settings = Settings {
        current_loop: Mode::Off,
        key: config["settings"]["key"].clone().unwrap(),
        city: config["settings"]["city"].clone().unwrap(),
        country: config["settings"]["country"].clone().unwrap(),
        path: config["settings"]["path"].clone().unwrap(),
        daytime_mode: get_mode(&config["modes"]["daytime-mode"].clone().unwrap()),
        custom_folder_names: get_mode(&config["folders"]["custom-folder-names"].clone().unwrap()),
        daytime_folder_name: "day".to_string(),
        nighttime_folder_name: "night".to_string(),
        weather_mode: get_mode(&config["modes"]["weather-mode"].clone().unwrap()),
        weather_groups_mode: get_mode(&config["weather-groups"]["weather-groups"].clone().unwrap()),
        weather_groups: HashMap::new(),
        cycle_mode: get_mode(&config["modes"]["cycle-mode"].clone().unwrap()),
        interval: get_interval(&config["modes"]["interval"].clone().unwrap()),
        daytime: "start".to_string(),
        weather: "start".to_string(),
        feh_mode: format!("--bg-{}", config["modes"]["feh-mode"].clone().unwrap()),
    };

    if settings.custom_folder_names == Mode::On {
        if let Some(folder_name) = config["folders"]["daytime-folder-name"].clone() {
            settings.daytime_folder_name = folder_name
        }
    }

    if settings.weather_groups_mode == Mode::On {
        for (folder_name, weather_types) in config["weather-groups"].clone() {
            for weather in weather_types.expect("").split(" ") {
                if default_weather_types.contains(&weather) {
                    settings
                        .weather_groups
                        .entry(folder_name.clone())
                        .and_modify(|types| types.push(weather.to_string()))
                        .or_insert(vec![weather.to_string()]);
                }
            }
        }
    }
    println!("test: {:?}", settings.weather_groups);

    Ok(settings)
}
