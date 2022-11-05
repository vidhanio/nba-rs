use crate::fields::{
    team::{NbaTeamAbbreviation, NbaTeamName},
    JerseyNumber, LeagueId, PerModeSimple, PerModeStatSimple, PlayerOrTeam, PlayerPosition,
    PlayerStat, SeasonType,
};

crate::endpoint! {
    AssistLeaders("assistleaders") {
        #[serde(rename = "LeagueID")]
        league_id: LeagueId,
        per_mode: PerModeSimple,
        player_or_team: PlayerOrTeam,
        season_type: SeasonType,
        #[serde(with = "crate::sd::season")]
        season: u32,
    } => {
        assist_leaders: AssistLeadersRow("AssistLeaders") {
            rank: u32,
            player_id: PlayerStat<u32>,
            player: PlayerStat<String>,
            team_id: u32,
            team_abbreviation: NbaTeamAbbreviation,
            team_name: NbaTeamName,
            jersey_num: PlayerStat<JerseyNumber>,
            player_position: PlayerStat<PlayerPosition>,
            ast: PerModeStatSimple,
        },
    }
}
