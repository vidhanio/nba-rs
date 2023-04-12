use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatCategory {
    #[serde(rename = "MIN")]
    Minutes,

    #[serde(rename = "FGM")]
    FieldGoalsMade,

    #[serde(rename = "FGA")]
    FieldGoalsAttempted,

    #[serde(rename = "FG_PCT")]
    FieldGoalPercentage,

    #[serde(rename = "FG3M")]
    ThreePointersMade,

    #[serde(rename = "FG3A")]
    ThreePointersAttempted,

    #[serde(rename = "FG3_PCT")]
    ThreePointPercentage,

    #[serde(rename = "FTM")]
    FreeThrowsMade,

    #[serde(rename = "FTA")]
    FreeThrowsAttempted,

    #[serde(rename = "FT_PCT")]
    FreeThrowPercentage,

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

    #[default]
    #[serde(rename = "PTS")]
    Points,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatCategorySimple {
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
