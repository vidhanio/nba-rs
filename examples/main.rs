use nba::{
    debug,
    stats::player::{official_leaders::fields::*, OfficialLeaders},
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let official_leaders = OfficialLeaders::default();

    debug(&official_leaders).await.map_err(Into::into)
}
