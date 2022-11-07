use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatCategory {
    #[serde(rename = "MIN")]
    Minutes,

    #[serde(rename = "OREB")]
    OffensiveRebounds,

    #[serde(rename = "DREB")]
    DefensiveRebounds,

    #[serde(rename = "REB")]
    Rebounds,

    #[serde(rename = "AST")]
    Assists,

    #[serde(rename = "STL")]
    Steals,

    #[serde(rename = "BLK")]
    Blocks,

    #[serde(rename = "TOV")]
    Turnovers,

    #[serde(rename = "EFF")]
    Efficiency,

    #[default]
    #[serde(rename = "PTS")]
    Points,
}
