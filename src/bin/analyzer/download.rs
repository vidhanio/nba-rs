use std::time::Duration;

use anyhow::Result;
use futures::prelude::*;
use log::info;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::{nba_request, RESPONSES_DIR};

async fn get_response(endpoint: &str) -> Result<String> {
    nba_request(&endpoint.to_ascii_lowercase(), &[])
        .await?
        .text()
        .await
        .map_err(Into::into)
}

pub async fn download(endpoints: &[&str]) -> Result<()> {
    stream::iter(endpoints.iter().copied())
        .skip_while(
            |&endpoint| async move { fs::metadata(RESPONSES_DIR.join(endpoint)).await.is_ok() },
        )
        .then(|endpoint| async move {
            tokio::time::sleep(Duration::from_secs(1)).await;

            info!("getting endpoint: {}", endpoint);

            get_response(endpoint).await.map(|r| (endpoint, r))
        })
        .try_for_each(|(endpoint, response)| async move {
            info!("writing endpoint to file: {}", endpoint);

            let mut file = File::create(RESPONSES_DIR.join(endpoint)).await?;

            file.write_all(response.as_bytes())
                .await
                .map_err(Into::into)
        })
        .await
}
