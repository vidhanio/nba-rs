use std::io;

use nba::{
    fields::{Season, SeasonType},
    stats::team::box_scores::league::{
        LeagueBoxScores, LeagueBoxScoresParameters, LeagueGameLogRow,
    },
    Endpoint,
};

async fn league_box_scores(season_type: SeasonType) -> nba::Result<Vec<LeagueGameLogRow>> {
    LeagueBoxScores::new(LeagueBoxScoresParameters {
        season_type,
        season: Season(2022),
        ..Default::default()
    })
    .send()
    .await
    .map(|response| response.result_sets.league_game_log)
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let records = league_box_scores(SeasonType::RegularSeason)
        .await?
        .into_iter()
        .chain(league_box_scores(SeasonType::Playoffs).await?);

    let mut wtr = csv::Writer::from_writer(io::stdout());

    for record in records {
        wtr.serialize(record)?;
    }

    wtr.flush()?;

    Ok(())
}
