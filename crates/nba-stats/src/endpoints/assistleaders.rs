use serde::{Deserialize, Serialize};

use crate::{
    fields::{trait_param, LeagueId, PerMode, Player, Season, SeasonType, Team},
    Endpoint,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Endpoint)]
#[endpoint(path = "assistleaders")]
#[endpoint(result_set(field = assist_leaders, ty = "PT::ResultSet", name = "AssistLeaders"))]
#[serde(deny_unknown_fields, rename_all = "PascalCase", bound = "")]
pub struct AssistLeaders<PT: AssistLeadersPlayerOrTeam> {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,
    pub per_mode: PerMode,
    pub player_or_team: PT,
    pub season: Season,
    pub season_type: SeasonType,
}

trait_param! {
    AssistLeadersPlayerOrTeam {
        Player { AssistLeadersPlayerResultSet };
        Team { AssistLeadersTeamResultSet };
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AssistLeadersPlayerResultSet {
    pub rank: i32,
    pub player_id: i32,
    pub player: String,
    pub team_id: i32,
    pub team_abbreviation: String,
    pub team_name: String,
    pub jersey_num: String,
    pub player_position: String,
    pub ast: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AssistLeadersTeamResultSet {
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

    #[ignore = "don't want to spam the api"]
    #[tokio::test]
    async fn player_works() {
        println!(
            "{:#?}",
            assert_ok!(AssistLeaders::<Player>::default().send().await)
        );
    }

    #[ignore = "don't want to spam the api"]
    #[tokio::test]
    async fn team_works() {
        println!(
            "{:#?}",
            assert_ok!(AssistLeaders::<Team>::default().send().await)
        );
    }
}
