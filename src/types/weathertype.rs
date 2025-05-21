use crate::fmt;
use anyhow::anyhow;

// WeatherTypes
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

// fmt trait for creating path
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

// Other traits
impl WeatherType {
    // Get weathertype from str
    pub fn get_weathertype(weathertype: &str) -> Result<WeatherType, anyhow::Error> {
        match weathertype.to_lowercase().as_str() {
            "clear" => Ok(WeatherType::Clear),
            "few-clouds" => Ok(WeatherType::FewClouds),
            "scattered-clouds" => Ok(WeatherType::ScatteredClouds),
            "broken-clouds" => Ok(WeatherType::BrokenClouds),
            "overcast-clouds" => Ok(WeatherType::OvercastClouds),
            "drizzle" => Ok(WeatherType::Drizzle),
            "mist" => Ok(WeatherType::Mist),
            "rain" => Ok(WeatherType::Rain),
            "snow" => Ok(WeatherType::Snow),
            "thunder" => Ok(WeatherType::Thunder),
            _ => Err(anyhow!("not a known weathertype: {}", weathertype)),
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
}
