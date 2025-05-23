use crate::fetch_weather::WeatherResponse;
use crate::settings::Settings;
use crate::types::modes::Mode;
use crate::types::weathertype::WeatherType;

use anyhow::Result;
use std::fmt;
use std::{fs, path::PathBuf, process::Command, thread};
use tokio::time;

mod config;
mod fetch_weather;
mod settings;
mod types;
mod utils;

// Main loop
async fn wallpaper_manager_loop(settings: &mut Settings) -> Result<(), anyhow::Error> {
    loop {
        // If cycle mode is on, change wallpaper if interval is reached.
        if settings.modes.contains(&Mode::Cycle) {
            settings.check_cycle_mode()
        }
        // Only fetch weather data when weather or day mode is on.
        if settings.modes.contains(&Mode::Daytime) || settings.modes.contains(&Mode::Weather) {
            // fetch weather data, if request fails, fallback to recovery loop.
            match fetch_weather::openweathermap(&settings.key, &settings.city, &settings.country)
                .await
            {
                Ok(response) => {
                    if settings.recovery_mode {
                        settings.current_loop = true;
                        settings.recovery_mode = false
                    }
                    // If daytime mode is on, change wallpaper on sunrise and sunset.
                    if settings.modes.contains(&Mode::Daytime) {
                        settings.check_daytime_mode(&response);
                    }
                    // If weather mode is on, change wallpaper when weather changes.
                    if settings.modes.contains(&Mode::Weather) {
                        settings.check_weather_mode(&response);
                    }
                }
                Err(_) => settings.recovery_mode = true,
            }
        }
        // If a change has been detected in any of the modes, change wallpaper.
        if settings.current_loop {
            settings.set_wallpaper().ok();
            settings.current_loop = false
        }
        // sleep for a minute
        thread::sleep(time::Duration::from_secs(60))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config_path = config::fetch_config_path()?;
    // Load configuration
    let mut settings: Settings = config::fetch_config(config_path)?;
    // Start loop
    wallpaper_manager_loop(&mut settings).await
}
