use serde::{Deserialize, Serialize};

use crate::{
    fields::{LeagueId, PerMode, PlayerOrTeam, Season, SeasonType},
    Endpoint,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Endpoint)]
#[endpoint(path = "assistleaders")]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
#[endpoint(row(field = assist_leaders, ty = "AssistLeadersRow", row = "AssistLeaders"))]
pub struct AssistLeaders {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,
    pub per_mode: PerMode,
    pub player_or_team: PlayerOrTeam,
    pub season: Season,
    pub season_type: SeasonType,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AssistLeadersRow {
    pub rank: i32,
    pub team_id: i32,
    pub team_abbreviation: String,
    pub team_name: String,
    pub ast: f64,
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;

    use super::*;

    #[tokio::test]
    async fn works() {
        println!("{:#?}", assert_ok!(AssistLeaders::default().send().await));
    }
}
