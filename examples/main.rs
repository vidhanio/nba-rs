use nba::{
    debug,
    stats::player::official_leaders::{fields::*, OfficialLeaders},
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let ep1 = OfficialLeaders {
        season: Season::From2018To19,
        season_type: SeasonType::PreSeason,
        per_mode: PerMode::Totals,
        stat_category: StatCategory::Points,
        scope: Scope::AllPlayers,
        ..Default::default()
    };

    let ep2 = OfficialLeaders {
        season: Season::From2018To19,
        season_type: SeasonType::RegularSeason,
        per_mode: PerMode::Per48Minutes,
        stat_category: StatCategory::Points,
        scope: Scope::AllPlayers,
        ..Default::default()
    };

    let ep3 = OfficialLeaders {
        season: Season::From2018To19,
        season_type: SeasonType::Playoffs,
        per_mode: PerMode::PerGame,
        stat_category: StatCategory::Points,
        scope: Scope::Rookies,
        ..Default::default()
    };

    debug(&ep1).await?;
    debug(&ep2).await?;
    debug(&ep3).await?;

    Ok(())
}
