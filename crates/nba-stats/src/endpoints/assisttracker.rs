use serde::{Deserialize, Serialize};

use crate::Endpoint;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Endpoint)]
#[endpoint(path = "assisttracker")]
#[endpoint(
    result_set(
        field = assist_tracker,
        ty = "AssistTrackerResultSet",
        name = "AssistTracker"
    )
)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AssistTracker {
    pub college: Option<()>,
    pub conference: Option<()>,
    pub country: Option<()>,
    pub date_from: Option<()>,
    pub date_to: Option<()>,
    pub division: Option<()>,
    pub draft_pick: Option<()>,
    pub draft_year: Option<()>,
    pub game_scope: Option<()>,
    pub height: Option<()>,
    pub last_n_games: Option<()>,
    pub league_id: Option<()>,
    pub location: Option<()>,
    pub month: Option<()>,
    pub opponent_team_id: Option<()>,
    pub outcome: Option<()>,
    pub po_round: Option<()>,
    pub per_mode: Option<()>,
    pub player_experience: Option<()>,
    pub player_position: Option<()>,
    pub season: Option<()>,
    pub season_segment: Option<()>,
    pub season_type: Option<()>,
    pub starter_bench: Option<()>,
    pub team_id: Option<()>,
    pub vs_conference: Option<()>,
    pub vs_division: Option<()>,
    pub weight: Option<()>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AssistTrackerResultSet {
    pub assists: i32,
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;

    use super::*;
    #[tokio::test]
    async fn works() {
        println!("{:#?}", assert_ok!(AssistTracker::default().send().await));
    }
}
