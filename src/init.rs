use std::{fs, path::Path};

use anyhow::Result;

pub fn init() -> Result<()> {
    match Path::new("content").try_exists()? {
        true => {
            fs::create_dir_all("content/recipe")?;
            fs::create_dir_all("content/plan")?;
            println!("Initialized recipe and plan directories.");
            Ok(())
        }
        false => Err(anyhow::anyhow!("Not a Hugo project. Make sure that the current directory contains a content directory.")),
    }
}
