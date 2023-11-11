mod codegen;
mod debug;

use clap::Parser;
use http::uri::PathAndQuery;
use nba_stats::BasicEndpoint;

#[derive(Parser)]
#[clap(name = "xtask")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Codegen(codegen::Opts),
    Debug(debug::Opts),
}

fn basic_endpoint(s: &str) -> Result<BasicEndpoint, String> {
    let path_and_query = s.parse::<PathAndQuery>().map_err(|e| e.to_string())?;
    let endpoint = path_and_query
        .path()
        .strip_prefix('/')
        .ok_or("missing leading `/`")?
        .to_owned();
    let query = path_and_query
        .query()
        .map(serde_urlencoded::from_str)
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    Ok(BasicEndpoint::new(endpoint, query))
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    match Opts::parse().subcmd {
        SubCommand::Codegen(opts) => opts.run().await,
        SubCommand::Debug(opts) => opts.run().await,
    }
}
