use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SeasonType {
    #[serde(rename = "Pre Season")]
    PreSeason,

    #[default]
    #[serde(rename = "Regular Season")]
    RegularSeason,

    #[serde(rename = "Playoffs")]
    Playoffs,

    #[serde(rename = "All Star")]
    AllStar,

    #[serde(rename = "PlayIn")]
    PlayIn,
}
