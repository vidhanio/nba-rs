use std::fmt::Display;

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/// A unique identifier for a game.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GameId(pub u64);

impl<'de> Deserialize<'de> for GameId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        s.parse::<u64>().map(Self).map_err(D::Error::custom)
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
