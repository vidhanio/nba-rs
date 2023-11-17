//! Play-by-play data for a game.

mod clock;
mod game_id;
mod period_type;

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub use self::{
    action_types::{ThreePointShot, TwoPointShot},
    clock::Clock,
    game_id::GameId,
    period_type::PeriodType,
};
use super::Meta;

/// Play-by-play data for a game.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayByPlay {
    /// Metadata about the API response.
    pub meta: Meta,

    /// The game.
    pub game: Game,
}

impl PlayByPlay {
    /// Get the URL for the play-by-play data for a game.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn url(game_id: GameId) -> Url {
        format!("https://cdn.nba.com/static/json/liveData/playbyplay/playbyplay_{game_id}.json")
            .parse()
            .expect("static url should be well-formed")
    }

    /// Get the play-by-play data for a game.
    ///
    /// # Errors
    ///
    /// This function will return an error if the API request fails or if the
    /// response body cannot be parsed.
    pub async fn get(client: &Client, game_id: GameId) -> crate::Result<Self> {
        client
            .get(Self::url(game_id))
            .send()
            .await?
            .error_for_status()?
            .json::<Self>()
            .await
            .map_err(Into::into)
    }
}

/// A game.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    /// The game's ID.
    pub game_id: GameId,

    /// The actions in the game.
    pub actions: Vec<Action>,
}

/// An action in a game.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "actionType")]
pub struct Action {
    /// The action's number.
    pub action_number: u32,

    /// The remaining time in the period.
    pub clock: Clock,

    /// The actual time of the action.
    #[serde(with = "time::serde::iso8601")]
    pub time_actual: OffsetDateTime,

    /// The period. `1` is the first quarter, `2` is the second quarter, etc.
    /// `5` would be the first overtime period.
    pub period: u16,

    /// The type of period.
    pub period_type: PeriodType,

    /// The qualifiers associated with the action.
    pub qualifiers: Vec<String>,

    /// The score for the home team.
    #[serde(with = "crate::serde::string_u32")]
    pub score_home: u32,

    /// The score for the away team.
    #[serde(with = "crate::serde::string_u32")]
    pub score_away: u32,

    /// The time at which the action was edited.
    #[serde(with = "time::serde::iso8601")]
    pub edited: OffsetDateTime,

    /// The order number of the action.
    pub order_number: u32,

    /// Whether the action occurred in the last period of an elam ending game.
    pub is_target_score_last_period: bool,

    /// The description of the action.
    pub description: String,

    /// The distinct action type.
    #[serde(flatten)]
    pub action_type: ActionType,
}

/// The distinct action type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "actionType")]
pub enum ActionType {
    /// A two-point shot.
    #[serde(rename = "2pt")]
    TwoPointShot(TwoPointShot),

    /// A three-point shot.
    #[serde(rename = "3pt")]
    ThreePointShot(ThreePointShot),
    // Block(Block),
    // Foul(Foul),
    // FreeThresult_set(FreeThrow),
    // Game(Game),
    // JumpBall(JumpBall),
    // Period(Period),
    // Rebound(Rebound),
    // Steal(Steal),
    // Substitution(Substitution),
    // Timeout(Timeout),
    // Turnover(Turnover),
    // Violation(Violation),
    /// TODO: Remove once all action types are implemented.
    #[serde(other)]
    Other,
}

mod action_types {

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TwoPointShot {
        pub team_id: u64,
        pub team_tricode: String,
        pub sub_type: String,
        pub descriptor: Option<String>,
        pub person_id: u32,
        pub x: f64,
        pub y: f64,
        pub area: String,
        pub area_detail: String,
        pub side: String,
        pub shot_distance: f64,
        pub possession: u64,
        pub x_legacy: i32,
        pub y_legacy: i32,
        pub shot_result: String,
        pub player_name: String,
        pub player_name_i: String,
        pub person_ids_filter: Vec<u32>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ThreePointShot {
        pub team_id: u64,
        pub team_tricode: String,
        pub sub_type: String,
        pub descriptor: Option<String>,
        pub person_id: u32,
        pub x: f64,
        pub y: f64,
        pub area: String,
        pub area_detail: String,
        pub side: String,
        pub shot_distance: f64,
        pub possession: u64,
        pub x_legacy: i32,
        pub y_legacy: i32,
        pub shot_result: String,
        pub player_name: String,
        pub player_name_i: String,
        pub person_ids_filter: Vec<u32>,
    }
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;

    use super::*;

    #[test]
    fn works() {
        let json = include_str!("../../tests/data/playbyplay.json");

        assert_ok!(serde_json::from_str::<PlayByPlay>(json));
    }
}
