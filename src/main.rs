#![allow(dead_code)]
use anyhow::Result;
mod fetch_weather;
mod utils;
use chrono::Local;
use std::{fs, path::PathBuf, process::Command, thread};
use tokio::time;
extern crate ini;

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif"];

#[derive(Debug)]
enum Mode {
    On,
    Off,
}

#[derive(Debug)]
pub struct Settings {
    key: String,
    city: String,
    country: String,
    path: String,
    weather_mode: Mode,
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
            Mode::On => format!("{}/{}/{}", self.path, self.daytime, self.weather),
            Mode::Off => format!("{}/{}", self.path, self.daytime),
        }
    }
    // Set wallpaper
    fn set_wallpaper(&self) -> Result<(), anyhow::Error> {
        println!("setting wallpaper");
        // Set directory path
        let directory_path = PathBuf::from(self.fetch_path());
        // Iterate over directory items
        let image_files: Vec<String> = fs::read_dir(&directory_path)?
            // If reading entry is succesful, we fetch the PathBuf
            .filter_map(|results| results.map(|entry| entry.path()).ok())
            // Check if path is file, not a directory
            .filter(|path| path.is_file())
            // Filter again, check if path extension is an image extension
            .filter_map(|path| {
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .filter(|ext| IMAGE_EXTENSIONS.contains(ext))
                    .and_then(|_| path.display().to_string().into())
            })
            .collect();
        // if image_files isn't empty, run feh command to change wallpaper
        if !image_files.is_empty() {
            let mut command = Command::new("feh");
            command.args([&self.feh_mode, "--randomize"]);
            command.args(&image_files);

            command.output().ok();
        }

        Ok(())
    }
}

async fn daytime_mode_loop(settings: &mut Settings) -> Result<(), anyhow::Error> {
    let mut timer = 0;
    loop {
        if timer == settings.interval {
            match settings.cycle_mode {
                Mode::On => {
                    timer = 0;
                    settings.set_wallpaper().ok();
                }
                Mode::Off => timer = 0,
            }
        }
        let response =
            fetch_weather::openweathermap(&settings.key, &settings.city, &settings.country).await?;
        let daytime = utils::fetch_daytime(
            response.sys.sunrise,
            response.sys.sunset,
            response.timezone,
            Local::now(),
        )?;
        if settings.daytime != daytime {
            settings.daytime = daytime;
            settings.set_wallpaper().ok();
        } else {
            timer += 1;
            thread::sleep(time::Duration::from_secs(60));
            continue;
        }
    }
}

async fn weather_mode_loop(settings: &mut Settings) -> Result<(), anyhow::Error> {
    let mut timer = 0;
    loop {
        if timer == settings.interval {
            match settings.cycle_mode {
                Mode::On => {
                    timer = 0;
                    settings.set_wallpaper().ok();
                }
                Mode::Off => timer = 0,
            }
        }
        let response =
            fetch_weather::openweathermap(&settings.key, &settings.city, &settings.country).await?;
        let current_daytime = utils::fetch_daytime(
            response.sys.sunrise,
            response.sys.sunset,
            response.timezone,
            Local::now(),
        )?;
        let weather = fetch_weather::condition(response.weather[0].id);
        if settings.daytime != current_daytime {
            settings.daytime = current_daytime;
            settings.set_wallpaper().ok();
        } else if settings.weather != weather {
            settings.weather = weather;
            settings.set_wallpaper().ok();
        } else {
            timer += 1;
            thread::sleep(time::Duration::from_secs(60));
            continue;
        }
    }
}

// Find path to home directory for reading config file
fn fetch_config_path() -> Result<PathBuf> {
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

// Read config file and mut settings
fn fetch_config() -> Result<Settings, anyhow::Error> {
    let config_path = fetch_config_path()?;
    let config = ini::ini!(config_path.to_str().expect("test"));

    Ok(Settings {
        key: config["settings"]["key"].clone().unwrap(),
        city: config["settings"]["city"].clone().unwrap(),
        country: config["settings"]["country"].clone().unwrap(),
        path: config["settings"]["path"].clone().unwrap(),
        weather_mode: match config["modes"]["weather-mode"]
            .clone()
            .unwrap()
            .to_lowercase()
            .as_str()
        {
            "on" => Mode::On,
            _ => Mode::Off,
        },
        cycle_mode: match config["modes"]["cycle-mode"]
            .clone()
            .unwrap()
            .to_lowercase()
            .as_str()
        {
            "on" => Mode::On,
            _ => Mode::Off,
        },
        interval: (config["modes"]["interval"]
            .clone()
            .unwrap()
            .parse::<i64>()?
            + 1),
        daytime: String::from("day"),
        weather: String::from("clear"),
        feh_mode: format!("--bg-{}", config["modes"]["feh-mode"].clone().unwrap()),
    })
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut settings: Settings = fetch_config()?;
    // fetch current Open Weather data
    let response =
        fetch_weather::openweathermap(&settings.key, &settings.city, &settings.country).await?;
    println!("response: {:?}", response);
    // fetch current daytime
    settings.daytime = utils::fetch_daytime(
        response.sys.sunrise,
        response.sys.sunset,
        response.timezone,
        Local::now(),
    )?;
    // fetch current weather
    settings.weather = fetch_weather::condition(response.weather[0].id);
    // Set wallpaper
    settings.set_wallpaper().ok();
    // Loop to check for changes
    match settings.weather_mode {
        Mode::On => weather_mode_loop(&mut settings).await,
        Mode::Off => daytime_mode_loop(&mut settings).await,
    }
}

#[test]
fn main_test() {
    let _ = main();
}
