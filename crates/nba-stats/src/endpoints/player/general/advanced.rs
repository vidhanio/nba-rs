use super::PlayersGeneralParameters;
use crate::fields::MeasureType;

crate::endpoint! { "leaguedashplayerstats"

    PlayersAdvanced(
        PlayersGeneralParameters {
            measure_type: MeasureType::Advanced,
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
            e_off_rating: f64,
            off_rating: f64,

            #[serde(rename(deserialize = "sp_work_OFF_RATING"))]
            sp_work_off_rating: f64,

            e_def_rating: f64,
            def_rating: f64,

            #[serde(rename(deserialize = "sp_work_DEF_RATING"))]
            sp_work_def_rating: f64,

            e_net_rating: f64,
            net_rating: f64,

            #[serde(rename(deserialize = "sp_work_NET_RATING"))]
            sp_work_net_rating: f64,

            ast_pct: f64,
            ast_to: f64,
            ast_ratio: f64,
            oreb_pct: f64,
            dreb_pct: f64,
            reb_pct: f64,
            tm_tov_pct: f64,
            e_tov_pct: f64,
            efg_pct: f64,
            ts_pct: f64,
            usg_pct: f64,
            e_usg_pct: f64,
            e_pace: f64,
            pace: f64,
            pace_per40: f64,

            #[serde(rename(deserialize = "sp_work_PACE"))]
            sp_work_pace: f64,

            pie: f64,
            poss: u32,
            fgm: u32,
            fga: u32,
            fgm_pg: f64,
            fga_pg: f64,
            fg_pct: f64,
            gp_rank: u32,
            w_rank: u32,
            l_rank: u32,
            w_pct_rank: u32,
            min_rank: u32,
            e_off_rating_rank: u32,
            off_rating_rank: u32,

            #[serde(rename(deserialize = "sp_work_OFF_RATING_RANK"))]
            sp_work_off_rating_rank: u32,

            e_def_rating_rank: u32,
            def_rating_rank: u32,

            #[serde(rename(deserialize = "sp_work_DEF_RATING_RANK"))]
            sp_work_def_rating_rank: u32,

            e_net_rating_rank: u32,
            net_rating_rank: u32,

            #[serde(rename(deserialize = "sp_work_NET_RATING_RANK"))]
            sp_work_net_rating_rank: u32,

            ast_pct_rank: u32,
            ast_to_rank: u32,
            ast_ratio_rank: u32,
            oreb_pct_rank: u32,
            dreb_pct_rank: u32,
            reb_pct_rank: u32,
            tm_tov_pct_rank: u32,
            e_tov_pct_rank: u32,
            efg_pct_rank: u32,
            ts_pct_rank: u32,
            usg_pct_rank: u32,
            e_usg_pct_rank: u32,
            e_pace_rank: u32,
            pace_rank: u32,

            #[serde(rename(deserialize = "sp_work_PACE_RANK"))]
            sp_work_pace_rank: u32,

            pie_rank: u32,
            fgm_rank: u32,
            fga_rank: u32,
            fgm_pg_rank: u32,
            fga_pg_rank: u32,
            fg_pct_rank: u32,
        },
    }
}
