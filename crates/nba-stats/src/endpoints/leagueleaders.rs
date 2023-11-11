use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{
    fields::{
        trait_param, ActiveFlag, AllTime, LeagueId, PerMode, Scope, Season, SeasonType,
        StatCategory,
    },
    Endpoint,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Endpoint)]
#[endpoint(path = "leagueleaders")]
#[endpoint(result_set(field = league_leaders, ty = "S::ResultSet", name = "LeagueLeaders"))]
#[serde(deny_unknown_fields, rename_all = "PascalCase", bound = "")]
pub struct LeagueLeaders<S: LeagueLeadersSeason> {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub per_mode: PerMode,

    pub stat_category: StatCategory,

    pub season: S,

    pub season_type: SeasonType,

    pub scope: Scope,

    pub active_flag: Option<ActiveFlag>,
}

trait_param! {
    LeagueLeadersSeason {
        Season { LeagueLeadersSeasonResultSet };
        AllTime { LeagueLeadersAllTimeResultSet };
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct LeagueLeadersSeasonResultSet {
    pub player_id: u32,
    pub rank: u32,
    pub player: String,
    pub team_id: u32,
    pub team: String,
    pub gp: u32,
    pub min: f64,
    pub fgm: f64,
    pub fga: f64,
    pub fg_pct: f64,
    pub fg3m: Option<f64>,
    pub fg3a: Option<f64>,
    pub fg3_pct: Option<f64>,
    pub ftm: f64,
    pub fta: f64,
    pub ft_pct: f64,
    pub oreb: Option<f64>,
    pub dreb: Option<f64>,
    pub reb: f64,
    pub ast: f64,
    pub stl: Option<f64>,
    pub blk: Option<f64>,
    pub tov: Option<f64>,
    pub pf: Option<f64>,
    pub pts: f64,
    pub eff: f64,
    pub ast_tov: Option<f64>,
    pub stl_tov: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct LeagueLeadersAllTimeResultSet {
    pub player_id: i32,
    pub player_name: String,
    pub gp: i32,
    pub min: f64,
    pub fgm: f64,
    pub fga: f64,
    pub fg_pct: f64,
    pub fg3m: Option<f64>,
    pub fg3a: Option<f64>,
    pub fg3_pct: Option<f64>,
    pub ftm: f64,
    pub fta: f64,
    pub ft_pct: f64,
    pub oreb: Option<f64>,
    pub dreb: Option<f64>,
    pub reb: f64,
    pub ast: f64,
    pub stl: Option<f64>,
    pub blk: Option<f64>,
    pub tov: Option<f64>,
    pub pf: f64,
    pub pts: f64,
    pub ast_tov: Option<f64>,
    pub stl_tov: Option<f64>,
    pub efg_pct: f64,
    pub ts_pct: f64,
    pub gp_rank: i32,
    pub min_rank: i32,
    pub fgm_rank: i32,
    pub fga_rank: i32,
    pub fg_pct_rank: i32,
    pub fg3m_rank: i32,
    pub fg3a_rank: i32,
    pub fg3_pct_rank: i32,
    pub ftm_rank: i32,
    pub fta_rank: i32,
    pub ft_pct_rank: i32,
    pub oreb_rank: i32,
    pub dreb_rank: i32,
    pub reb_rank: i32,
    pub ast_rank: i32,
    pub stl_rank: i32,
    pub blk_rank: i32,
    pub tov_rank: i32,
    pub pf_rank: i32,
    pub pts_rank: i32,
    pub ast_tov_rank: i32,
    pub stl_tov_rank: i32,
    pub efg_pct_rank: i32,
    pub ts_pct_rank: i32,
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;

    use super::*;

    #[ignore = "don't want to spam the api"]
    #[tokio::test]
    async fn season_works() {
        println!(
            "{:#?}",
            assert_ok!(LeagueLeaders::<Season>::default().send().await)
        );
    }

    #[ignore = "don't want to spam the api"]
    #[tokio::test]
    async fn all_time_works() {
        println!(
            "{:#?}",
            assert_ok!(LeagueLeaders::<AllTime>::default().send().await)
        );
    }
}
