use std::convert::Infallible;

use serde::{Deserialize, Serialize};

use crate::{
    serde::serde_optional_infallible,
    stats::fields::{
        LeagueId, PerMode48 as PerMode, Scope, SeasonSince1946 as Season,
        SeasonTypeWithoutPlayIn as SeasonType, StatCategory,
    },
};

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
