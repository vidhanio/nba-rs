use serde::{Deserialize, Serialize};

pub mod u32str {
    use serde::{de, Deserialize, Deserializer, Serializer};

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(value: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}

pub mod season {
    use serde::{de, Deserialize, Deserializer, Serializer};

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(value: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let next_year = (value + 1) % 100;
        serializer.serialize_str(&format!("{value}-{next_year}"))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.split_once('-')
            .ok_or_else(|| de::Error::custom(format!("invalid season string: {s}")))?
            .0
            .parse()
            .map_err(de::Error::custom)
    }

    pub mod optional {
        use serde::{de, Deserialize, Deserializer, Serializer};

        #[allow(clippy::trivially_copy_pass_by_ref)]
        pub fn serialize<S>(value: &Option<u32>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(value) => super::serialize(value, serializer),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = Option::<String>::deserialize(deserializer)?;

            match s {
                Some(s) => s
                    .split_once('-')
                    .ok_or_else(|| de::Error::custom(format!("invalid season string: {s}")))?
                    .0
                    .parse()
                    .map_err(de::Error::custom)
                    .map(Some),
                None => Ok(None),
            }
        }
    }
}

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
