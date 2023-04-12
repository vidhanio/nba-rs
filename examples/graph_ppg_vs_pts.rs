use std::{cmp::Ordering, collections::HashMap, io::Cursor};

use futures::{prelude::*, stream};
use image::{imageops::FilterType, DynamicImage, ImageFormat, Rgba};
use nba::{
    fields::PerMode48,
    stats::player::league_leaders::season::{
        LeagueLeadersRow, SeasonLeaders, SeasonLeadersParameters,
    },
    Endpoint,
};
use plotters::prelude::*;

const OUT_FILE_NAME: &str = "tmp/graphs.png";

async fn head_image(
    player_id: u32,
    (ppg, total_pts): (f64, u32),
) -> color_eyre::Result<BitMapElement<'static, (f64, u32)>> {
    eprintln!("Downloading image for player {player_id}...");

    let bytes = reqwest::get(format!(
        "https://cdn.nba.com/headshots/nba/latest/1040x760/{player_id}.png"
    ))
    .await?
    .bytes()
    .await?;

    let mut image = image::load(Cursor::new(bytes), ImageFormat::Png)?
        .resize(128, 128, FilterType::Nearest)
        .into_rgba8();

    let (w, h) = image.dimensions();
    image.pixels_mut().for_each(|pixel| {
        if pixel[3] == 0 {
            *pixel = Rgba([255, 255, 255, 255]);
        }
    });
    let rgb_image = DynamicImage::ImageRgba8(image).into_rgb8();

    let bitmap = BitMapElement::with_owned_buffer((ppg, total_pts), (w, h), rgb_image.into_raw())
        .expect("bitmap element should be created");

    Ok(bitmap)
}

async fn ppg_leaders() -> color_eyre::Result<Vec<LeagueLeadersRow>> {
    Ok(SeasonLeaders::new(SeasonLeadersParameters {
        per_mode: PerMode48::PerGame,
        ..Default::default()
    })
    .send()
    .await?
    .result_sets
    .league_leaders)
}

async fn total_pts_leaders() -> color_eyre::Result<Vec<LeagueLeadersRow>> {
    Ok(SeasonLeaders::new(SeasonLeadersParameters {
        per_mode: PerMode48::Totals,
        ..Default::default()
    })
    .send()
    .await?
    .result_sets
    .league_leaders)
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (1920, 1080)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Scoring Champion?", ("sans-serif", 50.0))
        .build_cartesian_2d(20f64..35f64, 1500u32..2300u32)?;

    chart
        .configure_mesh()
        .disable_mesh()
        .y_desc("Total Points")
        .x_desc("Points Per Game")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    let total_pts_map = total_pts_leaders()
        .await?
        .into_iter()
        .map(|leader| (leader.player_id, leader))
        .collect::<HashMap<_, _>>();

    let mut data = ppg_leaders()
        .await?
        .into_iter()
        .filter_map(|leader| {
            let total_pts = total_pts_map.get(&leader.player_id)?;

            Some((leader.player_id, (leader.pts, total_pts.pts as u32)))
        })
        .collect::<Vec<_>>();

    data.sort_by(|(_, (pts1, _)), (_, (pts2, _))| {
        pts2.partial_cmp(pts1).unwrap_or(Ordering::Equal)
    });

    let series = stream::iter(
        data.into_iter()
            .filter(|&(_, (ppg, pts))| ppg > 20. && pts > 1550),
    )
    .then(|(player_id, stats)| async move { head_image(player_id, stats).await })
    .try_collect::<Vec<_>>()
    .await?;

    chart.draw_series(series)?;

    Ok(())
}
