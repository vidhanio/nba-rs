use chrono::NaiveDateTime;
use serde::{self, de, Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%m/%d/%Y";

/// # Errors
///
/// Returns an error if serialization fails.
pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    crate::serde::serde_none_as_empty_string(
        &date.map(|d| d.format(FORMAT).to_string()),
        serializer,
    )
}

/// # Errors
///
/// Returns an error if deserialization fails.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?.map_or_else(
        || Ok(None),
        |date| {
            NaiveDateTime::parse_from_str(&date, FORMAT).map_or_else(
                |e| Err(de::Error::custom(format!("invalid date: {e}"))),
                |date| Ok(Some(date)),
            )
        },
    )
}
