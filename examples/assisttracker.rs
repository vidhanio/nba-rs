use nba::stats::assisttracker::AssistTracker;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let req = AssistTracker {
        date_from: Some("2020".to_string()),
        date_to: Some("2021".to_string()),
        ..Default::default()
    };

    nba::debug(&req).await.map_err(Into::into)
}
