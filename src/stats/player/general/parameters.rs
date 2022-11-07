use std::convert::Infallible;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    serde::{serde_none_as_empty_string, serde_optional_date, serde_optional_infallible},
    stats::fields::{
        College, Conference, Country, Division, DraftPick, DraftYear, Experience, Half, Height,
        LastNGames, LeagueId, Location, MeasureType, Month, Outcome, PerModePlayersGeneral,
        PlayoffRound, Position, Quarter, SeasonSegment, SeasonSince1996, SeasonType,
        ShotClockRange, StarterOrBench, Team, Weight, YesOrNo,
    },
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct PlayersGeneralParameters {
    pub measure_type: MeasureType,

    pub per_mode: PerModePlayersGeneral,

    pub plus_minus: YesOrNo,

    pub pace_adjust: YesOrNo,

    pub rank: YesOrNo,

    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub season: SeasonSince1996,

    pub season_type: SeasonType,

    #[serde(rename = "PORound")]
    pub po_round: PlayoffRound,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub outcome: Option<Outcome>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub location: Option<Location>,

    pub month: Month,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub season_segment: Option<SeasonSegment>,

    #[serde(with = "serde_optional_date")]
    pub date_from: Option<NaiveDateTime>,

    #[serde(with = "serde_optional_date")]
    pub date_to: Option<NaiveDateTime>,

    #[serde(rename = "OpponentTeamID")]
    pub opponent_team_id: Team,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub vs_conference: Option<Conference>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub vs_division: Option<Division>,

    #[serde(rename = "TeamID")]
    pub team_id: Team,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub conference: Option<Conference>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub division: Option<Division>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub game_segment: Option<Half>,

    pub period: Quarter,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub shot_clock_range: Option<ShotClockRange>,

    pub last_n_games: LastNGames,

    #[serde(with = "serde_optional_infallible")]
    pub game_scope: Option<Infallible>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub player_experience: Option<Experience>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub player_position: Option<Position>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub starter_bench: Option<StarterOrBench>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub draft_year: Option<DraftYear>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub draft_pick: Option<DraftPick>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub college: Option<College>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub country: Option<Country>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub height: Option<Height>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub weight: Option<Weight>,

    #[serde(serialize_with = "serde_none_as_empty_string")]
    pub two_way: Option<YesOrNo>,
}
