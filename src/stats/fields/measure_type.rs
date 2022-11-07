use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeasureType {
    #[default]
    #[serde(rename = "Base")]
    Base,

    #[serde(rename = "Advanced")]
    Advanced,

    #[serde(rename = "Misc")]
    Misc,

    #[serde(rename = "Scoring")]
    Scoring,

    #[serde(rename = "Usage")]
    Usage,

    #[serde(rename = "Opponent")]
    Opponent,

    #[serde(rename = "Defense")]
    Defense,
}
