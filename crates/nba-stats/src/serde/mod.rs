#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]

use serde::{Serialize, Serializer};

pub mod one_or_many;
pub mod optional_date;
pub mod optional_infallible;
pub mod player_or_team_char;
pub mod string_u32;

pub fn serialize_none_as_empty_string<T: Serialize, S>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(x) => x.serialize(serializer),
        None => serializer.serialize_str(""),
    }
}
