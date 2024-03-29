use super::PlayersClutchParameters;
use crate::fields::MeasureType;

crate::endpoint! { "leaguedashplayerclutch"

    PlayersClutchMisc(
        PlayersClutchParameters {
            measure_type: MeasureType::Misc,
        }
    ) {
        league_dash_player_clutch["LeagueDashPlayerClutch"]: LeagueDashPlayerClutchRow {
            group_set: String,
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
            pts_off_tov: f64,
            pts_2nd_chance: f64,
            pts_fb: f64,
            pts_paint: f64,
            opp_pts_off_tov: f64,
            opp_pts_2nd_chance: f64,
            opp_pts_fb: f64,
            opp_pts_paint: f64,
            blk: f64,
            blka: f64,
            pf: f64,
            pfd: f64,
            nba_fantasy_pts: f64,
            gp_rank: u32,
            w_rank: u32,
            l_rank: u32,
            w_pct_rank: u32,
            min_rank: u32,
            pts_off_tov_rank: u32,
            pts_2nd_chance_rank: u32,
            pts_fb_rank: u32,
            pts_paint_rank: u32,
            opp_pts_off_tov_rank: u32,
            opp_pts_2nd_chance_rank: u32,
            opp_pts_fb_rank: u32,
            opp_pts_paint_rank: u32,
            blk_rank: u32,
            blka_rank: u32,
            pf_rank: u32,
            pfd_rank: u32,
            nba_fantasy_pts_rank: u32,
        },
    }
}
