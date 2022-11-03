use std::env;

use anyhow::{anyhow, Result};
use nba::NbaClient;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = env::args();

    let endpoint = args.nth(1).ok_or_else(|| anyhow!("missing endpoint"))?;
    let params = args
        .map(|s| s.split_once('=').map(|(k, v)| (k.to_owned(), v.to_owned())))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| anyhow!("invalid params"))?;

    let client = NbaClient::new();

    eprintln!("endpoint: {endpoint:#?}");
    eprintln!("params: {params:#?}");

    let text = client
        .send_request(
            &endpoint,
            params.iter().map(|(k, v)| (k.as_str(), v.as_str())),
        )
        .await?
        .text()
        .await?;

    println!("{text}");

    Ok(())
}
