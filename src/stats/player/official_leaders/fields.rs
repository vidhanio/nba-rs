pub use crate::{
    serde::serde_optional_infallible,
    stats::fields::{
        LeagueId, PerMode48 as PerMode, SeasonSince1946 as Season,
        SeasonTypeWithoutPlayIn as SeasonType,
    },
};

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scope {
    #[default]
    #[serde(rename = "S")]
    AllPlayers,

    #[serde(rename = "Rookies")]
    Rookies,
}
