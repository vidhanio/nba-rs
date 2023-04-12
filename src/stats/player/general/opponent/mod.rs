mod parameters;

use crate::fields::MeasureType;
pub use parameters::PlayersOpponentParameters;

crate::endpoint! { "leagueplayerondetails"

    PlayersOpponent(
        PlayersOpponentParameters {
            measure_type: MeasureType::Opponent,
        }
    ) {
        players_on_court_league_player_details["PlayersOnCourtLeaguePlayerDetails"]: PlayersOnCourtLeaguePlayerDetailsRow {
            group_set: String,
            team_id: u32,
            team_abbreviation: String,
            team_name: String,
            vs_player_id: u32,
            vs_player_name: String,
            court_status: String,
            gp: u32,
            w: u32,
            l: u32,
            w_pct: f64,
            min: f64,
            opp_fgm: f64,
            opp_fga: f64,
            opp_fg_pct: f64,
            opp_fg3m: Option<f64>,
            opp_fg3a: Option<f64>,
            opp_fg3_pct: Option<f64>,
            opp_ftm: f64,
            opp_fta: f64,
            opp_ft_pct: f64,
            opp_oreb: f64,
            opp_dreb: f64,
            opp_reb: f64,
            opp_ast: f64,
            opp_tov: f64,
            opp_stl: f64,
            opp_blk: f64,
            opp_blka: f64,
            opp_pf: f64,
            opp_pfd: f64,
            opp_pts: f64,
            plus_minus: f64,
            gp_rank: u32,
            w_rank: u32,
            l_rank: u32,
            w_pct_rank: u32,
            min_rank: u32,
            opp_fgm_rank: u32,
            opp_fga_rank: u32,
            opp_fg_pct_rank: u32,
            opp_fg3m_rank: Option<u32>,
            opp_fg3a_rank: Option<u32>,
            opp_fg3_pct_rank: Option<u32>,
            opp_ftm_rank: u32,
            opp_fta_rank: u32,
            opp_ft_pct_rank: u32,
            opp_oreb_rank: u32,
            opp_dreb_rank: u32,
            opp_reb_rank: u32,
            opp_ast_rank: u32,
            opp_tov_rank: u32,
            opp_stl_rank: u32,
            opp_blk_rank: u32,
            opp_blka_rank: u32,
            opp_pf_rank: u32,
            opp_pfd_rank: u32,
            opp_pts_rank: u32,
            plus_minus_rank: u32,
        },
    }
}
