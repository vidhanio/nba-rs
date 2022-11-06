use std::fmt::Write;

use heck::AsPascalCase;
use tokio::fs;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let input = fs::read_to_string("tmp/input.txt").await?;
    let mut lines = input.lines();
    let name = AsPascalCase(lines.next().unwrap().split_once(' ').unwrap().1);

    let mut default_blank = false;
    let variants = lines
        .map(|line| {
            let mut split = line.trim_matches('"').splitn(3, '|');
            let default = split.next().unwrap() == "DEFAULT";
            let name = AsPascalCase(split.next().unwrap());
            let rename = split.next().unwrap();
            if rename.is_empty() {
                default_blank = true;
            }
            (default, name, rename)
        })
        .collect::<Vec<_>>();

    let mut output = String::new();
    writeln!(
        output,
        "#[derive(Clone, Copy, Debug, {}PartialEq, Eq, Serialize, Deserialize)]",
        if default_blank { "" } else { "Default, " }
    )?;
    writeln!(output, "pub enum {name} {{",)?;
    for (default, name, rename) in variants {
        if !rename.is_empty() {
            if default {
                writeln!(output, "    #[default]")?;
            }
            writeln!(output, "    #[serde(rename = \"{rename}\")]",)?;
            writeln!(output, "    {name},",)?;
            writeln!(output)?;
        }
    }
    writeln!(output, "}}")?;

    println!("{output}");

    Ok(())
}
