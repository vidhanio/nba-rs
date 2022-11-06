use nba::{debug, stats::player::general::traditional::PlayersTraditional, Endpoint};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let ep1 = PlayersTraditional::default();

    println!("{:#}", ep1.send_raw().await?.text().await?);

    debug(&ep1).await?;

    Ok(())
}
