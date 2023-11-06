use serde::{Deserialize, Deserializer, Serializer};

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize<S>(v: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_str(v)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    s.parse::<u32>()
        .map_err(|_| serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"u32"))
}
