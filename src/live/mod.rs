use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    fields::{LeagueId, Quarter},
    serde::serde_status_code,
};

/// Today's scoreboard.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodaysScoreboard {
    /// The metadata for the scoreboard.
    meta: Meta,

    /// The scoreboard.
    scoreboard: Scoreboard,
}

/// The metadata for the scoreboard.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meta {
    /// The version of the scoreboard.
    version: i64,

    /// The URL of the request.
    request: String,

    /// The time of the request.
    time: String,

    /// The status code of the request.
    #[serde(with = "serde_status_code")]
    code: StatusCode,
}

/// The scoreboard.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scoreboard {
    /// The date.
    game_date: String,

    /// The league id.
    league_id: LeagueId,

    /// The league name.
    league_name: String,

    /// The games being played.
    games: Vec<Game>,
}

/// A game being played.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    /// The game id.
    game_id: String,

    /// The game code.
    game_code: String,

    /// The game status.
    game_status: i64,

    /// The game status text.
    game_status_text: String,

    /// The game period.
    period: Quarter,

    /// The game clock.
    // TODO: use `chrono` instead.
    game_clock: String,

    // TODO: use `chrono` instead.
    #[serde(rename = "gameTimeUTC")]
    game_time_utc: String,

    game_et: String,

    regulation_periods: i64,

    if_necessary: bool,

    series_game_number: String,

    series_text: String,

    home_team: Team,

    away_team: Team,

    game_leaders: GameLeaders,

    pb_odds: PbOdds,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "teamId")]
    team_id: i64,

    #[serde(rename = "teamName")]
    team_name: String,

    #[serde(rename = "teamCity")]
    team_city: String,

    #[serde(rename = "teamTricode")]
    team_tricode: String,

    #[serde(rename = "wins")]
    wins: i64,

    #[serde(rename = "losses")]
    losses: i64,

    #[serde(rename = "score")]
    score: i64,

    #[serde(rename = "seed")]
    seed: Option<serde_json::Value>,

    #[serde(rename = "inBonus")]
    in_bonus: Option<serde_json::Value>,

    #[serde(rename = "timeoutsRemaining")]
    timeouts_remaining: i64,

    #[serde(rename = "periods")]
    periods: Vec<Period>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Period {
    #[serde(rename = "period")]
    period: i64,

    #[serde(rename = "periodType")]
    period_type: PeriodType,

    #[serde(rename = "score")]
    score: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameLeaders {
    home_leaders: Leaders,

    away_leaders: Leaders,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Leaders {
    #[serde(rename = "personId")]
    person_id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "jerseyNum")]
    jersey_num: String,

    #[serde(rename = "position")]
    position: String,

    #[serde(rename = "teamTricode")]
    team_tricode: String,

    #[serde(rename = "playerSlug")]
    player_slug: Option<serde_json::Value>,

    #[serde(rename = "points")]
    points: i64,

    #[serde(rename = "rebounds")]
    rebounds: i64,

    #[serde(rename = "assists")]
    assists: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PbOdds {
    #[serde(rename = "team")]
    team: Option<serde_json::Value>,

    #[serde(rename = "odds")]
    odds: i64,

    #[serde(rename = "suspended")]
    suspended: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeriodType {
    #[default]
    #[serde(rename = "REGULAR")]
    Regular,
}
