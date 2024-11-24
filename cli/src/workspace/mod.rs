use std::fs;
#[allow(dead_code)]
use std::path::PathBuf;

use anyhow::{anyhow, Context, Ok, Result};

use crate::config::CometConfig;

pub fn get_working_dir() -> Result<PathBuf> {
    let mut dir = dirs_next::data_dir()
        .ok_or(anyhow!("Expected working directory, found: None"))
        .context("Could not fetch working dir")?;
    dir.push("Comet");
    Ok(dir)
}

pub fn get_data_dir() -> Result<PathBuf> {
    let mut dir = get_working_dir()?;
    dir.push("data");
    Ok(dir)
}

pub fn first_run() -> Result<()> {
    let working_dir = get_working_dir().context("Failed to obtain working directory")?;
    fs::create_dir_all(working_dir.as_path()).context("Failed to create working directory")?;
    
    let config = CometConfig::default();
    let contents = toml::to_string(&config).context("Failed to seralize Configuration to toml")?;
    let file_name = "comet.config.toml";
    fs::write(working_dir.as_path().join(file_name), contents)?;

    Ok(())
}