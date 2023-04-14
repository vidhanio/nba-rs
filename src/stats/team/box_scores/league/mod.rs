mod parameters;

pub use parameters::LeagueBoxScoresParameters;

use crate::fields::PlayerOrTeam;

crate::endpoint! { "leaguegamelog"

    LeagueBoxScores(
        LeagueBoxScoresParameters {
            player_or_team: PlayerOrTeam::Team,
        }
    ) {
        league_game_log["LeagueGameLog"]: LeagueGameLogRow {
            season_id: String,
            team_id: u32,
            team_abbreviation: String,
            team_name: String,
            game_id: String,
            game_date: String,
            matchup: String,
            wl: String,
            min: u32,
            fgm: Option<u32>,
            fga: Option<u32>,
            fg_pct: Option<f64>,
            fg3m: Option<u32>,
            fg3a: Option<u32>,
            fg3_pct: Option<f64>,
            ftm: Option<u32>,
            fta: Option<u32>,
            ft_pct: Option<f64>,
            oreb: Option<u32>,
            dreb: Option<u32>,
            reb: Option<u32>,
            ast: Option<u32>,
            stl: Option<u32>,
            blk: Option<u32>,
            tov: Option<u32>,
            pf: Option<u32>,
            pts: u32,
            plus_minus: i32,
            video_available: u32,
        },
    }
}
