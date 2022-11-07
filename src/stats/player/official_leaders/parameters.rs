use std::convert::Infallible;

use serde::{Deserialize, Serialize};

use crate::{
    serde::serde_optional_infallible,
    stats::fields::{
        LeagueId, PerMode48, Scope, SeasonSince1946, SeasonTypeWithoutPlayIn, StatCategory,
    },
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct OfficialLeadersParameters {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub per_mode: PerMode48,

    pub stat_category: StatCategory,

    pub season: SeasonSince1946,

    pub season_type: SeasonTypeWithoutPlayIn,

    pub scope: Scope,

    #[serde(with = "serde_optional_infallible")]
    pub active_flag: Option<Infallible>,
}
