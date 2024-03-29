use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeagueId {
    #[default]
    #[serde(rename = "00")]
    Nba,

    #[serde(rename = "01")]
    Aba,

    #[serde(rename = "10")]
    Wnba,

    #[serde(rename = "20")]
    GLeague,
}
