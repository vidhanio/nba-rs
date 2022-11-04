use nba::{
    fields::{LeagueId, PerModeSimple, SeasonType},
    stats::{alltimeleadersgrids::AllTimeLeadersGrids, Endpoint},
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let req = AllTimeLeadersGrids {
        league_id: LeagueId::Nba,
        per_mode: PerModeSimple::Totals,
        season_type: SeasonType::RegularSeason,
        top_x: 10,
    };

    println!("{:#?}", req.send_basic().await?);
    println!("{:#?}", req.send().await?);

    Ok(())
}
