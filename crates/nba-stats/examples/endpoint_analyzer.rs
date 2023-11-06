use nba_stats::{stats::endpoint::BasicEndpoint, Endpoint};

fn json_type_name(val: serde_json::Value) -> &'static str {
    match val {
        serde_json::Value::Null => "Option<???>",
        serde_json::Value::Bool(_) => "bool",
        serde_json::Value::Number(n) => match n.as_i64() {
            Some(_) => "u32",
            None => "f64",
        },
        serde_json::Value::String(_) => "String",
        serde_json::Value::Array(_) => "Vec<???>",
        serde_json::Value::Object(_) => "serde_json::Map<???>",
    }
}

#[fncli::cli]
#[tokio::main]
async fn main(endpoint: String) -> color_eyre::Result<()> {
    color_eyre::install()?;

    let resp = BasicEndpoint::from_endpoint(endpoint.clone())
        .send_basic()
        .await?;

    resp.result_sets.into_iter().for_each(|rs| {
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
                    json_type_name(val)
                )
            });

        println!("        }},");
    });

    print!("    ");

    Ok(())
}
