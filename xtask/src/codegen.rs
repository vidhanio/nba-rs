use std::{env, fs, path::PathBuf};

use clap::Parser;
use heck::{AsUpperCamelCase, ToSnakeCase, ToUpperCamelCase};
use nba_stats::{BasicEndpoint, BasicResultSet, Endpoint};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Item, Type};

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

impl From<&JsonType> for Type {
    fn from(ty: &JsonType) -> Self {
        match ty {
            JsonType::Unknown => parse_quote! { () },
            JsonType::Bool => parse_quote! { bool },
            JsonType::Int => parse_quote! { i32 },
            JsonType::Float => parse_quote! { f64 },
            JsonType::String => parse_quote! { String },
            JsonType::Array => parse_quote! { Vec<serde_json::Value> },
            JsonType::Object => parse_quote! { serde_json::Map<String, serde_json::Value> },
            JsonType::Optional(ty) => {
                let ty = Type::from(&**ty);
                parse_quote! { Option<#ty> }
            }
        }
    }
}

impl ToTokens for JsonType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Type::from(self).to_tokens(tokens)
    }
}

#[derive(Parser)]
pub struct Opts {
    #[arg(value_parser = crate::basic_endpoint)]
    endpoint: BasicEndpoint,
}

impl Opts {
    pub async fn run(self) -> color_eyre::Result<()> {
        let path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
            .parent()
            .unwrap()
            .join("crates")
            .join("nba-stats")
            .join("src")
            .join("endpoints")
            .join(&*self.endpoint.path())
            .with_extension("rs");

        let endpoint_path = self.endpoint.path();

        let output = prettyplease::unparse(&generate(self).await?);

        // if file does not exist, create and write to it.
        // if file exists, then print to stdout and exit.
        if path.exists() {
            eprintln!(
                "file '{}' already exists, printing to stdout instead",
                path.display()
            );

            println!("{output}");
        } else {
            eprintln!("writing to '{}'", path.display());

            fs::write(&path, output)?;
        }

        eprintln!("to test the endpoint, run:");
        eprintln!("    cargo test -p nba-stats endpoints::{endpoint_path} -- --nocapture",);

        Ok(())
    }
}

async fn generate(opts: Opts) -> color_eyre::Result<syn::File> {
    let resp = opts.endpoint.send_basic().await?;

    let endpoint_str = opts.endpoint.path();

    let mut items = Vec::<Item>::new();

    items.extend([
        parse_quote! {
            use serde::{Deserialize, Serialize};
        },
        parse_quote! {
            use crate::Endpoint;
        },
    ]);

    let endpoint_type = format_ident!("{}", endpoint_str.to_upper_camel_case());

    let result_set_attrs = resp.result_sets.iter().map(|BasicResultSet { name, .. }| {
        let field = format_ident!("{}", name.to_snake_case());
        let ty = format!("{}ResultSet", AsUpperCamelCase(name));

        quote! {
            #[endpoint(result_set(field = #field, ty = #ty, name = #name))]
        }
    });

    let parameters = resp.parameters.iter().map(|(key, val)| {
        let field = format_ident!("{}", key.to_snake_case());
        let ty = JsonType::from(val);

        quote!(pub #field: #ty)
    });

    items.push(parse_quote! {
        #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Endpoint)]
        #[endpoint(path = #endpoint_str)]
        #(#result_set_attrs)*
        #[serde(deny_unknown_fields, rename_all = "PascalCase")]
        pub struct #endpoint_type {
            #(#parameters,)*
        }
    });

    items.extend(resp.result_sets.iter().map(result_set_struct));

    items.push(parse_quote! {
        #[cfg(test)]
        mod tests {
            use claims::assert_ok;

            use super::*;

            #[tokio::test]
            async fn works() {
                println!(
                    "{:#?}",
                    assert_ok!(#endpoint_type::default().send().await)
                );
            }
        }
    });

    Ok(syn::File {
        shebang: None,
        attrs: Vec::new(),
        items,
    })
}

fn result_set_struct(result_set: &BasicResultSet) -> Item {
    let name = format_ident!("{}ResultSet", result_set.name.to_upper_camel_case());

    let fields = result_set.headers.iter().enumerate().map(|(i, header)| {
        let ty = result_set
            .row_set
            .iter()
            .map(|row| JsonType::from(&row[i]))
            .try_fold(JsonType::Unknown, JsonType::merge)
            .expect("merge should succeed");

        let field = format_ident!("{}", header.to_snake_case());

        quote! {
            pub #field: #ty,
        }
    });

    parse_quote! {
        #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
        #[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
        pub struct #name {
            #(#fields)*
        }
    }
}
