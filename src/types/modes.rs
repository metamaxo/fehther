use anyhow::anyhow;
// Modes
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Mode {
    Cycle,
    Daytime,
    Weather,
}

impl Mode {
    // Get mode type from str
    pub fn from_string(mode: &str) -> Result<Mode, anyhow::Error> {
        match mode {
            "cycle-mode" => Ok(Mode::Cycle),
            "daytime-mode" => Ok(Mode::Daytime),
            "weather-mode" => Ok(Mode::Weather),
            _ => Err(anyhow!("unknown mode")),
        }
    }
    // Get String from mode type
    pub fn to_string_vec() -> Vec<String> {
        vec![
            "cycle-mode".to_string(),
            "daytime-mode".to_string(),
            "weather-mode".to_string(),
        ]
    }
}
