use super::PlayersGeneralParameters;
use crate::fields::MeasureType;

crate::endpoint! { "leaguedashplayerstats"

    PlayersDefense(
        PlayersGeneralParameters {
            measure_type: MeasureType::Defense,
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
            def_rating: f64,
            dreb: f64,
            dreb_pct: f64,
            pct_dreb: f64,
            stl: f64,
            pct_stl: f64,
            blk: f64,
            pct_blk: f64,
            opp_pts_off_tov: f64,
            opp_pts_2nd_chance: f64,
            opp_pts_fb: f64,
            opp_pts_paint: f64,
            def_ws: f64,
            gp_rank: u32,
            w_rank: u32,
            l_rank: u32,
            w_pct_rank: u32,
            min_rank: u32,
            def_rating_rank: u32,
            dreb_rank: u32,
            dreb_pct_rank: u32,
            pct_dreb_rank: u32,
            stl_rank: u32,
            pct_stl_rank: u32,
            blk_rank: u32,
            pct_blk_rank: u32,
            opp_pts_off_tov_rank: u32,
            opp_pts_2nd_chance_rank: u32,
            opp_pts_fb_rank: u32,
            opp_pts_paint_rank: u32,
            def_ws_rank: u32,
        },
    }
}
