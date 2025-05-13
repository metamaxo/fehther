use crate::fetch_weather::WeatherResponse;
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

#[derive(Debug, PartialEq, PartialOrd)]
pub enum WeatherType {
    Clear,
    Clouds,
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
            WeatherType::Clouds => write!(f, "clouds"),
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
        800 => WeatherType::Clear,
        _ => WeatherType::Clouds,
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Mode {
    On,
    Off,
}

#[derive(Debug, PartialEq, PartialOrd)]
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
    current_loop: Mode,
    key: String,
    city: String,
    country: String,
    path: String,
    daytime_mode: Mode,
    custom_folder_names: Mode,
    daytime_folder_name: String,
    nighttime_folder_name: String,
    sunrise_folder_name: String,
    sunset_folder_name: String,
    weather_mode: Mode,
    weather_groups_mode: Mode,
    weather_groups: HashMap<String, Vec<WeatherType>>,
    daytime_weather_mode: Mode,
    nighttime_weather_mode: Mode,
    sunrise_weather_mode: Mode,
    sunset_weather_mode: Mode,
    cycle_mode: Mode,
    sunset_mode: Mode,
    sunset_time: i32,
    interval: i32,
    daytime: Daytime,
    weather: WeatherType,
    feh_mode: String,
    timer: i32,
}

impl Settings {
    // Fetch path to wallpaper directory.
    fn fetch_path(&self) -> String {
        match self.weather_mode {
            Mode::On => match self.daytime_mode {
                Mode::On => match self.check_conditionals() {
                    Mode::On => format!(
                        "{}{}/{}",
                        self.path,
                        self.fetch_folder_name(),
                        self.check_group()
                    ),
                    Mode::Off => format!("{}{}", self.path, self.fetch_folder_name()),
                },
                Mode::Off => format!("{}{}", self.path, self.check_group()),
            },
            Mode::Off => match self.daytime_mode {
                Mode::On => format!("{}{}", self.path, self.fetch_folder_name()),
                Mode::Off => self.path.clone(),
            },
        }
    }

    // Fetch correct folder name.
    fn fetch_folder_name(&self) -> &str {
        match self.daytime {
            Daytime::Day => &self.daytime_folder_name,
            Daytime::Night => &self.nighttime_folder_name,
            Daytime::Sunrise => &self.sunrise_folder_name,
            Daytime::Sunset => &self.sunset_folder_name,
        }
    }

    // Check if current weather type is in a custom weather group. Also checks if weather mode is
    // turned off for current daytime.
    fn check_group(&self) -> String {
        match self.weather_groups_mode {
            Mode::On => {
                if let Some((group, _)) = self
                    .weather_groups
                    .iter()
                    .find(|(_, weather_list)| weather_list.contains(&self.weather))
                {
                    group.to_string()
                } else {
                    self.weather.to_string()
                }
            }
            Mode::Off => self.weather.to_string(),
        }
    }

    // Check if weather mode is turned off for current daytime.
    fn check_conditionals(&self) -> Mode {
        match self.daytime {
            Daytime::Day => match self.daytime_weather_mode {
                Mode::Off => Mode::Off,
                Mode::On => Mode::On,
            },
            Daytime::Night => match self.nighttime_weather_mode {
                Mode::Off => Mode::Off,
                Mode::On => Mode::On,
            },
            Daytime::Sunrise => match self.sunrise_weather_mode {
                Mode::On => Mode::On,
                Mode::Off => Mode::Off,
            },
            Daytime::Sunset => match self.sunset_weather_mode {
                Mode::On => Mode::On,
                Mode::Off => Mode::Off,
            },
        }
    }

    // If cycle mode is on, check if timer has reached limit.
    fn check_cycle_mode(&mut self) {
        if self.timer == self.interval {
            self.timer = 0;
            println!("timer detected");
            self.current_loop = Mode::On
        } else {
            self.timer += 1
        }
    }

    // If weather mode is on, check if weather has changed.
    fn check_weather_mode(&mut self, response: &WeatherResponse) {
        println!("checking weather mode");
        let weather = condition(response.weather[0].id);
        if self.weather != weather {
            println!("weather detected");
            self.weather = weather;
            self.current_loop = Mode::On
        }
    }

    // If daytime mode is on, check if daytime has changed
    fn check_daytime_mode(&mut self, response: &WeatherResponse) {
        let daytime = utils::fetch_daytime(
            response.sys.sunrise,
            response.sys.sunset,
            &self.sunset_mode,
            self.sunset_time,
        );
        if self.daytime != daytime {
            self.daytime = daytime;
            println!("daytime detected");
            self.current_loop = Mode::On
        }
    }

    // Set wallpaper
    fn set_wallpaper(&self) -> Result<(), anyhow::Error> {
        println!("setting wallpaper");
        // Set directory path
        let directory_path = PathBuf::from(self.fetch_path());
        println!("path is: {:?}", directory_path);

        // Attempt to read the directory
        match fs::read_dir(&directory_path) {
            Ok(entries) => {
                let image_files: Vec<String> = entries
                    // If reading entry was succcesful, we fetch the path
                    .filter_map(|results| results.map(|entry| entry.path()).ok())
                    // Check if path is file and not a directory
                    .filter(|path| path.is_file())
                    // Check if file extension is compatible.
                    .filter_map(|path| {
                        path.extension()
                            .and_then(|ext| ext.to_str())
                            .filter(|ext| IMAGE_EXTENSIONS.contains(ext))
                            .and_then(|_| path.display().to_string().into())
                    })
                    .collect();

                // If image_files isn't empty, run feh command to change wallpaper
                if !image_files.is_empty() {
                    let mut command = Command::new("feh");
                    command.args([&self.feh_mode, "--randomize"]);
                    command.args(&image_files);
                    command.output().ok();
                } else {
                    println!("no images found in directory")
                    // If no images are found in directory, wallpaper remains unchanged.
                }
            }
            // If no directory is found for the selected mode, wallpaper remains unchanged.
            Err(_) => {
                println!("No directory found for: {:?}", directory_path);
            }
        }

        Ok(())
    }
}

async fn wallpaper_manager_loop(settings: &mut Settings) -> Result<(), anyhow::Error> {
    loop {
        println!("looping");
        // If cycle mode is on, change wallpaper if interval is reached.
        if settings.cycle_mode == Mode::On {
            settings.check_cycle_mode()
        }
        // Only fetch weather data when weather or day mode is on.
        if settings.daytime_mode == Mode::On || settings.weather_mode == Mode::On {
            // fetch weather data
            let response =
                fetch_weather::openweathermap(&settings.key, &settings.city, &settings.country)
                    .await?;
            // If daytime mode is on, change wallpaper on sunrise and sunset.
            if settings.daytime_mode == Mode::On {
                settings.check_daytime_mode(&response);
            }
            // If weather mode is on, change wallpaper when weather changes.
            if settings.weather_mode == Mode::On {
                settings.check_weather_mode(&response);
            }
        }
        // If a change has been detected in any of the modes, change wallpaper.
        if settings.current_loop == Mode::On {
            settings.set_wallpaper().ok();
            settings.current_loop = Mode::Off
        }
        thread::sleep(time::Duration::from_secs(60))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load configuration
    let mut settings: Settings = config::fetch_config()?;
    // Start loop
    wallpaper_manager_loop(&mut settings).await
}

#[test]
fn main_test() {
    let _ = main();
}
