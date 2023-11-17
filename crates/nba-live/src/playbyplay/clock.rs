use std::{
    fmt,
    fmt::{Display, Formatter},
    num::{ParseFloatError, ParseIntError},
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

/// A clock representing the time left in a period.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Clock {
    /// The number of minutes left in the period.
    pub minutes: u32,

    /// The number of seconds left in the minute.
    pub seconds: f64,
}

impl std::str::FromStr for Clock {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let re = regex!(r"PT(?P<minutes>\d{2})M(?P<seconds>\d{2}\.\d{2})S");

        let (before, after) = s.split_once('M').ok_or(ParseError::MissingM)?;
        let minutes = before
            .strip_prefix("PT")
            .ok_or(ParseError::MissingPT)?
            .parse::<u32>()
            .map_err(ParseError::ParseMinutes)?;

        let seconds = after
            .strip_suffix('S')
            .ok_or(ParseError::MissingS)?
            .parse::<f64>()
            .map_err(ParseError::ParseSeconds)?;

        Ok(Self { minutes, seconds })
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PT{:02}M{:05.2}S", self.minutes, self.seconds)
    }
}

impl Serialize for Clock {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Clock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        s.parse::<Self>().map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("missing `PT`")]
    MissingPT,

    #[error("missing `M`")]
    MissingM,

    #[error("missing `S`")]
    MissingS,

    #[error("could not parse minutes")]
    ParseMinutes(ParseIntError),

    #[error("could not parse seconds")]
    ParseSeconds(ParseFloatError),
}

#[cfg(test)]
mod tests {
    use claims::{assert_err_eq, assert_matches, assert_ok_eq};

    use super::*;

    #[test]
    fn clock_parses() {
        assert_ok_eq!(
            "PT00M00.00S".parse::<Clock>(),
            Clock {
                minutes: 0,
                seconds: 0.0
            }
        );

        assert_ok_eq!(
            "PT00M00.01S".parse::<Clock>(),
            Clock {
                minutes: 0,
                seconds: 0.01
            }
        );

        assert_ok_eq!(
            "PT12M00.00S".parse::<Clock>(),
            Clock {
                minutes: 12,
                seconds: 0.0
            }
        );
    }

    #[test]
    fn clock_fails_parse() {
        assert_err_eq!("PT00M00.00".parse::<Clock>(), ParseError::MissingS);

        assert_err_eq!("PT0000.00S".parse::<Clock>(), ParseError::MissingM);

        assert_err_eq!("00M00.00S".parse::<Clock>(), ParseError::MissingPT);

        assert_matches!(
            "PT00.1M00.00S".parse::<Clock>(),
            Err(ParseError::ParseMinutes(_))
        );

        assert_matches!(
            "PT00M00.2.0S".parse::<Clock>(),
            Err(ParseError::ParseSeconds(_))
        );
    }
}
