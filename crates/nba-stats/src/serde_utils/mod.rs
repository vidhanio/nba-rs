use serde::{Serialize, Serializer};

pub mod date;
pub mod one_or_many;
pub mod u32_as_str;

pub fn none_as_empty_str<T: Serialize, S>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(x) => x.serialize(serializer),
        None => "".serialize(serializer),
    }
}
