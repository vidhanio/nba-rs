use serde::{de::Error as _, ser::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use time::{format_description::FormatItem, macros::format_description, PrimitiveDateTime};

static FORMAT_DESCRIPTION: &[FormatItem] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]");

pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<PrimitiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    PrimitiveDateTime::parse(&s, &FORMAT_DESCRIPTION).map_err(D::Error::custom)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize<S>(
    datetime: &PrimitiveDateTime,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    datetime
        .format(&FORMAT_DESCRIPTION)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

#[cfg(test)]
mod tests {
    use serde_test::Token;
    use time::macros::datetime;

    use super::*;

    #[test]
    fn serde_works() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(transparent)]
        struct TestPrimitiveDateTime(#[serde(with = "super")] PrimitiveDateTime);

        serde_test::assert_tokens(
            &TestPrimitiveDateTime(datetime!(1970-01-01 00:00:00.000000)),
            &[Token::Str("1970-01-01 00:00:00.000000")],
        );
    }
}
