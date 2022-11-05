use nba::{
    fields::{LeagueId, PerModeSimple, SeasonType},
    stats::AllTimeLeadersGrids,
    Endpoint,
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let req = AllTimeLeadersGrids {
        league_id: LeagueId::GLeague,
        per_mode: PerModeSimple::Totals,
        season_type: SeasonType::RegularSeason,
        top_x: 10,
    };

    nba::debug(&req).await.map_err(Into::into)
}
