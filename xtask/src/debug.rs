use std::{
    env, fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::{Parser, ValueEnum};
use color_eyre::eyre;
use nba_stats::{BasicEndpoint, Endpoint};

#[derive(Parser)]
pub struct Opts {
    #[clap(short, long = "type", default_value = "basic")]
    #[arg(value_enum)]
    type_: ResponseType,

    #[arg(value_parser = crate::basic_endpoint)]
    endpoint: BasicEndpoint,
}

impl Opts {
    pub async fn run(self) -> color_eyre::Result<()> {
        let (ext, contents) = match self.type_ {
            ResponseType::Basic => ("rs", format!("{:#?}", self.endpoint.send_basic().await?)),
            ResponseType::Map => ("rs", format!("{:#?}", self.endpoint.send().await?)),
            ResponseType::Json => ("json", self.endpoint.send_raw().await?.text().await?),
        };

        let temp_dir = env::temp_dir();

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let path = temp_dir
            .join(format!("nba-stats-debug-{timestamp}"))
            .with_extension(ext);

        fs::write(&path, contents)?;

        let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".into());

        let status = Command::new(editor).arg(&path).status()?;

        if status.success() {
            Ok(())
        } else {
            Err(eyre::eyre!("editor exited with status {status}"))
        }
    }
}

#[derive(Copy, Clone, ValueEnum)]
enum ResponseType {
    Basic,
    Map,
    Json,
}
