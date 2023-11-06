use nba::{
    fields::PerMode,
    stats::player::league_leaders::all_time::{AllTimeLeaders, AllTimeLeadersParameters},
    Endpoint,
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut totals = AllTimeLeaders::new(AllTimeLeadersParameters {
        per_mode: PerMode::Totals,
        ..Default::default()
    })
    .send()
    .await?
    .result_sets
    .league_leaders
    .into_iter()
    .map(|leader| {
        (
            leader.player_name.unwrap_or_else(|| "UNNAMED".into()),
            leader.pts as u32,
        )
    })
    .collect::<Vec<_>>();

    totals.sort_by(|(_, pts1), (_, pts2)| pts2.cmp(pts1));

    for (i, (player, pts)) in totals.into_iter().enumerate() {
        println!("{}. {} ({})", i + 1, player, pts);
    }

    Ok(())
}
