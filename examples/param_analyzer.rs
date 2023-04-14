use futures::TryStreamExt;
use heck::{AsPascalCase, ToPascalCase};
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let mut lines = LinesStream::new(BufReader::new(stdin()).lines());
    let header = lines.try_next().await?.unwrap();
    let name = AsPascalCase(header.split_once(' ').unwrap().1);

    let mut repr = false;
    let mut default_blank = false;
    let variants = lines
        .map_ok(|line| {
            let mut split = line.trim_matches('"').splitn(3, '|');
            let default = split.next().unwrap() == "DEFAULT";
            let name = split.next().unwrap().to_pascal_case();
            let rename = split.next().unwrap().to_owned();
            if rename.is_empty() {
                default_blank = true;
            }
            if rename.chars().all(char::is_numeric) {
                repr = true;
            }

            (default, name, rename)
        })
        .try_collect::<Vec<_>>()
        .await?;

    println!(
        "#[derive(Clone, Copy, Debug, {}PartialEq, Eq, Serialize{}, Deserialize{})]",
        if default_blank { "" } else { "Default, " },
        if repr { "_repr" } else { "" },
        if repr { "_repr" } else { "" },
    );
    println!("pub enum {name} {{",);
    for (default, name, rename) in variants {
        if !rename.is_empty() {
            if default {
                println!("    #[default]");
            }

            if !repr {
                println!("    #[serde(rename(deserialize = \"{rename}\"))]",);
            }

            println!(
                "    {name}{},\n",
                if repr {
                    format!(" = {rename}")
                } else {
                    String::new()
                }
            );
        }
    }
    println!("}}");

    Ok(())
}
