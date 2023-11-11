use serde::{Deserialize, Serialize};

use crate::{
    fields::{GameId, Quarter},
    Endpoint,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Endpoint)]
#[endpoint(path = "playbyplayv2")]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
#[endpoint(result_set(field = play_by_play, ty = "PlayByPlayResultSet", name = "PlayByPlay"))]
#[endpoint(result_set(field = available_video, ty = "AvailableVideoResultSet", name = "AvailableVideo"))]
pub struct PlayByPlayV2 {
    #[serde(rename = "GameID")]
    pub game_id: GameId,
    pub start_period: Quarter,
    pub end_period: Quarter,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PlayByPlayResultSet {
    pub game_id: String,
    pub eventnum: i32,
    pub eventmsgtype: i32,
    pub eventmsgactiontype: i32,
    pub period: i32,
    pub wctimestring: String,
    pub pctimestring: String,
    pub homedescription: Option<String>,
    pub neutraldescription: Option<String>,
    pub visitordescription: Option<String>,
    pub score: Option<String>,
    pub scoremargin: Option<String>,
    pub person1type: i32,
    pub player1_id: i32,
    pub player1_name: Option<String>,
    pub player1_team_id: Option<i32>,
    pub player1_team_city: Option<String>,
    pub player1_team_nickname: Option<String>,
    pub player1_team_abbreviation: Option<String>,
    pub person2type: i32,
    pub player2_id: i32,
    pub player2_name: Option<String>,
    pub player2_team_id: Option<i32>,
    pub player2_team_city: Option<String>,
    pub player2_team_nickname: Option<String>,
    pub player2_team_abbreviation: Option<String>,
    pub person3type: i32,
    pub player3_id: i32,
    pub player3_name: Option<String>,
    pub player3_team_id: Option<i32>,
    pub player3_team_city: Option<String>,
    pub player3_team_nickname: Option<String>,
    pub player3_team_abbreviation: Option<String>,
    pub video_available_flag: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AvailableVideoResultSet {
    pub video_available_flag: i32,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unreadable_literal, clippy::zero_prefixed_literal)]

    use claims::assert_ok;

    use super::*;

    #[tokio::test]
    async fn works() {
        println!(
            "{:#?}",
            assert_ok!(
                PlayByPlayV2::default()
                    .with_game_id(GameId(0021700807))
                    .send()
                    .await
            )
        );
    }
}
