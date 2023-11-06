use serde::{Deserialize, Serialize};

use crate::{
    fields::{LeagueId, PerMode, PlayType, PlayerOrTeam, Season, SeasonType, TypeGrouping},
    serde::player_or_team_char,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct PlayersPlaytypeParameters {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub season_year: Season,

    pub season_type: SeasonType,

    pub per_mode: PerMode,

    #[doc(hidden)]
    #[serde(with = "player_or_team_char")]
    pub player_or_team: PlayerOrTeam,

    pub play_type: PlayType,

    pub type_grouping: TypeGrouping,
}
