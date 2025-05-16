use crate::fetch_weather::WeatherResponse;
use anyhow::anyhow;
use std::fmt;
mod config;
use anyhow::Result;
use std::collections::HashMap;
mod fetch_weather;
mod utils;
use std::{fs, path::PathBuf, process::Command, thread};
use tokio::time;
extern crate ini;

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif"];

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum WeatherType {
    Clear,
    FewClouds,
    ScatteredClouds,
    BrokenClouds,
    OvercastClouds,
    Drizzle,
    Mist,
    Rain,
    Snow,
    Thunder,
}

impl fmt::Display for WeatherType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeatherType::Clear => write!(f, "clear"),
            WeatherType::ScatteredClouds => write!(f, "scattered clouds"),
            WeatherType::BrokenClouds => write!(f, "broken clouds"),
            WeatherType::FewClouds => write!(f, "few clouds"),
            WeatherType::OvercastClouds => write!(f, "overcast clouds"),
            WeatherType::Drizzle => write!(f, "drizzle"),
            WeatherType::Mist => write!(f, "mist"),
            WeatherType::Rain => write!(f, "rain"),
            WeatherType::Snow => write!(f, "snow"),
            WeatherType::Thunder => write!(f, "thunder"),
        }
    }
}
// Get weather condition from weather id
pub fn condition(id: i32) -> WeatherType {
    match id {
        199..233 => WeatherType::Thunder,
        299..321 => WeatherType::Drizzle,
        499..532 => WeatherType::Rain,
        599..623 => WeatherType::Snow,
        700..781 => WeatherType::Mist,
        801 => WeatherType::FewClouds,
        802 => WeatherType::ScatteredClouds,
        803 => WeatherType::BrokenClouds,
        804 => WeatherType::OvercastClouds,
        _ => WeatherType::Clear,
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Mode {
    CycleMode,
    DaytimeMode,
    WeatherMode,
    SunriseMode,
}
impl Mode {
    fn from_string(mode: &str) -> Result<Mode, anyhow::Error> {
        match mode {
            "cycle-mode" => Ok(Mode::CycleMode),
            "daytime-mode" => Ok(Mode::DaytimeMode),
            "weather-mode" => Ok(Mode::WeatherMode),
            "SunriseMode" => Ok(Mode::SunriseMode),
            _ => Err(anyhow!("unknown mode")),
        }
    }
    fn to_string_vec() -> Vec<String> {
        vec![
            "cycle-mode".to_string(),
            "daytime-mode".to_string(),
            "weather-mode".to_string(),
            "sunrise-mode".to_string(),
        ]
    }
}

#[derive(Eq, Hash, Debug, PartialEq, PartialOrd)]
pub enum Daytime {
    Day,
    Night,
    Sunrise,
    Sunset,
}

impl fmt::Display for Daytime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Daytime::Day => write!(f, "day"),
            Daytime::Night => write!(f, "night"),
            Daytime::Sunrise => write!(f, "Sunrise"),
            Daytime::Sunset => write!(f, "Sunset"),
        }
    }
}

#[derive(Debug)]
pub struct Settings {
    current_loop: bool,
    key: String,
    city: String,
    country: String,
    path: String,
    modes: Vec<Mode>,
    daytime: Daytime,
    disabled_daytimes: Option<Vec<Daytime>>,
    folder_names: HashMap<Daytime, String>,
    custom_weather_groups: bool,
    weather_groups: HashMap<String, Vec<WeatherType>>,
    sunset_time: i32,
    interval: i32,
    weather: WeatherType,
    feh_mode: String,
    timer: i32,
}

impl Settings {
    // Fetch path to wallpaper directory.

    fn fetch_path(&self) -> String {
        match self.modes.contains(&Mode::WeatherMode) {
            true => match self.modes.contains(&Mode::DaytimeMode) {
                true => match &self.disabled_daytimes {
                    Some(daytimes) => match daytimes.contains(&self.daytime) {
                        true => format!("{}{}", self.path, self.fetch_folder_name(),),
                        false => format!(
                            "{}{}/{}",
                            self.path,
                            self.fetch_folder_name(),
                            self.check_group()
                        ),
                    },
                    None => format!("{}{}", self.path, self.fetch_folder_name()),
                },
                false => format!("{}{}", self.path, self.check_group()),
            },
            false => match self.modes.contains(&Mode::DaytimeMode) {
                true => format!("{}{}", self.path, self.fetch_folder_name()),
                false => self.path.clone(),
            },
        }
    }

