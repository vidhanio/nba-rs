use serde::{Deserialize, Serialize};
use time::Date;

use crate::{
    fields::{
        Conference, Division, Half, LastNGames, LeagueId, Location, MeasureType, Month, Outcome,
        PerMode, Quarter, Season, SeasonSegment, SeasonType, Team, YesOrNo,
    },
    serde::{optional_date, serialize_none_as_empty_string},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct PlayersOpponentParameters {
    #[doc(hidden)]
    pub measure_type: MeasureType,

    pub per_mode: PerMode,

    pub plus_minus: YesOrNo,

    pub pace_adjust: YesOrNo,

    pub rank: YesOrNo,

    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub season: Season,

    pub season_type: SeasonType,

    #[serde(rename = "TeamID")]
    pub team_id: Team,

    #[serde(serialize_with = "serialize_none_as_empty_string")]
    pub outcome: Option<Outcome>,

    #[serde(serialize_with = "serialize_none_as_empty_string")]
    pub location: Option<Location>,

    pub month: Month,

    #[serde(serialize_with = "serialize_none_as_empty_string")]
    pub season_segment: Option<SeasonSegment>,

    #[serde(with = "optional_date")]
    pub date_from: Option<Date>,

    #[serde(with = "optional_date")]
    pub date_to: Option<Date>,

    #[serde(rename = "OpponentTeamID")]
    pub opponent_team_id: Team,

    #[serde(serialize_with = "serialize_none_as_empty_string")]
    pub vs_conference: Option<Conference>,

    #[serde(serialize_with = "serialize_none_as_empty_string")]
    pub vs_division: Option<Division>,

    #[serde(serialize_with = "serialize_none_as_empty_string")]
    pub game_segment: Option<Half>,

    pub period: Quarter,

    pub last_n_games: LastNGames,
}
