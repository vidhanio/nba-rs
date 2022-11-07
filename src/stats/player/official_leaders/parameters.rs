use std::convert::Infallible;

use super::fields::{
    serde_optional_infallible, LeagueId, PerMode, Scope, Season, SeasonType, StatCategory,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct OfficialLeadersParameters {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub season: Season,

    pub season_type: SeasonType,

    pub per_mode: PerMode,

    pub stat_category: StatCategory,

    pub scope: Scope,

    #[serde(with = "serde_optional_infallible")]
    pub active_flag: Option<Infallible>,
}
