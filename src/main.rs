#![allow(dead_code)]
mod config;
use anyhow::Result;
use std::collections::HashMap;
mod fetch_weather;
mod utils;
use std::{default, fs, path::PathBuf, process::Command, thread};
use tokio::time;
extern crate ini;

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif"];

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Mode {
    On,
    Off,
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
    weather_mode: Mode,
    weather_groups_mode: Mode,
    weather_groups: HashMap<String, Vec<String>>,
    cycle_mode: Mode,
    interval: i64,
    daytime: String,
    weather: String,
    feh_mode: String,
}

impl Settings {
    // fetch path to wallpaper directory
    fn fetch_path(&self) -> String {
        match self.weather_mode {
            Mode::On => match self.daytime_mode {
                Mode::On => format!("{}{}/{}", self.path, self.daytime, self.check_group()),
                Mode::Off => format!("{}{}", self.path, self.check_group()),
            },
            Mode::Off => match self.daytime_mode {
                Mode::On => format!("{}{}", self.path, self.daytime),
                Mode::Off => self.path.clone(),
            },
        }
    }
    fn check_group(&self) -> String {
        //  Use `find` to locate the first group matching the condition.
        if let Some((group, _)) = self
            .weather_groups
            .iter()
            .find(|(_, weather_list)| weather_list.contains(&self.weather))
        {
            group.clone() // Return a cloned String, consistent with original code
        } else {
            self.weather.clone() // Return the original weather if no group matches
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
    let mut timer = 0;
    loop {
        // If cycle mode is on, change wallpaper if interval is reached.
        println!("checking cycle_mode");
        if settings.cycle_mode == Mode::On {
            if timer == settings.interval {
                timer = 0;
                println!("timer detected");
                settings.current_loop = Mode::On
            } else {
                timer += 1
            }
        }
        if settings.daytime_mode == Mode::On || settings.weather_mode == Mode::On {
            // fetch weather data
            let response =
                fetch_weather::openweathermap(&settings.key, &settings.city, &settings.country)
                    .await?;

            println!("checking daytime mode");
            // If daytime mode is on, change wallpaper on sunrise and sunset.
            if let Mode::On = settings.daytime_mode {
                let daytime = utils::fetch_daytime(response.sys.sunrise, response.sys.sunset);
                if settings.daytime != daytime {
                    settings.daytime = daytime;
                    println!("daytime detected");
                    settings.current_loop = Mode::On
                }
            }

            println!("checking weather mode");
            // If weather mode is on, change wallpaper when weather changes.
            if let Mode::On = settings.weather_mode {
                let weather = fetch_weather::condition(response.weather[0].id);
                if settings.weather != weather {
                    println!("weather detected");
                    settings.weather = weather;
                    settings.current_loop = Mode::On
                }
            }
        }
        if settings.current_loop == Mode::On {
            settings.set_wallpaper().ok();
            settings.current_loop = Mode::Off
        }
        println!("sleeping");
        thread::sleep(time::Duration::from_secs(60))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load configuration
    let mut settings: Settings = config::fetch_config()?;
    println!("test: {:?}", settings.daytime_folder_name);
    // Start loop
    wallpaper_manager_loop(&mut settings).await
}

#[test]
fn main_test() {
    let _ = main();
}
