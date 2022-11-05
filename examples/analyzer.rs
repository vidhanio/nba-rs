use std::{env, fmt::Write};

use color_eyre::eyre::eyre;
use nba::BasicResponse;
use tokio::fs;

fn to_type(header: &str, val: serde_json::Value) -> &'static str {
    match header {
        "LEAGUE_ID" => "LeagueId",
        "is_active_flag" => "YesNo",
        _ => match val {
            serde_json::Value::Null => "Option<serde_json::Value>",
            serde_json::Value::Bool(_) => "bool",
            serde_json::Value::Number(n) => match n.as_i64() {
                Some(_) => "u32",
                None => "f64",
            },
            serde_json::Value::String(_) => "String",
            serde_json::Value::Array(_) => "Vec<serde_json::Value>",
            serde_json::Value::Object(_) => "serde_json::Map<String, serde_json::Value>",
        },
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let url = env::args().nth(1).ok_or(eyre!("args should have a url"))?;

    let resp = nba::CLIENT
        .get(url)
        .send()
        .await?
        .json::<BasicResponse>()
        .await?;

    let mut s = "\n".to_owned();

    Vec::from(resp.result_sets).into_iter().try_for_each(|rs| {
        if rs.row_set.is_empty() {
            return Ok(());
        }

        writeln!(
            s,
            "        {}: {}Row({:?}) {{",
            heck::AsSnakeCase(&rs.name),
            heck::AsUpperCamelCase(&rs.name),
            rs.name
        )?;

        rs.headers
            .into_iter()
            .zip(rs.row_set[0].clone())
            .try_for_each(|(header, val)| {
                writeln!(
                    s,
                    "            {}: {},",
                    header.to_lowercase(),
                    to_type(&header, val)
                )
            })?;

        writeln!(s, "        }},")
    })?;

    write!(s, "    ")?;

    fs::write("tmp/analysis.txt", s).await?;

    Ok(())
}
