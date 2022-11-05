use nba::{
    fields::{LeagueId, PerModeSimple, PlayerOrTeam, SeasonType},
    stats::AssistLeaders,
    Endpoint,
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let req = AssistLeaders {
        league_id: LeagueId::Wnba,
        per_mode: PerModeSimple::PerGame,
        player_or_team: PlayerOrTeam::Player,
        season_type: SeasonType::RegularSeason,
        season: 2021,
    };

    nba::debug(&req).await.map_err(Into::into)
}
