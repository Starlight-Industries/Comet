use crate::config::CometConfig;
use anyhow::Context;
use anyhow::Result;
use libcomet::workspace::get_working_dir;
use std::fs;
pub fn first_run() -> Result<()> {
    let working_dir = get_working_dir().context("Failed to obtain working directory")?;
    fs::create_dir_all(working_dir.as_path()).context("Failed to create working directory")?;

    let config = CometConfig::default();
    let contents = toml::to_string(&config).context("Failed to seralize Configuration to toml")?;
    let file_name = "comet.config.toml";
    fs::write(working_dir.as_path().join(file_name), contents)?;

    Ok(())
}
