use serde::{Deserialize, Serialize};

use crate::stats::fields::{
    LeagueId, PerModeGame, PlayType, PlayerOrTeamChar, SeasonSince2015, SeasonType, TypeGrouping,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct PlayersPlaytypeParameters {
    #[serde(rename = "LeagueID")]
    pub league_id: LeagueId,

    pub season_year: SeasonSince2015,

    pub season_type: SeasonType,

    pub per_mode: PerModeGame,

    #[doc(hidden)]
    pub player_or_team: PlayerOrTeamChar,

    pub play_type: PlayType,

    pub type_grouping: TypeGrouping,
}
