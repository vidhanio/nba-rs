use super::PlayersGeneralParameters;
use crate::fields::MeasureType;

crate::endpoint! { "leaguedashplayerstats"

    PlayersUsage(
        PlayersGeneralParameters {
            measure_type: MeasureType::Usage,
        }
    ) {
        league_dash_player_stats["LeagueDashPlayerStats"]: LeagueDashPlayerStatsRow {
            player_id: u32,
            player_name: String,
            nickname: String,
            team_id: u32,
            team_abbreviation: String,
            age: f64,
            gp: u32,
            w: u32,
            l: u32,
            w_pct: f64,
            min: f64,
            usg_pct: f64,
            pct_fgm: f64,
            pct_fga: f64,
            pct_fg3m: Option<f64>,
            pct_fg3a: Option<f64>,
            pct_ftm: f64,
            pct_fta: f64,
            pct_oreb: f64,
            pct_dreb: f64,
            pct_reb: f64,
            pct_ast: f64,
            pct_tov: f64,
            pct_stl: f64,
            pct_blk: f64,
            pct_blka: f64,
            pct_pf: f64,
            pct_pfd: f64,
            pct_pts: f64,
            gp_rank: u32,
            w_rank: u32,
            l_rank: u32,
            w_pct_rank: u32,
            min_rank: u32,
            usg_pct_rank: u32,
            pct_fgm_rank: u32,
            pct_fga_rank: u32,
            pct_fg3m_rank: Option<u32>,
            pct_fg3a_rank: Option<u32>,
            pct_ftm_rank: u32,
            pct_fta_rank: u32,
            pct_oreb_rank: u32,
            pct_dreb_rank: u32,
            pct_reb_rank: u32,
            pct_ast_rank: u32,
            pct_tov_rank: u32,
            pct_stl_rank: u32,
            pct_blk_rank: u32,
            pct_blka_rank: u32,
            pct_pf_rank: u32,
            pct_pfd_rank: u32,
            pct_pts_rank: u32,
        },
    }
}
