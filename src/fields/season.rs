use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Season(pub u32);

impl Default for Season {
    fn default() -> Self {
        Self(2023)
    }
}

impl<'de> Deserialize<'de> for Season {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let (last, season) = s.split_once('-').ok_or_else(|| {
            de::Error::invalid_value(
                de::Unexpected::Other("string without a `-`"),
                &"string with a `-`",
            )
        })?;

        let last = last
            .parse::<u32>()
            .map_err(|_| de::Error::invalid_type(de::Unexpected::Other("non u32"), &"u32"))?;

        let season = season
            .parse::<u32>()
            .map_err(|_| de::Error::invalid_type(de::Unexpected::Other("non u32"), &"u32"))?;

        if season != (last + 1) % 100 {
            return Err(de::Error::invalid_value(
                de::Unexpected::Other("season `LLLL-YY` where `YY` != `LLLL` % 100 + 1"),
                &"string in the format of `LLLL-YY` where `YY` is the year's last two digits",
            ));
        }

        Ok(Self(last + 1))
    }
}

impl Serialize for Season {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{:04}-{:02}", self.0 - 1, self.0 % 100);
        serializer.serialize_str(&s)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllTime {
    #[default]
    #[serde(rename = "All Time")]
    AllTime,
}
