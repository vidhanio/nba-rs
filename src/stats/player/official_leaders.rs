pub mod fields {
    pub use crate::{
        serde::serde_optional_infallible,
        stats::fields::{
            LeagueId, PerMode48 as PerMode, Season2022To1946 as Season,
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
}

use std::convert::Infallible;

use fields::{
    serde_optional_infallible, LeagueId, PerMode, Scope, Season, SeasonType, StatCategory,
};

crate::endpoint! {
    OfficialLeaders("leagueleaders") {
        #[serde(rename = "LeagueID")]
        league_id: LeagueId,

        season: Season,

        season_type: SeasonType,

        per_mode: PerMode,

        stat_category: StatCategory,

        scope: Scope,

        #[serde(with = "serde_optional_infallible")]
        active_flag: Option<Infallible>,
    } => {
        league_leaders: LeagueLeadersRow("LeagueLeaders") {
            player_id: u32,
            rank: u32,
            player: String,
            team: String,
            gp: u32,
            min: f64,
            fgm: f64,
            fga: f64,
            fg_pct: f64,
            fg3m: f64,
            fg3a: f64,
            fg3_pct: f64,
            ftm: f64,
            fta: f64,
            ft_pct: f64,
            oreb: f64,
            dreb: f64,
            reb: f64,
            ast: f64,
            stl: f64,
            blk: f64,
            tov: f64,
            pf: Option<f64>,
            pts: f64,
            eff: f64,
            ast_tov: Option<f64>,
            stl_tov: Option<f64>,
        },
    }
}
