use std::convert::Infallible;

use serde::{Deserialize, Serialize};

use crate::{
    fields::{LeagueId, PerMode48, Scope, Season, SeasonTypeWithoutPlayIn, StatCategory},
    serde::serde_optional_infallible,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct SeasonLeadersParameters {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub per_mode: PerMode48,

    pub stat_category: StatCategory,

    pub season: Season,

    pub season_type: SeasonTypeWithoutPlayIn,

    pub scope: Scope,

    #[serde(with = "serde_optional_infallible")]
    pub active_flag: Option<Infallible>,
}
