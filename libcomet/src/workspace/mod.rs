#[allow(dead_code)]
use std::path::PathBuf;

use anyhow::{Context, Ok, Result, anyhow};

pub fn get_working_dir() -> Result<PathBuf> {
    let mut dir = dirs_next::data_dir()
        .ok_or(anyhow!("Expected working directory, found: None"))
        .context("Could not fetch working dir")?;
    dir.push("Comet");
    Ok(dir)
}

pub fn get_data_dir() -> Result<PathBuf> {
    let mut dir = get_working_dir().context("Failed to obtain working dir")?;
    dir.push("data");
    Ok(dir)
}
pub fn get_server_dir() -> Result<PathBuf> {
    let mut dir = get_working_dir().context("Failed to obtain server dir")?;
    dir.push("server");
    Ok(dir)
}
