use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AheadOrBehind {
    #[default]
    #[serde(rename = "Ahead or Behind")]
    AheadOrBehind,

    #[serde(rename = "Behind or Tied")]
    BehindOrTied,

    #[serde(rename = "Ahead or Tied")]
    AheadOrTied,
}
