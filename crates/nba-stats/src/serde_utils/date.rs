const FORMAT: &[time::format_description::FormatItem<'static>] =
    time::macros::format_description!("[month]/[day]/[year]");

pub mod option {
    use serde::{self, de, ser::Error, Deserialize, Deserializer, Serializer};
    use time::Date;

    use super::FORMAT;

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(date: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let date = date
            .map(|d| d.format(FORMAT).map_err(S::Error::custom))
            .transpose()?;

        crate::serde_utils::none_as_empty_str(&date, serializer)
    }

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
}
