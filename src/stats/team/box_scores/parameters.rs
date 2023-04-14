use serde::{Deserialize, Serialize};
use time::Date;

use crate::{
    fields::{LeagueId, Season, SeasonType, Team},
    serde::serde_optional_date,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct BoxScoresParameters {
    #[serde(rename = "TeamID")]
    pub team_id: Team,

    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub season: Season,

    pub season_type: SeasonType,

    #[serde(with = "serde_optional_date")]
    pub date_from: Option<Date>,

    #[serde(with = "serde_optional_date")]
    pub date_to: Option<Date>,
}
