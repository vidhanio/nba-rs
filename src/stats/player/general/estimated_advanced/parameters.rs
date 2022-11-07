use serde::{Deserialize, Serialize};

use crate::stats::fields::{LeagueId, Season, SeasonType};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct PlayersEstimatedAdvancedParameters {
    pub season: Season,

    pub season_type: SeasonType,

    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,
}
