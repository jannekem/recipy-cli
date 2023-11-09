use anyhow::Result;

use crate::parser::Recipe;

pub fn write_recipe(recipe: &Recipe, mut writer: impl std::io::Write) -> Result<()> {
    let yaml = serde_yaml::to_string(&recipe)?;
    // start front matter
    writer.write_all(b"---\n")?;
    // write yaml
    writer.write_all(yaml.as_bytes())?;
    // end front matter
    writer.write_all(b"---\n")?;
    Ok(())
}
