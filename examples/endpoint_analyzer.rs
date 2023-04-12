use nba::BasicResponse;

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

#[fncli::cli]
#[tokio::main]
async fn main(url: String) -> color_eyre::Result<()> {
    color_eyre::install()?;

    let resp = nba::CLIENT
        .get(url)
        .send()
        .await?
        .json::<BasicResponse>()
        .await?;

    println!();

    resp.result_sets.into_vec().into_iter().for_each(|rs| {
        if rs.row_set.is_empty() {
            return;
        }

        println!(
            "        {}[{:?}]: {}Row {{",
            heck::AsSnakeCase(&rs.name),
            rs.name,
            heck::AsUpperCamelCase(&rs.name),
        );

        rs.headers
            .into_iter()
            .zip(rs.row_set[0].clone())
            .for_each(|(header, val)| {
                println!(
                    "            {}: {},",
                    header.to_lowercase(),
                    to_type(&header, val)
                )
            });

        println!("        }},");
    });

    print!("    ");

    Ok(())
}
