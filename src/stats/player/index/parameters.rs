use std::convert::Infallible;

use serde::{Deserialize, Serialize};

use crate::{
    fields::{
        College, Country, DraftPick, DraftYear, Height, LeagueId, Position, Season, Team, Weight,
    },
    serde::serde_optional_infallible,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerIndexParameters {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub season: Season,

    pub historical: u32,

    #[serde(rename = "TeamID")]
    pub team_id: Team,

    pub country: Option<Country>,

    pub college: Option<College>,

    pub draft_year: Option<DraftYear>,

    pub draft_pick: Option<DraftPick>,

    pub player_position: Option<Position>,

    pub height: Option<Height>,

    pub weight: Option<Weight>,

    #[serde(with = "serde_optional_infallible")]
    pub active: Option<Infallible>,

    #[serde(with = "serde_optional_infallible")]
    pub all_star: Option<Infallible>,
}
