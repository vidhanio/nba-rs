use std::fmt::Debug;

use nba_macros::Endpoint;
use sealed::sealed;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::fields::{
    ActiveFlag, AllTime, LeagueId, PerMode, Scope, Season, SeasonType, StatCategory,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Endpoint)]
#[endpoint(path = "leagueleaders")]
#[endpoint(row(field = league_leaders, ty = "S::Row", row = "LeagueLeaders"))]
#[serde(deny_unknown_fields, rename_all = "PascalCase", bound = "")]
pub struct LeagueLeaders<S>
where
    S: LeagueLeadersSeason + Serialize + DeserializeOwned + Debug + Clone + PartialEq + Eq + Sync,
    S::Row: Serialize + DeserializeOwned + Debug + Clone + PartialEq + Sync,
{
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub per_mode: PerMode,

    pub stat_category: StatCategory,

    pub season: S,

    pub season_type: SeasonType,

    pub scope: Scope,

    pub active_flag: Option<ActiveFlag>,
}

#[sealed]
pub trait LeagueLeadersSeason {
    type Row;
}

#[sealed]
impl LeagueLeadersSeason for Season {
    type Row = LeagueLeadersSeasonRow;
}

#[sealed]
impl LeagueLeadersSeason for AllTime {
    type Row = LeagueLeadersAllTimeRow;
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct LeagueLeadersSeasonRow {
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
pub struct LeagueLeadersAllTimeRow {
    pub player_id: u32,
    pub player_name: Option<String>,
    pub gp: u32,
    pub min: Option<f64>,
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
    pub reb: Option<f64>,
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
    pub gp_rank: u32,
    pub min_rank: u32,
    pub fgm_rank: u32,
    pub fga_rank: u32,
    pub fg_pct_rank: u32,
    pub fg3m_rank: u32,
    pub fg3a_rank: u32,
    pub fg3_pct_rank: u32,
    pub ftm_rank: u32,
    pub fta_rank: u32,
    pub ft_pct_rank: u32,
    pub oreb_rank: u32,
    pub dreb_rank: u32,
    pub reb_rank: u32,
    pub ast_rank: u32,
    pub stl_rank: u32,
    pub blk_rank: u32,
    pub tov_rank: u32,
    pub pf_rank: u32,
    pub pts_rank: u32,
    pub ast_tov_rank: u32,
    pub stl_tov_rank: u32,
    pub efg_pct_rank: Option<u32>,
    pub ts_pct_rank: Option<u32>,
    pub efg_pct1: Option<f64>,
    pub ts_pct1: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Endpoint;

    #[tokio::test]
    async fn all_time_leaders() -> Result<(), Box<dyn std::error::Error>> {
        let resp = LeagueLeaders::<Season> {
            per_mode: PerMode::Totals,
            stat_category: StatCategory::Points,
            season_type: SeasonType::RegularSeason,
            ..Default::default()
        }
        .send()
        .await?;

        // print table of top 10 players
        for player in resp.result_sets.league_leaders.into_iter().take(10) {
            println!(
                "{:<20} {:<20} {:<20}",
                player.player_id, player.pts, player.gp
            );
        }

        Ok(())
    }
}
