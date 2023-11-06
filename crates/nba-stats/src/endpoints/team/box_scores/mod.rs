pub mod league;
mod parameters;

pub use parameters::BoxScoresParameters;

crate::endpoint! { "teamgamelog"

    BoxScores(BoxScoresParameters) {
        team_game_log["TeamGameLog"]: TeamGameLogRow {
            #[serde(rename(deserialize = "Team_ID"))]
            team_id: u32,

            #[serde(rename(deserialize = "Game_ID"))]
            game_id: String,

            game_date: String,
            matchup: String,
            wl: String,
            w: u32,
            l: u32,
            w_pct: f64,
            min: u32,
            fgm: u32,
            fga: u32,
            fg_pct: f64,
            fg3m: u32,
            fg3a: u32,
            fg3_pct: f64,
            ftm: u32,
            fta: u32,
            ft_pct: f64,
            oreb: u32,
            dreb: u32,
            reb: u32,
            ast: u32,
            stl: u32,
            blk: u32,
            tov: u32,
            pf: u32,
            pts: u32,
        },
    }
}
