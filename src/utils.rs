use crate::Daytime;
use crate::Mode;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

// Use sunset and sunrise data to find the current day time. If sunset mode is on, function will
// detect sunrise and sunset if current time is within configured sunset time limit.
pub fn fetch_daytime(sunrise: i32, sunset: i32, mode: &Mode, sunset_limit: i32) -> Daytime {
    // get current time
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32;
    // find day time
    match mode {
        Mode::On => {
            let sunset_secs = sunset_limit * 60;
            if sunrise <= current_time && current_time < sunset {
                if current_time < sunrise + sunset_secs {
                    Daytime::Sunrise
                } else if current_time >= sunset - sunset_secs {
                    Daytime::Sunset
                } else {
                    Daytime::Day
                }
            } else {
                Daytime::Night
            }
        }
        Mode::Off => {
            if sunrise <= current_time && current_time <= sunset {
                Daytime::Day
            } else {
                Daytime::Night
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // identical function for testing

    pub fn test_fetch_daytime(
        current_time: i64,
        sunrise: i64,
        sunset: i64,
        mode: &Mode,
        sunset_time: i64,
    ) -> Daytime {
        // Fetch current time
        match mode {
            Mode::On => {
                // return night, day, sunrise or sunset based on sunset time. Sunset time is the time
                // after sunrise and before sunset that marks 'golden hour'
                let sunset_secs = sunset_time * 60;
                if sunrise <= current_time
                    && current_time < sunset
                    && current_time < sunrise + sunset_secs
                {
                    Daytime::Sunrise
                } else if sunrise <= current_time
                    && current_time < sunset
                    && current_time >= sunrise + sunset_secs
                    && current_time < sunset - sunset_secs
                {
                    Daytime::Day
                } else if sunrise <= current_time
                    && current_time < sunset
                    && current_time >= sunset - sunset_secs
                {
                    Daytime::Sunset
                } else {
                    Daytime::Night
                }
            }
            Mode::Off => {
                // Return night or day
                if sunrise <= current_time && current_time <= sunset {
                    Daytime::Day
                } else {
                    Daytime::Night
                }
            }
        }
    }
    #[test]
    fn test_fetch_daytime_mode_on() {
        // Sunrise and sunset times
        let sunrise: i64 = 1747021974;
        let sunset: i64 = 1747077771;
        let sunset_time: i64 = 30; // 30 minutes for sunset/sunrise period

        // Test within sunrise period
        let current_time = sunrise + 15 * 60; // Sunrise + 15 minutes
        let result = test_fetch_daytime(current_time, sunrise, sunset, &Mode::On, sunset_time);
        assert_eq!(result, Daytime::Sunrise);

        // Test within day period.  Let's say 2 hours after sunrise
        let current_time = sunrise + 2 * 3600;
        let result = test_fetch_daytime(current_time, sunrise, sunset, &Mode::On, sunset_time);
        assert_eq!(result, Daytime::Day);

        // Test within sunset period. Let's say, 20 minutes before sunset
        let current_time = sunset - 20 * 60;
        let result = test_fetch_daytime(current_time, sunrise, sunset, &Mode::On, sunset_time);
        assert_eq!(result, Daytime::Sunset);

        // Test at night,  say, 2 hours after sunset
        let current_time = sunset + 2 * 3600;
        let result = test_fetch_daytime(current_time, sunrise, sunset, &Mode::On, sunset_time);
        assert_eq!(result, Daytime::Night);
    }

    #[test]
    fn test_fetch_daytime_mode_off() {
        let sunrise: i64 = 1747021974;
        let sunset: i64 = 1747077771;

        // Test during the day
        let current_time = sunrise + (sunset - sunrise) / 2; // Middle of day
        let result = test_fetch_daytime(current_time, sunrise, sunset, &Mode::Off, 30); // sunset_time is irrelevant in Off mode
        assert_eq!(result, Daytime::Day);

        // Test at night
        let current_time = sunset + 2 * 3600; // 2 hours after sunset
        let result = test_fetch_daytime(current_time, sunrise, sunset, &Mode::Off, 30);
        assert_eq!(result, Daytime::Night);
    }

    #[test]
    fn test_fetch_daytime_edge_cases() {
        let sunrise: i64 = 1747021974;
        let sunset: i64 = 1747077771;
        let sunset_time: i64 = 30;

        // Test at the exact time of sunrise
        let current_time = sunrise;
        let result_on_sunrise =
            test_fetch_daytime(current_time, sunrise, sunset, &Mode::On, sunset_time);
        let result_off_sunrise =
            test_fetch_daytime(current_time, sunrise, sunset, &Mode::Off, sunset_time);
        assert_eq!(result_on_sunrise, Daytime::Sunrise);
        assert_eq!(result_off_sunrise, Daytime::Day);

        // Test at the exact time of sunset
        let current_time = sunset - 1;
        let result_on_sunset =
            test_fetch_daytime(current_time, sunrise, sunset, &Mode::On, sunset_time);
        let result_off_sunset =
            test_fetch_daytime(current_time, sunrise, sunset, &Mode::Off, sunset_time);
        assert_eq!(result_on_sunset, Daytime::Sunset);
        assert_eq!(result_off_sunset, Daytime::Day);
    }
}
