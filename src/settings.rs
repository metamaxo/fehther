use crate::Command;
use crate::PathBuf;
use crate::WeatherResponse;
use crate::WeatherType;
use crate::fs;
use crate::types::daytime::Daytime;
use crate::types::default_types::IMAGE_EXTENSIONS;
use crate::types::modes::Mode;
use crate::utils;
use std::collections::HashMap;

// All possible settings, parsed from config.ini.
#[derive(Debug)]
pub struct Settings {
    pub current_loop: bool,
    pub key: String,
    pub city: String,
    pub country: String,
    pub path: String,
    pub modes: Vec<Mode>,
    pub daytime: Daytime,
    pub golden_hour: bool,
    pub disabled_daytimes: Option<Vec<Daytime>>,
    pub folder_names: HashMap<Daytime, String>,
    pub custom_weather_groups: bool,
    pub weather_groups: HashMap<String, Vec<WeatherType>>,
    pub sunset_timer: i32,
    pub interval: i32,
    pub weather: WeatherType,
    pub feh_mode: String,
    pub timer: i32,
    pub recovery_mode: bool,
}

// Full configuration is stored in the Settings struct and called through traits.
impl Settings {
    // check if the current daytime is explicitly disabled for weather mode
    fn is_current_daytime_disabled_for_weather_mode(&self) -> bool {
        self.disabled_daytimes
            .as_ref()
            .is_some_and(|daytimes| daytimes.contains(&self.daytime))
    }

    // Fetch path to wallpaper directory.
    pub fn fetch_path(&self) -> String {
        // If in recovery mode, return only the base path
        if self.recovery_mode {
            return self.path.clone();
        }

        let is_weather_mode_on = self.modes.contains(&Mode::Weather);
        let is_daytime_mode_on = self.modes.contains(&Mode::Daytime);
        let is_daytime_disabled_for_weather = self.is_current_daytime_disabled_for_weather_mode();

        let mut final_path = self.path.clone();

        // Append daytime folder if daytime mode is on
        if is_daytime_mode_on {
            final_path.push_str(&self.fetch_folder_name());
        }

        // Append weather group if weather mode is on AND
        // (daytime mode is off OR current daytime is NOT disabled for weather)
        if is_weather_mode_on && !(is_daytime_mode_on && is_daytime_disabled_for_weather) {
            // Add a separator if a previous segment (daytime folder) was added
            if is_daytime_mode_on {
                final_path.push('/');
            }
            final_path.push_str(&self.check_group());
        }

        final_path
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
            self.golden_hour,
            self.sunset_timer,
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
