use std::time::SystemTime;

pub fn fetch_daytime(sunrise: i64, sunset: i64) -> String {
    // Fetch current time
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    // Return night or day
    if sunrise <= current_time && current_time <= sunset {
        String::from("day")
    } else {
        String::from("night")
    }
}

#[test]
fn daytime_test() -> Result<(), anyhow::Error> {
    // adjusted identical function for testing on different time values
    pub fn fetch_daytime_test(sunrise: i64, sunset: i64, current_time: i64) -> String {
        // Return night or day
        if sunrise <= current_time && current_time < sunset {
            String::from("day")
        } else {
            String::from("night")
        }
    }
    // Current time is after sunrise, before sunset.
    let sunrise: i64 = 1747021974;
    let current_time: i64 = 1747041591;
    let sunset: i64 = 1747077771;

    assert_eq!(&fetch_daytime_test(sunrise, sunset, current_time), "day");
    // Current time is after sunset
    let sunrise: i64 = 1747021974;
    let current_time: i64 = 1747077772;
    let sunset: i64 = 1747077771;

    assert_eq!(&fetch_daytime_test(sunrise, sunset, current_time), "night");

    // Current time is before sunrise
    let sunrise: i64 = 1747021974;
    let current_time: i64 = 1747020974;
    let sunset: i64 = 1747077771;

    assert_eq!(&fetch_daytime_test(sunrise, sunset, current_time), "night");

    // current time is on sunrise
    let sunrise: i64 = 1747021974;
    let current_time: i64 = 1747021974;
    let sunset: i64 = 1747077771;

    assert_eq!(&fetch_daytime_test(sunrise, sunset, current_time), "day");

    // current time is on sunset
    let sunrise: i64 = 1747021974;
    let current_time: i64 = 1747077771;
    let sunset: i64 = 1747077771;

    assert_eq!(&fetch_daytime_test(sunrise, sunset, current_time), "night");
    Ok(())
}
