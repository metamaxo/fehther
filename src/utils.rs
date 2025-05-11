use anyhow::anyhow;
use chrono::{DateTime, Duration, Local, TimeZone, Utc};

pub fn fetch_daytime(
    sunrise: i64,
    sunset: i64,
    timezone: i64,
    current_time: DateTime<Local>,
) -> Result<String, anyhow::Error> {
    // Create a DateTime object from the Unix timestamp
    let sunrise_utc = Utc.timestamp_opt(sunrise, 0).unwrap();
    let sunset_utc = Utc.timestamp_opt(sunset, 0).unwrap();

    // Add the timezone offset
    let adjusted_sunrise = sunrise_utc + Duration::seconds(timezone);
    let adjusted_sunset = sunset_utc + Duration::seconds(timezone);

    // Return night or day
    if adjusted_sunrise < current_time {
        Ok(String::from("day"))
    } else if adjusted_sunset < current_time {
        Ok(String::from("night"))
    } else {
        return Err(anyhow!("daytime not found"));
    }
}

#[test]
fn fetch_daytime_test() -> Result<(), anyhow::Error> {
    // Timezone for testing
    const TIMEZONE: i64 = 3600;

    // First test variables
    let sunrise_1: i64 = 1746935670;
    let sunset_1: i64 = 1746991276;
    let current_time_1 = DateTime::parse_from_str(
        "2025-05-11 13:11:33.665442683 +02:00",
        "%Y-%m-%d %H:%M:%S.%f %z",
    )
    .expect("Failed to parse datetime string");
    // Test 1, expected = "day"
    assert_eq!(
        &fetch_daytime(sunrise_1, sunset_1, TIMEZONE, current_time_1.into())?,
        "day"
    );
    // Second test variables
    let sunrise_2: i64 = 1746935670;
    let sunset_2: i64 = 1746991276;
    let current_time_2 = DateTime::parse_from_str(
        "2025-05-11 23:11:33.665442683 +02:00",
        "%Y-%m-%d %H:%M:%S.%f %z",
    )
    .expect("Failed to parse datetime string");
    // Test 1, expected = "night"
    assert_eq!(
        &fetch_daytime(sunrise_2, sunset_2, TIMEZONE, current_time_2.into())?,
        "night"
    );
    Ok(())
}
