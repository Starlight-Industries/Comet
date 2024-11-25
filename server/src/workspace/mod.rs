use std::path::PathBuf;

use anyhow::Result;
use anyhow::Context;
use anyhow::anyhow;

pub fn get_working_dir() -> Result<PathBuf> {
    let mut dir = dirs_next::data_dir()
        .ok_or(anyhow!("Expected working directory, found: None"))
        .context("Could not fetch working dir")?;
    dir.push("Comet/server");
    Ok(dir)
}

