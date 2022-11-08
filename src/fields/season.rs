use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Season(pub u32);

impl<'de> Deserialize<'de> for Season {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let (season, next) = s.split_once('-').ok_or_else(|| {
            de::Error::invalid_value(
                de::Unexpected::Other("string without a `-`"),
                &"string with a `-`",
            )
        })?;

        let season = season
            .parse::<u32>()
            .map_err(|_| de::Error::invalid_type(de::Unexpected::Other("non u32"), &"u32"))?;

        let next = next
            .parse::<u32>()
            .map_err(|_| de::Error::invalid_type(de::Unexpected::Other("non u32"), &"u32"))?;

        if next != season % 100 + 1 {
            return Err(de::Error::invalid_value(
                de::Unexpected::Other("season `YYYY-NN` where `NN` != `YYYY` % 100 + 1"),
                &"string in the format of `YYYY-NN` where `NN` is the next year's last two digits",
            ));
        }

        Ok(Self(season))
    }
}

impl Serialize for Season {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{:04}-{:02}", self.0, self.0 % 100 + 1);
        serializer.serialize_str(&s)
    }
}
