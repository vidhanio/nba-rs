use serde::{Deserialize, Serialize};

use crate::stats::fields::{LeagueId, SeasonSince1996, SeasonType};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct PlayersEstimatedAdvancedParameters {
    pub season: SeasonSince1996,

    pub season_type: SeasonType,

    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,
}
