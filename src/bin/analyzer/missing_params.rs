use anyhow::Result;
use futures::prelude::*;
use log::info;
use serde_json::Value;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use tokio_stream::wrappers::ReadDirStream;

use crate::{regex, JSON_DIR, PARAMS_NOT_FOUND_DIR, VALID_DIR};

pub fn split_params(response: &str) -> impl Iterator<Item = &str> {
    let re = regex!(r"(?P<param>[\w]+)(?: property)? is required");

    re.captures_iter(response).map(|c| {
        c.name("param")
            .expect("param should exist in match")
            .as_str()
    })
}

pub async fn missing_params() -> Result<()> {
    ReadDirStream::new(fs::read_dir(&*VALID_DIR).await?)
        .try_for_each(|entry| async move {
            let response = fs::read_to_string(entry.path()).await?;

            info!(
                "checking validity of parameters in response of endpoint: {}",
                entry.file_name().to_str().unwrap()
            );

            if serde_json::from_str::<Value>(&response).is_ok() {
                fs::copy(
                    entry.path(),
                    JSON_DIR.join(entry.file_name()).with_extension("json"),
                )
                .await
                .map(|_| ())
            } else {
                let f = File::create(PARAMS_NOT_FOUND_DIR.join(entry.file_name())).await?;

                futures::stream::iter(split_params(&response))
                    .map(Ok)
                    .try_fold(f, |mut f, param| async move {
                        f.write_all(param.as_bytes()).await?;
                        f.write_all(b"\n").await.map(|_| f)
                    })
                    .await
                    .map(|_| ())
            }
        })
        .await
        .map_err(Into::into)
}
