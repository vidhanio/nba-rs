#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::collections::HashMap;

use anyhow::Result;
use futures::prelude::*;
use nba_analysis::{
    analyzers::{self, DataSource},
    consts::{ANALYSIS_JSON, ENDPOINTS},
};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // let hm = analyzers::deserialize_from(DataSource::EndpointList(ENDPOINTS))
    let hm = analyzers::deserialize_from(DataSource::JsonFilepath(&ANALYSIS_JSON))
        .await
        .map_ok(analyzers::deprecation)
        .map_ok(analyzers::params::from_json)
        .map_ok(analyzers::params::required)
        .map_ok(|analysis| (analysis.endpoint().to_owned(), analysis))
        .try_collect::<HashMap<_, _>>()
        .await?;

    fs::write(&*ANALYSIS_JSON, serde_json::to_string_pretty(&hm)?).await?;

    Ok(())
}
