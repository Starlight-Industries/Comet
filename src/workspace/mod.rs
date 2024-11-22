use std::path::PathBuf;

use anyhow::{anyhow, Context, Ok, Result};

pub fn get_working_dir() -> Result<PathBuf> {
    let mut dir = dirs_next::data_dir()
        .ok_or(anyhow!("Expected working directory, found: None"))
        .context("Could not fetch working dir")?;
    dir.push("Comet");
    Ok(dir)
}