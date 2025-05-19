use crate::Mode;
use crate::PathBuf;
use crate::Settings;
use crate::WeatherType;
use crate::types::Daytime;
use anyhow::anyhow;
use anyhow::{Context, Result};
use ini::Ini;
use std::collections::HashMap;

// get interval and add 1 minute.
fn get_interval(interval: &str) -> i32 {
    interval.parse::<i32>().unwrap() + 1
}

// Find path to home directory for reading config file
pub fn fetch_config_path() -> Result<PathBuf> {
    Ok(home::home_dir()
        .ok_or_else(|| anyhow!("Could not determine user's home directory"))?
        .join(".config")
        .join("fehther")
        .join("config.ini"))
}

// Read config file and load configuration
pub fn fetch_config(config_path: PathBuf) -> Result<Settings> {
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
    // Should panic if no path is found
    let path: Result<String, anyhow::Error> = config
        .get_from(Some("settings"), "path")
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("no path to wallpaper folder"));

    let path = path?;

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
                    .filter_map(|weather| WeatherType::get_weathertype(weather).ok())
                    .collect();
                if !weather_types.is_empty() {
                    weather_groups.insert(folder_name.to_string(), weather_types);
                }
            }
        }
    }

    let sunset_time = get_interval(
        config
            .get_from(Some("modes"), "golden-hour-time")
            .unwrap_or("60"),
    );
    let interval = get_interval(
        config
            .get_from(Some("modes"), "cycle-timer")
            .unwrap_or("60"),
    );

    let mut disabled_daytimes: Vec<Daytime> = Vec::new();

    for daytime in config
        .get_from(Some("modes"), "disabled-daytime-modes")
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
        recovery_mode: false,
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

#[test]
// testing normal config
fn fetch_config_test_1() -> Result<()> {
    let config_path = PathBuf::from(r"./test_config/test_1.ini");
    let settings: Settings = fetch_config(config_path)?;

    let expected_disabled_daytimes = Some(vec![Daytime::Sunset, Daytime::Sunrise]);

    let mut expected_weather_groups = HashMap::new();
    expected_weather_groups.insert(
        "rainy".to_string(),
        vec![WeatherType::Drizzle, WeatherType::Rain],
    );
    expected_weather_groups.insert(
        "very-cloudy".to_string(),
        vec![WeatherType::BrokenClouds, WeatherType::OvercastClouds],
    );
    expected_weather_groups.insert(
        "slightly-cloudy".to_string(),
        vec![WeatherType::FewClouds, WeatherType::ScatteredClouds],
    );
    expected_weather_groups.insert("clear".to_string(), vec![WeatherType::Clear]);

    let mut expected_folder_names = HashMap::new();
    expected_folder_names.insert(Daytime::Sunrise, "moo".to_string());
    expected_folder_names.insert(Daytime::Day, "foo".to_string());
    expected_folder_names.insert(Daytime::Sunset, "woo".to_string());
    expected_folder_names.insert(Daytime::Night, "boo".to_string());
    assert_eq!(
        settings.modes,
        vec![
            Mode::DaytimeMode,
            Mode::GoldenHourMode,
            Mode::WeatherMode,
            Mode::CycleMode
        ]
    );
    assert_eq!(settings.disabled_daytimes, expected_disabled_daytimes);
    assert_eq!(settings.weather_groups, expected_weather_groups);
    assert_eq!(settings.folder_names, expected_folder_names);
    assert_eq!(settings.key, "fake-key".to_string());
    assert_eq!(settings.city, "london".to_string());
    assert_eq!(settings.country, "UK".to_string());
    assert_eq!(settings.path, "/home/user/files/documents/wallpapers");

    Ok(())
}
#[test]
// testing config with missing parts
fn fetch_config_test_2() -> Result<()> {
    let config_path = PathBuf::from(r"./test_config/test_2.ini");
    let settings: Settings = fetch_config(config_path)?;

    let expected_disabled_daytimes = Some(vec![]);
    let expected_modes = vec![Mode::CycleMode];

    let expected_weather_groups = HashMap::new();
    let mut expected_folder_names = HashMap::new();
    expected_folder_names.insert(Daytime::Sunrise, "sunrise".to_string());
    expected_folder_names.insert(Daytime::Day, "day".to_string());
    expected_folder_names.insert(Daytime::Sunset, "sunset".to_string());
    expected_folder_names.insert(Daytime::Night, "night".to_string());

    assert_eq!(settings.feh_mode, "--bg-fill".to_string());
    assert_eq!(settings.sunset_time, 61);
    assert_eq!(settings.disabled_daytimes, expected_disabled_daytimes);
    assert_eq!(settings.modes, expected_modes);
    assert_eq!(settings.weather_groups, expected_weather_groups);
    assert_eq!(settings.folder_names, expected_folder_names);

    Ok(())
}
#[test]
// testing config with custom folder names turned off, but custom folder names in condig
fn fetch_config_test_3() -> Result<()> {
    let config_path = PathBuf::from(r"./test_config/test_3.ini");
    let settings: Settings = fetch_config(config_path)?;

    let expected_disabled_daytimes = Some(vec![]);
    let expected_modes = vec![];

    let expected_weather_groups = HashMap::new();
    let mut expected_folder_names = HashMap::new();
    expected_folder_names.insert(Daytime::Sunrise, "sunrise".to_string());
    expected_folder_names.insert(Daytime::Day, "day".to_string());
    expected_folder_names.insert(Daytime::Sunset, "sunset".to_string());
    expected_folder_names.insert(Daytime::Night, "night".to_string());

    assert_eq!(settings.feh_mode, "--bg-fill".to_string());
    assert_eq!(settings.sunset_time, 61);
    assert_eq!(settings.disabled_daytimes, expected_disabled_daytimes);
    assert_eq!(settings.modes, expected_modes);
    assert_eq!(settings.weather_groups, expected_weather_groups);
    assert_eq!(settings.folder_names, expected_folder_names);

    Ok(())
}

#[test]
// testing with empty config, should panic
fn fetch_config_test_4() -> Result<()> {
    let config_path = PathBuf::from(r"./test_config/test_4.ini");
    let result = fetch_config(config_path);

    // Assert that the result is an error using `is_err()`
    if result.is_ok() {
        panic!("fetch_config should have returned an error, but it returned Ok");
    }
    Ok(())
}
