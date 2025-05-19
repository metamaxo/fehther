use crate::Command;
use crate::PathBuf;
use crate::WeatherResponse;
use crate::WeatherType;
use crate::fs;
use crate::types::Daytime;
use crate::types::Mode;
use crate::utils;
use std::collections::HashMap;

// Image types for finding image files
const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif"];

// Full configuration
#[derive(Debug)]
pub struct Settings {
    pub current_loop: bool,
    pub key: String,
    pub city: String,
    pub country: String,
    pub path: String,
    pub modes: Vec<Mode>,
    pub daytime: Daytime,
    pub disabled_daytimes: Option<Vec<Daytime>>,
    pub folder_names: HashMap<Daytime, String>,
    pub custom_weather_groups: bool,
    pub weather_groups: HashMap<String, Vec<WeatherType>>,
    pub sunset_time: i32,
    pub interval: i32,
    pub weather: WeatherType,
    pub feh_mode: String,
    pub timer: i32,
    pub recovery_mode: bool,
}

// Full configuration is stored in the Settings struct and called through traits.
impl Settings {
    // Fetch path to wallpaper directory.
    pub fn fetch_path(&self) -> String {
        // Check if currently in recovery mode(no internet connection)
        match self.recovery_mode {
            // If recovery mode = true, path is only root path.
            true => self.path.to_string(),
            // Check if weather mode is turned on
            false => match self.modes.contains(&Mode::WeatherMode) {
                // If weather mode is turned on, check if daytme mode is turned on.
                true => match self.modes.contains(&Mode::DaytimeMode) {
                    // If daytime mode is turned on, check if any daytimes are disabled.
                    true => match &self.disabled_daytimes {
                        // If daytimes are disabled, check if current daytime is disabled for
                        // weather mode.
                        Some(daytimes) => match daytimes.contains(&self.daytime) {
                            // If current daytime is disabled for weather mode, return path +
                            // daytime.
                            true => format!("{}{}", self.path, self.fetch_folder_name(),),
                            // if current daytime isn't disabled for weather mode, return path +
                            // daytime + weather group name.
                            false => format!(
                                "{}{}/{}",
                                self.path,
                                self.fetch_folder_name(),
                                self.check_group()
                            ),
                        },
                        // If there are no disabled daytimes for weather mode, return path + daytime
                        // + weather group name.
                        None => format!(
                            "{}{}/{}",
                            self.path,
                            self.fetch_folder_name(),
                            self.check_group(),
                        ),
                    },
                    // If daytime mode is disabled, return only path and weather group.
                    false => format!("{}{}", self.path, self.check_group()),
                },
                // Weather mode turned off, check if daytime mode turned on. Return only path if
                // daytime mode is turned off, else return path + daytime folder name.
                false => match self.modes.contains(&Mode::DaytimeMode) {
                    true => format!("{}{}", self.path, self.fetch_folder_name()),
                    false => self.path.to_string(),
                },
            },
        }
    }

    // Fetch correct folder name.
    fn fetch_folder_name(&self) -> String {
        self.folder_names.get(&self.daytime).unwrap().to_string()
    }

    // Check if current weather type is in a custom weather group.
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
    pub fn check_cycle_mode(&mut self) {
        if self.timer == self.interval {
            self.timer = 0;
            self.current_loop = true;
        } else {
            self.timer += 1;
        }
    }

    // If weather mode is on, check if weather has changed.
    pub fn check_weather_mode(&mut self, response: &WeatherResponse) {
        let weather = WeatherType::condition(response.weather[0].id);
        if self.weather != weather {
            self.weather = weather;
            self.current_loop = true;
        }
    }

    // If daytime mode is on, check if daytime has changed
    pub fn check_daytime_mode(&mut self, response: &WeatherResponse) {
        let daytime = utils::fetch_daytime(
            response.sys.sunrise,
            response.sys.sunset,
            &self.modes,
            self.sunset_time,
        );
        if self.daytime != daytime {
            self.daytime = daytime;
            self.current_loop = true;
        }
    }

    // Set wallpaper
    pub fn set_wallpaper(&self) -> Result<(), anyhow::Error> {
        let directory_path = PathBuf::from(self.fetch_path());

        if let Ok(entries) = fs::read_dir(&directory_path) {
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
            }
        }
        Ok(())
    }
}