    // Fetch correct folder name.
    fn fetch_folder_name(&self) -> String {
        self.folder_names.get(&self.daytime).unwrap().to_string()
    }

    // Check if current weather type is in a custom weather group. Also checks if weather mode is
    // turned off for current daytime.
    fn check_group(&self) -> String {
        if self.custom_weather_groups {
            self.weather_groups
                .iter()
                .find(|(_, weather_list)| weather_list.contains(&self.weather))
                .map_or_else(|| self.weather.to_string(), |(group, _)| group.to_string())
        } else {
            self.weather.to_string()
        }
    }

    // If cycle mode is on, check if timer has reached limit.
    fn check_cycle_mode(&mut self) {
        if self.timer == self.interval {
            self.timer = 0;
            println!("timer detected");
            self.current_loop = true;
        } else {
            self.timer += 1;
        }
    }

    // If weather mode is on, check if weather has changed.
    fn check_weather_mode(&mut self, response: &WeatherResponse) {
        println!("checking weather mode");
        let weather = condition(response.weather[0].id);
        if self.weather != weather {
            println!("weather detected");
            self.weather = weather;
            self.current_loop = true;
        }
    }

    // If daytime mode is on, check if daytime has changed
    fn check_daytime_mode(&mut self, response: &WeatherResponse) {
        let daytime = utils::fetch_daytime(
            response.sys.sunrise,
            response.sys.sunset,
            &self.modes,
            self.sunset_time,
        );
        if self.daytime != daytime {
            self.daytime = daytime;
            println!("daytime detected");
            self.current_loop = true;
        }
    }

    // Set wallpaper
    fn set_wallpaper(&self) -> Result<(), anyhow::Error> {
        println!("setting wallpaper");
        let directory_path = PathBuf::from(self.fetch_path());
        println!("path is: {:?}", directory_path);

        match fs::read_dir(&directory_path) {
            Ok(entries) => {
                let image_files: Vec<String> = entries
                    .filter_map(|entry| entry.ok().map(|e| e.path()))
                    .filter(|path| path.is_file())
                    .filter_map(|path| {
                        path.extension()
                            .and_then(|ext| ext.to_str())
                            .filter(|ext| IMAGE_EXTENSIONS.contains(ext))
                            .map(|_| path.display().to_string())
                    })
                    .collect();

                if !image_files.is_empty() {
                    let mut command = Command::new("feh");
                    command.args([&self.feh_mode, "--randomize"]);
                    command.args(&image_files);
                    command.output().ok();
                } else {
                    println!("no images found in directory")
                }
            }
            Err(e) => {
                println!("No directory found for: {:?} {:?}", directory_path, e);
            }
        }
        Ok(())
    }
}

async fn wallpaper_manager_loop(settings: &mut Settings) -> Result<(), anyhow::Error> {
    loop {
        println!("looping");
        // If cycle mode is on, change wallpaper if interval is reached.
        if settings.modes.contains(&Mode::CycleMode) {
            settings.check_cycle_mode()
        }
        // Only fetch weather data when weather or day mode is on.
        if settings.modes.contains(&Mode::DaytimeMode)
            || settings.modes.contains(&Mode::WeatherMode)
        {
            // fetch weather data
            let response =
                fetch_weather::openweathermap(&settings.key, &settings.city, &settings.country)
                    .await?;
            // If daytime mode is on, change wallpaper on sunrise and sunset.
            if settings.modes.contains(&Mode::DaytimeMode) {
                settings.check_daytime_mode(&response);
            }
            // If weather mode is on, change wallpaper when weather changes.
            if settings.modes.contains(&Mode::WeatherMode) {
                settings.check_weather_mode(&response);
            }
        }
        // If a change has been detected in any of the modes, change wallpaper.
        if settings.current_loop {
            settings.set_wallpaper().ok();
            settings.current_loop = false
        }
        thread::sleep(time::Duration::from_secs(60))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load configuration
    let mut settings: Settings = config::fetch_config()?;
    // Start loop
    println!("configuration: {:?} ", settings);
    wallpaper_manager_loop(&mut settings).await
}

#[test]
fn main_test() {
    let _ = main();
}
