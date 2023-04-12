mod parameters;

pub use parameters::SeasonLeadersParameters;

crate::endpoint! { "leagueleaders"

    SeasonLeaders(SeasonLeadersParameters) {
        league_leaders["LeagueLeaders"]: LeagueLeadersRow {
            player_id: u32,
            rank: u32,
            player: String,
            team: String,
            gp: u32,
            min: f64,
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
            reb: f64,
            ast: f64,
            stl: Option<f64>,
            blk: Option<f64>,
            tov: Option<f64>,
            pf: Option<f64>,
            pts: f64,
            eff: f64,
            ast_tov: Option<f64>,
            stl_tov: Option<f64>,
        },
    }
}
