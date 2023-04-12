use serde::{self, de, Deserialize, Deserializer, Serializer};
use time::Date;

const FORMAT: &[time::format_description::FormatItem<'static>] =
    time::macros::format_description!("[month]/[day]/[year]");

/// # Errors
///
/// Returns an error if serialization fails.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize<S>(date: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date = date
        .map(|d| d.format(FORMAT).map_err(serde::ser::Error::custom))
        .transpose()?;

    crate::serde::serialize_none_as_empty_string(&date, serializer)
}

/// # Errors
///
/// Returns an error if deserialization fails.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?.map_or_else(
        || Ok(None),
        |date| {
            Date::parse(&date, FORMAT)
                .map_or_else(|e| Err(de::Error::custom(e)), |date| Ok(Some(date)))
        },
    )
}
