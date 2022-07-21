use anyhow::Result;
use futures::prelude::*;
use log::info;
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;

use crate::{INVALID_DIR, RESPONSES_DIR, VALID_DIR};

fn is_valid(response: &str) -> bool {
    !response.contains(r"<!DOCTYPE html>")
        && !response.contains(r#"{"Message":"An error has occurred."}"#)
        && !response.contains(r"Invalid parameter")
}

pub async fn filter_invalid() -> Result<()> {
    ReadDirStream::new(fs::read_dir(&*RESPONSES_DIR).await?)
        .try_for_each(|entry| async move {
            let response = fs::read_to_string(entry.path()).await?;

            info!(
                "checking validity of endpoint: {}",
                entry.file_name().to_str().unwrap()
            );

            if is_valid(&response) {
                fs::rename(entry.path(), VALID_DIR.join(entry.file_name())).await
            } else {
                fs::rename(entry.path(), INVALID_DIR.join(entry.file_name())).await
            }
        })
        .await
        .map_err(Into::into)
}
