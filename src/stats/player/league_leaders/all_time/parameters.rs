use serde::{Deserialize, Serialize};

use crate::fields::{
    ActiveFlag, AllTime, LeagueId, PerModeGame, Scope, SeasonTypeBasicWithPreseason, StatCategory,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct AllTimeLeadersParameters {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub per_mode: PerModeGame,

    pub stat_category: StatCategory,

    #[doc(hidden)]
    pub season: AllTime,

    pub season_type: SeasonTypeBasicWithPreseason,

    pub scope: Scope,

    pub active_flag: ActiveFlag,
}
