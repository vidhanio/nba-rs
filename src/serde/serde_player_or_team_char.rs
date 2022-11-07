use serde::{de, Deserialize, Deserializer, Serializer};

use crate::stats::fields::PlayerOrTeam;

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize<S>(value: &PlayerOrTeam, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        PlayerOrTeam::Player => serializer.serialize_str("P"),
        PlayerOrTeam::Team => serializer.serialize_str("T"),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<PlayerOrTeam, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match s.as_str() {
        "P" => Ok(PlayerOrTeam::Player),
        "T" => Ok(PlayerOrTeam::Team),
        _ => Err(de::Error::invalid_value(
            de::Unexpected::Other("string not `P` or `T`"),
            &"`P` or `T`",
        )),
    }
}
