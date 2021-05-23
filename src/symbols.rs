use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;


use anyhow::bail;

pub enum Wind {
    N,
    E,
    W,
    S,
    NE,
    NW,
    SE,
    SW,
}

impl Wind {
    pub fn value(&self) -> char {
        match self {
            Wind::N => '\u{2191}',
            Wind::E => '\u{2192}',
            Wind::W => '\u{2190}',
            Wind::S => '\u{2193}',
            Wind::NE => '\u{2197}',
            Wind::NW => '\u{2196}',
            Wind::SE => '\u{2198}',
            Wind::SW => '\u{2199}',
        }
    }
}

impl FromStr for Wind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Wind::N),
            "E" => Ok(Wind::E),
            "W" => Ok(Wind::W),
            "S" => Ok(Wind::S),
            "NE" => Ok(Wind::NE),
            "NW" => Ok(Wind::NW),
            "SE" => Ok(Wind::SE),
            "SW" => Ok(Wind::SW),
            _ => bail!(UnmatchedPattern::Msg(format!(
                "Wind match not found for {}",
                s
            ))),
        }
    }
}

pub enum Weather {
    Sun,
    // \u{1f31e}
    Cloud,
    // U+2601
    SunBehindCloud,
    //'\u{26C5}',
    Rain,
    // 1F327
    Moon, // 1F319
}

impl Weather {
    pub fn value(&self) -> char {
        match self {
            Weather::Sun => '\u{1f31e}',
            Weather::Cloud => '\u{2601}',
            Weather::SunBehindCloud => '\u{26C5}',
            Weather::Rain => '\u{1F327}',
            Weather::Moon => '\u{1F319}',
        }
    }
}

impl FromStr for Weather {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "3" => Ok(Weather::SunBehindCloud),
            "3n" | "4n" | "2" | "4" => Ok(Weather::Cloud),
            "46" | "10" | "5n" | "10n" | "9n" => Ok(Weather::Rain),
            "1n" => Ok(Weather::Moon),
            "1" => Ok(Weather::Sun),
            _ => bail!(UnmatchedPattern::Msg(format!(
                "Weather match not found for {}",
                s
            ))),
        }
    }
}

#[derive(Debug)]
pub enum UnmatchedPattern {
    Msg(String),
}

impl Error for UnmatchedPattern {}

impl Display for UnmatchedPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
