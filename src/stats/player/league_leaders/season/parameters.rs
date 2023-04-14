use serde::{Deserialize, Serialize};

use crate::fields::{ActiveFlag, LeagueId, PerMode, Scope, Season, SeasonType, StatCategory};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct SeasonLeadersParameters {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub per_mode: PerMode,

    pub stat_category: StatCategory,

    pub season: Season,

    pub season_type: SeasonType,

    pub scope: Scope,

    pub active_flag: Option<ActiveFlag>,
}
