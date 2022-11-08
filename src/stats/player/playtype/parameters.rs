use serde::{Deserialize, Serialize};

use crate::{
    fields::{LeagueId, PerModeGame, PlayType, PlayerOrTeam, Season, SeasonType, TypeGrouping},
    serde::serde_player_or_team_char,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct PlayersPlaytypeParameters {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub season_year: Season,

    pub season_type: SeasonType,

    pub per_mode: PerModeGame,

    #[doc(hidden)]
    #[serde(with = "serde_player_or_team_char")]
    pub player_or_team: PlayerOrTeam,

    pub play_type: PlayType,

    pub type_grouping: TypeGrouping,
}
