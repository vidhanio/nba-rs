use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum YesOrNo {
    #[default]
    #[serde(rename = "N")]
    No,

    #[serde(rename = "Y")]
    Yes,
}

impl From<YesOrNo> for bool {
    fn from(value: YesOrNo) -> Self {
        match value {
            YesOrNo::No => false,
            YesOrNo::Yes => true,
        }
    }
}

impl From<bool> for YesOrNo {
    fn from(value: bool) -> Self {
        if value {
            Self::Yes
        } else {
            Self::No
        }
    }
}
