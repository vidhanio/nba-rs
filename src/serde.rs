#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]

use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VecOrSingle<T> {
    Vec(Vec<T>),
    Single(T),
}

impl<T> VecOrSingle<T> {
    pub fn into_vec(self) -> Vec<T> {
        match self {
            Self::Vec(v) => v,
            Self::Single(s) => vec![s],
        }
    }
}

impl<T> From<Vec<T>> for VecOrSingle<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Vec(value)
    }
}

impl<T> From<T> for VecOrSingle<T> {
    fn from(value: T) -> Self {
        Self::Single(value)
    }
}

impl<T> From<VecOrSingle<T>> for Vec<T> {
    fn from(value: VecOrSingle<T>) -> Self {
        match value {
            VecOrSingle::Vec(v) => v,
            VecOrSingle::Single(s) => vec![s],
        }
    }
}

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

pub mod serde_optional_infallible {
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
}
