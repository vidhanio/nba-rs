use serde::{Deserialize, Serialize};
use time::Date;

use crate::{serde_utils, Endpoint};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Endpoint)]
#[endpoint(path = "playergamelogs")]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
#[endpoint(result_set(field = player_game_logs, ty = "PlayerGameLogsResultSet", name = "PlayerGameLogs"))]
pub struct PlayerGameLogs {
    #[serde(with = "serde_utils::date::option")]
    pub date_from: Option<Date>,
    #[serde(with = "serde_utils::date::option")]
    pub date_to: Option<Date>,
    pub game_segment: Option<()>,
    pub ist_round: Option<()>,
    pub last_n_games: Option<()>,
    pub league_id: Option<()>,
    pub location: Option<()>,
    pub measure_type: Option<()>,
    pub month: Option<()>,
    pub opp_team_id: Option<()>,
    pub outcome: Option<()>,
    pub po_round: Option<()>,
    pub per_mode: Option<()>,
    pub period: Option<()>,
    pub player_id: Option<()>,
    pub season_segment: Option<()>,
    pub season_type: Option<()>,
    pub season_year: Option<()>,
    pub shot_clock_range: Option<()>,
    pub team_id: Option<()>,
    pub vs_conference: Option<()>,
    pub vs_division: Option<()>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PlayerGameLogsResultSet {
    pub season_year: String,
    pub player_id: i32,
    pub player_name: String,
    pub nickname: String,
    pub team_id: i32,
    pub team_abbreviation: String,
    pub team_name: String,
    pub game_id: String,
    pub game_date: String,
    pub matchup: String,
    pub wl: String,
    pub min: f64,
    pub fgm: i32,
    pub fga: i32,
    pub fg_pct: f64,
    pub fg3m: i32,
    pub fg3a: i32,
    pub fg3_pct: f64,
    pub ftm: i32,
    pub fta: i32,
    pub ft_pct: f64,
    pub oreb: i32,
    pub dreb: i32,
    pub reb: i32,
    pub ast: i32,
    pub tov: i32,
    pub stl: i32,
    pub blk: i32,
    pub blka: i32,
    pub pf: i32,
    pub pfd: i32,
    pub pts: i32,
    pub plus_minus: i32,
    pub nba_fantasy_pts: f64,
    pub dd2: i32,
    pub td3: i32,
    pub wnba_fantasy_pts: f64,
    pub gp_rank: i32,
    pub w_rank: i32,
    pub l_rank: i32,
    pub w_pct_rank: i32,
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
    pub tov_rank: i32,
    pub stl_rank: i32,
    pub blk_rank: i32,
    pub blka_rank: i32,
    pub pf_rank: i32,
    pub pfd_rank: i32,
    pub pts_rank: i32,
    pub plus_minus_rank: i32,
    pub nba_fantasy_pts_rank: i32,
    pub dd2_rank: i32,
    pub td3_rank: i32,
    pub wnba_fantasy_pts_rank: i32,
    pub available_flag: i32,
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;

    use super::*;

    #[tokio::test]
    async fn works() {
        println!("{:#?}", assert_ok!(PlayerGameLogs::default().send().await));
    }
}
