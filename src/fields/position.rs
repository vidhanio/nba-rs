use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    #[default]
    AllPositions,

    #[serde(rename = "F")]
    Forward,

    #[serde(rename = "C")]
    Center,

    #[serde(rename = "G")]
    Guard,
}
