use std::fmt::{self, Display, Formatter};

use http::uri::PathAndQuery;
use nba_stats::{BasicEndpoint, Endpoint};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum JsonType {
    #[default]
    Unknown,
    Bool,
    Int,
    Float,
    String,
    Array,
    Object,
    Optional(Box<JsonType>),
}

impl JsonType {
    fn merge(self, other: Self) -> Option<Self> {
        match (self, other) {
            (s @ Self::Bool, Self::Bool)
            | (s @ Self::Int, Self::Int)
            | (s @ Self::Float, Self::Float)
            | (s @ Self::String, Self::String)
            | (s @ Self::Array, Self::Array)
            | (s @ Self::Object, Self::Object) => Some(s),
            (Self::Unknown, real) | (real, Self::Unknown) => Some(real),
            (Self::Int, Self::Float) | (Self::Float, Self::Int) => Some(Self::Float),
            (Self::Optional(v), Self::Optional(v2)) => {
                v.merge(*v2).map(|v| Self::Optional(Box::new(v)))
            }
            (Self::Optional(v), real) | (real, Self::Optional(v)) => {
                v.merge(real).map(|v| Self::Optional(Box::new(v)))
            }
            _ => None,
        }
    }
}

impl From<&serde_json::Value> for JsonType {
    fn from(val: &serde_json::Value) -> Self {
        match val {
            serde_json::Value::Null => Self::Optional(Box::new(Self::Unknown)),
            serde_json::Value::Bool(_) => Self::Bool,
            serde_json::Value::Number(n) => match n.as_i64() {
                Some(_) => Self::Int,
                None => Self::Float,
            },
            serde_json::Value::String(_) => Self::String,
            serde_json::Value::Array(_) => Self::Array,
            serde_json::Value::Object(_) => Self::Object,
        }
    }
}

impl Display for JsonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JsonType::Unknown => write!(f, "()"),
            JsonType::Bool => write!(f, "bool"),
            JsonType::Int => write!(f, "i32"),
            JsonType::Float => write!(f, "f64"),
            JsonType::String => write!(f, "String"),
            JsonType::Array => write!(f, "Vec<serde_json::Value>"),
            JsonType::Object => write!(f, "serde_json::Map<String, serde_json::Value>"),
            JsonType::Optional(ty) => write!(f, "Option<{ty}>"),
        }
    }
}

fn write_row_struct(name: &str, headers: &[String], rows: &[Vec<serde_json::Value>]) {
    println!("#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]");
    println!("#[serde(deny_unknown_fields, rename_all = \"SCREAMING_SNAKE_CASE\")]");
    println!("pub struct {}Row {{", heck::AsUpperCamelCase(name));

    headers.iter().enumerate().for_each(|(i, header)| {
        let ty = rows
            .iter()
            .map(|row| JsonType::from(&row[i]))
            .try_fold(JsonType::Unknown, JsonType::merge)
            .expect("merge should succeed");

        println!("    pub {}: {ty},", heck::AsSnakeCase(header));
    });

    println!("}}");
}

#[fncli::cli]
#[tokio::main]
async fn main(endpoint: PathAndQuery) -> color_eyre::Result<()> {
    color_eyre::install()?;

    let endpoint_str = &endpoint.path()[1..];
    let endpoint_params = endpoint
        .query()
        .map(serde_urlencoded::from_str)
        .transpose()?
        .unwrap_or_default();

    let resp = BasicEndpoint::new(endpoint_str.into(), endpoint_params)
        .send_basic()
        .await?;

    println!("use crate::Endpoint;");
    println!("use serde::{{Deserialize, Serialize}};");

    println!();

    println!(
        "#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Endpoint)]"
    );
    println!("#[endpoint(path = {endpoint_str:?})]");
    println!("#[serde(deny_unknown_fields, rename_all = \"PascalCase\")]");

    resp.result_sets.iter().for_each(|row| {
        println!(
            "#[endpoint(row(field = {}, ty = \"{}Row\", row = \"{}\"))]",
            heck::AsSnakeCase(&row.name),
            heck::AsUpperCamelCase(&row.name),
            row.name,
        );
    });

    println!("pub struct {} {{", heck::AsUpperCamelCase(&endpoint_str));

    resp.parameters.iter().for_each(|(key, val)| {
        println!(
            "    pub {}: {},",
            heck::AsSnakeCase(key),
            JsonType::from(val)
        );
    });

    println!("}}");

    resp.result_sets.iter().for_each(|row| {
        println!();
        write_row_struct(&row.name, &row.headers, &row.row_set);
    });

    println!();

    // #[cfg(test)]
    // mod tests {
    //     use claims::assert_ok;

    //     use super::*;
    //     use crate::Endpoint;

    //     #[tokio::test]
    //     async fn works() {
    //         assert_ok!(AssistLeaders::default().send().await);
    //     }
    // }

    println!("#[cfg(test)]");
    println!("mod tests {{");
    println!("    use claims::assert_ok;");
    println!();
    println!("    use super::*;");
    println!();
    println!("    #[tokio::test]");
    println!("    async fn works() {{");
    println!(
        "        assert_ok!({}::default().send().await);",
        heck::AsUpperCamelCase(&endpoint_str)
    );
    println!("    }}");
    println!("}}");

    Ok(())
}
