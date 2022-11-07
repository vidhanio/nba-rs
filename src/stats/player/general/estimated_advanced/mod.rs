mod parameters;

pub use parameters::PlayersEstimatedAdvancedParameters;

crate::endpoint! {
    PlayersEstimatedAdvanced(PlayersEstimatedAdvancedParameters): "playerestimatedmetrics" => {
    } => {
        player_estimated_metrics: PlayerEstimatedMetricsRow("PlayerEstimatedMetrics") {
            player_id: u32,
            player_name: String,
            gp: u32,
            w: u32,
            l: u32,
            w_pct: f64,
            min: f64,
            e_off_rating: f64,
            e_def_rating: f64,
            e_net_rating: f64,
            e_ast_ratio: f64,
            e_oreb_pct: f64,
            e_dreb_pct: f64,
            e_reb_pct: f64,
            e_tov_pct: f64,
            e_usg_pct: f64,
            e_pace: f64,
            gp_rank: u32,
            w_rank: u32,
            l_rank: u32,
            w_pct_rank: u32,
            min_rank: u32,
            e_off_rating_rank: u32,
            e_def_rating_rank: u32,
            e_net_rating_rank: u32,
            e_ast_ratio_rank: u32,
            e_oreb_pct_rank: u32,
            e_dreb_pct_rank: u32,
            e_reb_pct_rank: u32,
            e_tov_pct_rank: u32,
            e_usg_pct_rank: u32,
            e_pace_rank: u32,
        },
    }
}
