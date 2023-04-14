use serde::{Deserialize, Serialize};
use time::Date;

use crate::{
    fields::{Direction, LeagueId, PlayerOrTeam, Season, SeasonType, Sorter},
    serde::serde_optional_date,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct LeagueBoxScoresParameters {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub season: Season,

    pub season_type: SeasonType,

    #[doc(hidden)]
    pub player_or_team: PlayerOrTeam,

    pub counter: u32,

    pub sorter: Sorter,

    pub direction: Direction,

    #[serde(with = "serde_optional_date")]
    pub date_from: Option<Date>,

    #[serde(with = "serde_optional_date")]
    pub date_to: Option<Date>,
}
