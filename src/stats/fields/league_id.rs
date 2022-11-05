use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeagueId {
    #[default]
    #[serde(rename = "00")]
    Nba,

    #[serde(rename = "10")]
    Wnba,

    #[serde(rename = "20")]
    GLeague,
}
