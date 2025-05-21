use crate::Mode;
use crate::PathBuf;
use crate::Settings;
use crate::WeatherType;
use crate::types::daytime::Daytime;

use anyhow::anyhow;
use anyhow::{Context, Result};

use ini::Ini;
use std::collections::HashMap;

// get interval and add 1 minute.
fn fetch_timer(interval: &str) -> i32 {
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

// Splits a string into whitespace and tries to find daytimes.
fn fetch_daytimes(config: &Ini) -> Vec<Daytime> {
    config
        .get_from(Some("modes"), "disabled-daytime-modes")
        .unwrap_or("")
        .to_lowercase()
        .split_whitespace()
        .map(|daytime| match daytime {
            "sunrise" => Daytime::Sunrise,
            "day" => Daytime::Day,
            "sunset" => Daytime::Sunset,
            _ => Daytime::Night,
        })
        .collect()
}

// Gets the folder names, if custom folder names is true, folder names get updated.
fn fetch_folder_names(config: &Ini, custom_folder_names: bool) -> HashMap<Daytime, String> {
    // Default folder names.
    let mut folder_names_map = HashMap::from([
        (Daytime::Day, "day".to_string()),
        (Daytime::Night, "night".to_string()),
        (Daytime::Sunrise, "sunrise".to_string()),
        (Daytime::Sunset, "sunset".to_string()),
    ]);

    if custom_folder_names {
        //  custom folder names
        let mut update_folder_name = |daytime: Daytime, key: &str| {
            if let Some(folder_name) = config.get_from(Some("folders"), key) {
                folder_names_map.insert(daytime, folder_name.to_string());
            }
        };

        // Update the map with custom values, if provided.
        update_folder_name(Daytime::Day, "daytime-folder-name");
        update_folder_name(Daytime::Sunset, "sunset-folder-name");
        update_folder_name(Daytime::Sunrise, "sunrise-folder-name");
        update_folder_name(Daytime::Night, "nighttime-folder-name");
    }

    folder_names_map
}

// Check for custom weather groups in config.ini
fn fetch_weather_groups(
    config: &Ini,
    custom_weather_groups: bool,
) -> HashMap<String, Vec<WeatherType>> {
    let mut weather_groups = HashMap::new();
    if custom_weather_groups {
        if let Some(weather_groups_section) = config.section(Some("weather-groups")) {
            for (folder_name, weather_types_str) in weather_groups_section.iter() {
                let weather_types: Vec<WeatherType> = weather_types_str
                    .split_whitespace()
                    .filter_map(|weather| WeatherType::get_weathertype(weather).ok())
                    .collect();
                if !weather_types.is_empty() {
                    weather_groups.insert(folder_name.to_string(), weather_types);
                }
            }
        }
    }
    weather_groups
}

fn fetch_modes(config: &Ini) -> Vec<Mode> {
    config
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
                .collect::<Vec<Mode>>()
        })
        .unwrap_or_default()
}

// Read config file and load configuration
pub fn fetch_config(config_path: PathBuf) -> Result<Settings> {
    // Load the config file, located in ~/.config
    let config = Ini::load_from_file(&config_path)
        .with_context(|| format!("Failed to load config file: {}", config_path.display()))?;
    // Parse config
    // Fetch API key
    let key = config
        .get_from(Some("settings"), "key")
        .unwrap_or_default()
        .to_string();
    // Fetch city
    let city = config
        .get_from(Some("settings"), "city")
        .unwrap_or_default()
        .to_string();
    // Fetch countru
    let country = config
        .get_from(Some("settings"), "country")
        .unwrap_or_default()
        .to_string();
    // Fetch path
    // Should panic if no path is found
    let path: Result<String, anyhow::Error> = config
        .get_from(Some("settings"), "path")
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("no path to wallpaper folder"));
    let path = path?;
    // Fetch modes
    let modes = fetch_modes(&config);
    // Fetch golden hour(bool)
    let golden_hour = config
        .get_from(Some("modes"), "golden-hour-mode")
        .unwrap_or("false")
        == "true";
    // Fetch disabled_daytimes
    let disabled_daytimes = Some(fetch_daytimes(&config));
    // Fetch custom folder names(bool)
    let custom_folder_names = config
        .get_from(Some("folders"), "custom-folder-names")
        .unwrap_or("false")
        == "true";
    // Fetch custom weather groups(bool)
    let custom_weather_groups = config
        .get_from(Some("weather-groups"), "weather-groups")
        .unwrap_or("false")
        == "true";
    // Fetch folder names
    let folder_names = fetch_folder_names(&config, custom_folder_names);
    // Fetch weather groups
    let weather_groups = fetch_weather_groups(&config, custom_weather_groups);
    // Fetch sunset timer
    let sunset_timer = fetch_timer(
        config
            .get_from(Some("modes"), "golden-hour-timer")
            .unwrap_or("60"),
    );
    // Fetch cycle mode timer
    let interval = fetch_timer(
        config
            .get_from(Some("modes"), "cycle-timer")
            .unwrap_or("60"),
    );
    // Create feh command string for desired mode.
    let feh_mode = format!(
        "--bg-{}",
        config.get_from(Some("modes"), "feh-mode").unwrap_or("fill")
    );

    // Load config into Settings
    Ok(Settings {
        current_loop: true,
        key,
        city,
        country,
        path,
        modes,
        recovery_mode: false,
        disabled_daytimes,
        custom_weather_groups,
        folder_names,
        weather_groups,
        sunset_timer,
        golden_hour,
        interval,
        daytime: Daytime::Day,
        weather: WeatherType::Clear,
        feh_mode,
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
        vec![Mode::Daytime, Mode::Weather, Mode::Cycle]
    );
    assert!(settings.golden_hour);
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
    let expected_modes = vec![Mode::Cycle];

    let expected_weather_groups = HashMap::new();
    let mut expected_folder_names = HashMap::new();
    expected_folder_names.insert(Daytime::Sunrise, "sunrise".to_string());
    expected_folder_names.insert(Daytime::Day, "day".to_string());
    expected_folder_names.insert(Daytime::Sunset, "sunset".to_string());
    expected_folder_names.insert(Daytime::Night, "night".to_string());

    assert!(!settings.golden_hour);
    assert_eq!(settings.feh_mode, "--bg-fill".to_string());
    assert_eq!(settings.sunset_timer, 61);
    assert_eq!(settings.disabled_daytimes, expected_disabled_daytimes);
    assert_eq!(settings.modes, expected_modes);
    assert_eq!(settings.weather_groups, expected_weather_groups);
    assert_eq!(settings.folder_names, expected_folder_names);

    Ok(())
}

#[test]
// testing config with custom folder names turned off, but custom folder names in config
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
    assert_eq!(settings.sunset_timer, 61);
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
