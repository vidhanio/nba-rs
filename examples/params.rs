use std::fmt::Write;

use heck::AsPascalCase;
use tokio::fs;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let input = fs::read_to_string("tmp/input.txt").await?;
    let mut lines = input.lines();
    let name = AsPascalCase(lines.next().unwrap().split_once(' ').unwrap().1);
    let variants = lines
        .skip(1)
        .take_while(|l| l.starts_with(char::is_numeric))
        .map(|l| {
            let (_, l) = l.split_once(' ').unwrap();
            let mut parts = l
                .trim_matches(|c| c == '[' || c == ']')
                .splitn(3, ", ")
                .map(|s| s.trim_matches('"'));
            let default = parts.next().unwrap() == "DEFAULT";
            let name = AsPascalCase(parts.next().unwrap());
            let rename = parts.next().unwrap();
            let rename = &rename[..rename.len() - 6];
            (default, name, rename)
        });

    let mut output = String::new();
    writeln!(
        output,
        "#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]"
    )?;
    writeln!(output, "pub enum {name} {{",)?;
    for (default, name, rename) in variants {
        if default {
            writeln!(output, "    #[default]")?;
        }
        writeln!(output, "    #[serde(rename = \"{rename}\")]",)?;
        writeln!(output, "    {name},",)?;
        writeln!(output)?;
    }
    writeln!(output, "}}")?;

    fs::write("tmp/params.txt", output).await?;

    Ok(())
}
