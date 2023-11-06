use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse()
        .map_err(|_| D::Error::invalid_value(Unexpected::Str(&s), &"a number"))
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize<S>(int: &u32, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    int.to_string().serialize(serializer)
}
