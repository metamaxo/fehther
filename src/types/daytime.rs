use crate::fmt;
// Daytimes
#[derive(Eq, Hash, Debug, PartialEq, PartialOrd)]
pub enum Daytime {
    Day,
    Night,
    Sunrise,
    Sunset,
}

// fmt trait for creating path
impl fmt::Display for Daytime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Daytime::Day => write!(f, "day"),
            Daytime::Night => write!(f, "night"),
            Daytime::Sunrise => write!(f, "Sunrise"),
            Daytime::Sunset => write!(f, "Sunset"),
        }
    }
}
