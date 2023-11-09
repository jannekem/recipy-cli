use std::fs::OpenOptions;

use anyhow::{Context, Result};
use clap::Parser;

mod cli;
mod parser;
mod writer;
use cli::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();
    let body = reqwest::blocking::get(&args.url)
        .context("Failed to fetch URL")?
        .text()?;
    let mut recipe = parser::parse_recipe(&body)?;

    recipe.favorite = args.favorite;
    recipe.temperature = args.temperature.unwrap_or_default();

    let mut output: Box<dyn std::io::Write> = if args.stdout {
        Box::new(std::io::stdout())
    } else {
        let file_path = format!(
            "content/recipe/{}",
            args.filename.unwrap_or_else(|| recipe.filename())
        );
        let file = OpenOptions::new()
            .write(true)
            .create(args.force)
            .create_new(!args.force)
            .open(&file_path);
        match file {
            Ok(f) => Box::new(std::io::BufWriter::new(f)),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    return Err(anyhow::anyhow!(
                        "Recipe already exists, use --force to overwrite."
                    ));
                }
                return Err(anyhow::anyhow!("Could not create the recipe file. Make sure that this is a Hugo project with a content/recipe directory."));
            }
        }
    };

    writer::write_recipe(&recipe, &mut output).context("Failed to write recipe.")?;
    Ok(())
}
