use std::{fmt::Display, num::ParseIntError, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A unique identifier for a game.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GameId(pub u64);

impl FromStr for GameId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl<'de> Deserialize<'de> for GameId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        s.parse::<Self>().map_err(serde::de::Error::custom)
    }
}

impl Display for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:010}", self.0)
    }
}

impl Serialize for GameId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
