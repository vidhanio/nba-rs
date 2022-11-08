#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]

use serde::{Serialize, Serializer};

pub mod serde_optional_date;
pub mod serde_optional_infallible;
pub mod serde_player_or_team_char;
pub mod serde_player_or_team_str;
pub mod serde_status_code;
pub mod vec_or_single;

pub fn serde_none_as_empty_string<T: Serialize, S>(
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
