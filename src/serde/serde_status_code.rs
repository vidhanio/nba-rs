use reqwest::StatusCode;
use serde::{self, de, Deserialize, Deserializer, Serializer};

/// # Errors
///
/// Returns an error if serialization fails.
pub fn serialize<S>(date: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(date.as_u16())
}

/// # Errors
///
/// Returns an error if deserialization fails.
pub fn deserialize<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    let status_code = u16::deserialize(deserializer)?;
    StatusCode::from_u16(status_code).map_err(|_| {
        de::Error::invalid_value(
            de::Unexpected::Unsigned(status_code as u64),
            &"a valid HTTP status code",
        )
    })
}
