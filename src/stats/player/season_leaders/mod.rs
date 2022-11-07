mod parameters;

pub use parameters::SeasonLeadersParameters;

crate::endpoint! {
    SeasonLeaders(SeasonLeadersParameters): "leagueleaders" => {
    } => {
        league_leaders: LeagueLeadersRow("LeagueLeaders") {
            player_id: u32,
            rank: u32,
            player: String,
            team: String,
            gp: u32,
            min: f64,
            fgm: f64,
            fga: f64,
            fg_pct: f64,
            fg3m: f64,
            fg3a: f64,
            fg3_pct: f64,
            ftm: f64,
            fta: f64,
            ft_pct: f64,
            oreb: f64,
            dreb: f64,
            reb: f64,
            ast: f64,
            stl: f64,
            blk: f64,
            tov: f64,
            pf: Option<f64>,
            pts: f64,
            eff: f64,
            ast_tov: Option<f64>,
            stl_tov: Option<f64>,
        },
    }
}
