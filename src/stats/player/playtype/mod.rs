mod parameters;

pub use parameters::PlayersPlaytypeParameters;

use crate::stats::fields::PlayerOrTeam;

crate::endpoint! {
    PlayersPlaytype(PlayersPlaytypeParameters): "synergyplaytypes" => {
        player_or_team: PlayerOrTeam::Player,
    } => {
        synergy_play_type: SynergyPlayTypeRow("SynergyPlayType") {
            season_id: String,
            player_id: u32,
            player_name: String,
            team_id: u32,
            team_abbreviation: String,
            team_name: String,
            play_type: String,
            type_grouping: String,
            percentile: f64,
            gp: u32,
            poss_pct: f64,
            ppp: f64,
            fg_pct: f64,
            ft_poss_pct: f64,
            tov_poss_pct: f64,
            sf_poss_pct: f64,
            plusone_poss_pct: f64,
            score_poss_pct: f64,
            efg_pct: f64,
            poss: f64,
            pts: f64,
            fgm: f64,
            fga: f64,
            fgmx: f64,
        },
    }
}
