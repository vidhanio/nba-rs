use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActiveFlag {
    #[default]
    #[serde(rename = "No")]
    No,

    #[serde(rename = "Yes")]
    Yes,
}

impl From<ActiveFlag> for bool {
    fn from(value: ActiveFlag) -> Self {
        match value {
            ActiveFlag::No => false,
            ActiveFlag::Yes => true,
        }
    }
}

impl From<bool> for ActiveFlag {
    fn from(value: bool) -> Self {
        if value {
            Self::Yes
        } else {
            Self::No
        }
    }
}
