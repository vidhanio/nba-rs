use serde::{Deserialize, Serialize};

use crate::fields::{
    ActiveFlag, LeagueId, PerMode48, Scope, Season, SeasonTypeWithoutPlayIn, StatCategorySimple,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct SeasonLeadersParameters {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub per_mode: PerMode48,

    pub stat_category: StatCategorySimple,

    pub season: Season,

    pub season_type: SeasonTypeWithoutPlayIn,

    pub scope: Scope,

    pub active_flag: Option<ActiveFlag>,
}
