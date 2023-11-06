use futures::{prelude::*, stream};
use itertools::Itertools;
use nba::{
    fields::SeasonType,
    live::playbyplay::PlayByPlay,
    stats::team::box_scores::league::{LeagueBoxScores, LeagueBoxScoresParameters},
    Endpoint,
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let box_scores = LeagueBoxScores::new(LeagueBoxScoresParameters {
        season_type: SeasonType::Playoffs,
        ..Default::default()
    })
    .send()
    .await?
    .result_sets
    .league_game_log
    .into_iter()
    .unique_by(|game| game.game_id.clone())
    .collect::<Vec<_>>();

    let len = box_scores.len();

    let (made, total) = stream::iter(box_scores)
        .enumerate()
        .then(|(i, row)| async move {
            eprint!("{} ({}/{})", row.game_id, i + 1, len);

            let pbp = PlayByPlay::send(&row.game_id).await?;

            let first_3pt_attempt = pbp
                .game
                .actions
                .iter()
                .find(|action| action.action_type == "3pt");

            let made = match first_3pt_attempt {
                Some(action) if action.shot_result.as_deref() == Some("Made") => 1,
                _ => 0,
            };

            eprint!(" - {}", if made == 1 { "Made" } else { "Missed" });

            Ok::<_, nba::Error>(made)
        })
        .try_fold((0, 0), |(made, total), shot| async move {
            let made = made + shot;
            let total = total + 1;

            eprintln!(" ({}%)", made as f64 / total as f64 * 100.0);

            Ok((made + shot, total + 1))
        })
        .await?;

    println!("{}%", made as f64 / total as f64 * 100.0);

    Ok(())
}
