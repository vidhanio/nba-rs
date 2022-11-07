use nba::{stats::player::clutch::usage::PlayersClutchUsage, Endpoint};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut f = File::create("tmp/debug.txt").await?;

    let endpoint = PlayersClutchUsage::default();

    f.write_all(format!("{endpoint:#?}\n\n").as_bytes()).await?;

    f.write_all(format!("{:#?}\n\n", endpoint.to_request()).as_bytes())
        .await?;

    f.write_all(format!("{}\n\n", endpoint.send_raw().await?.text().await?).as_bytes())
        .await?;

    f.write_all(format!("{:#?}\n\n", endpoint.send_basic().await?).as_bytes())
        .await?;

    f.write_all(format!("{:#?}\n", endpoint.send().await?).as_bytes())
        .await?;

    f.flush().await?;

    Ok(())
}
