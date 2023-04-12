mod parameters;

pub use parameters::AllTimeLeadersParameters;

crate::endpoint! { "leagueleaders"

    AllTimeLeaders(AllTimeLeadersParameters) {
        league_leaders["LeagueLeaders"]: LeagueLeadersRow {
            player_id: u32,
            player_name: Option<String>,
            gp: u32,
            min: Option<f64>,
            fgm: f64,
            fga: f64,
            fg_pct: f64,
            fg3m: Option<f64>,
            fg3a: Option<f64>,
            fg3_pct: Option<f64>,
            ftm: f64,
            fta: f64,
            ft_pct: f64,
            oreb: Option<f64>,
            dreb: Option<f64>,
            reb: Option<f64>,
            ast: f64,
            stl: Option<f64>,
            blk: Option<f64>,
            tov: Option<f64>,
            pf: f64,
            pts: f64,
            ast_tov: Option<f64>,
            stl_tov: Option<f64>,
            efg_pct: f64,
            ts_pct: f64,
            gp_rank: u32,
            min_rank: u32,
            fgm_rank: u32,
            fga_rank: u32,
            fg_pct_rank: u32,
            fg3m_rank: u32,
            fg3a_rank: u32,
            fg3_pct_rank: u32,
            ftm_rank: u32,
            fta_rank: u32,
            ft_pct_rank: u32,
            oreb_rank: u32,
            dreb_rank: u32,
            reb_rank: u32,
            ast_rank: u32,
            stl_rank: u32,
            blk_rank: u32,
            tov_rank: u32,
            pf_rank: u32,
            pts_rank: u32,
            ast_tov_rank: u32,
            stl_tov_rank: u32,
            efg_pct_rank: Option<u32>,
            ts_pct_rank: Option<u32>,
        },
    }
}
