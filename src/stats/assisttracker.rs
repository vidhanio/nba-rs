use crate::fields::{
    Conference, ConferenceOrDivision, Division, GameScope, LeagueId, Location, PerModeSimple,
    PlayerExperience, PlayerPosition, SeasonSegment, SeasonType, StarterOrBench, WinOrLoss,
};

crate::endpoint! {
    AssistTracker("assisttracker") {
        weight: Option<f64>,
        vs_division: Option<ConferenceOrDivision>,
        vs_conference: Option<Conference>,
        #[serde(rename = "TeamID")]
        team_id: Option<u32>,
        starter_bench: Option<StarterOrBench>,
        season_type: Option<SeasonType>,
        season_segment: Option<SeasonSegment>,
        #[serde(with = "crate::sd::season::optional")]
        season: Option<u32>,
        player_position: Option<PlayerPosition>,
        player_experience: Option<PlayerExperience>,
        per_mode: Option<PerModeSimple>,
        #[serde(rename = "PORound")]
        po_round: Option<u32>,
        outcome: Option<WinOrLoss>,
        #[serde(rename = "OpponentTeamID")]
        opponent_team_id: Option<u32>,
        month: Option<u32>,
        location: Option<Location>,
        #[serde(rename = "LeagueID")]
        league_id: Option<LeagueId>,
        last_n_games: Option<u32>,
        height: Option<u32>,
        game_scope: Option<GameScope>,
        draft_year: Option<u32>,
        draft_pick: Option<u32>,
        division: Option<Division>,
        date_to: Option<String>,
        date_from: Option<String>,
        country: Option<String>,
        conference: Option<Conference>,
        college: Option<String>,
    } => {
        assist_tracker: AssistTrackerRow("AssistTracker") {
            assists: u32,
        },
    }
}
