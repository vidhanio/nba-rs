use std::convert::Infallible;

use serde::{Deserialize, Deserializer, Serializer};

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize<S>(_: &Option<Infallible>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("")
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Infallible>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<()>::deserialize(deserializer).map(|_| None)
}
