use std::{path::PathBuf, str::FromStr};

use crate::workspace::{self, ServerConfig};
use anyhow::{Context, Ok, Result};
use inquire::{Confirm, MultiSelect, Select, Text};
use libcomet::{
    package::{Architecture, ArchitectureType, PackageState},
    workspace::get_server_dir,
};
use log::debug;
use url::Url;

pub fn create_config_interactive() -> Result<ServerConfig> {
    let repo_name = Text::new("What would you like your repo name to be?")
        .with_help_message("Can be anything!")
        .prompt()?;
    let uid = Text::new("What would you like your repo's UID to be?")
        .with_help_message("Should be a reverse dns url, eg. io.github.john.myrepo")
        .prompt()?;
    let port: u16 = Text::new("Enter the port number you would like the server to run on:")
        .with_default("8000")
        .prompt()?
        .parse()?;
    let repo_type = MultiSelect::new(
        "Choose the types of packages you would like this repo to provide:",
        PackageState::iter().collect(),
    )
    .with_default(&[2])
    .prompt()?;
    let architecture = MultiSelect::new(
        "Select the architectures you would like your repository to support",
        ArchitectureType::iter().collect(),
    )
    .prompt()?;
    let is_32_bit = Confirm::new("Would you like to support 32-bit architectures when possible?")
        .with_help_message("Note: Packages that are 32bit only will still be enabled, eg steam")
        .with_default(true)
        .prompt()?;
    let allow_missing_variants = Confirm::new("Would you like to allow missing package variants?")
        .with_default(false)
        .with_help_message("If this is false certain packages will not be provided if they are missing for an enabled architecture
        In order In order to prevent this consider enabling use_auto_build")
    .prompt()?;
    let mut architectures = Vec::new();
    for idx in architecture.iter() {
        let arch = Architecture {
            typ: *idx,
            bitness: libcomet::package::Bitnes::X64,
        };
        if is_32_bit {
            let arch_32 = Architecture {
                typ: *idx,
                bitness: libcomet::package::Bitnes::X32,
            };
            architectures.push(arch_32);
        }
        architectures.push(arch);
    }
    let input_url = Text::new("What will the baseURL for your repository be?")
        .with_initial_value("https://")
        .prompt()
        .unwrap();
    let baseurl = Url::from_str(&input_url)?;
    let protected = Confirm::new("Would you like this repository to be password protected")
        .with_default(false)
        .prompt()?;
    let use_http = Confirm::new("Would you like to enable HTTP?")
        .with_default(false)
        .prompt()?;
    let log_level = Select::new(
        "What log level would you like the server to use",
        vec![
            log::LevelFilter::Trace,
            log::LevelFilter::Error,
            log::LevelFilter::Warn,
            log::LevelFilter::Debug,
            log::LevelFilter::Info,
        ],
    )
    .prompt()?;
    let log_path: Option<PathBuf> = Text::new("Where do you want to store your logs?")
        .prompt_skippable()
        .map(|s| {
            if s.is_some() {
                let path = PathBuf::from_str(&s.unwrap()).ok();
                return path;
            } else {
                None
            }
        })?;
    let allow_external_mirrors = Confirm::new(
        "Would you like to allow others to automatically create a mirror of your repo?",
    )
    .prompt()?;
    let use_auto_build =
        Confirm::new("Would you like to automatically cross-compile missing packages").prompt()?;
    let config = ServerConfig {
        port,
        baseurl,
        repo_type,
        architectures,
        allow_missing_variants,
        use_http,
        protected,
        log_level,
        log_path,
        allow_external_mirrors,
        use_auto_build,
        repo_name,
        uid,
    };
    let config_path = get_server_dir().context("Failed to obtain config path")?;

    let config_content =
        serde_yml::to_string(&config).context("Failed to serialize config content")?;
    debug!("config: {config_content:#}");
    workspace::write_to_path(&config_path, config_content, "config.server.yml")
        .context("Failed to write new configuration to server.config.toml")?;
    Ok(config)
}

pub fn create_config() -> ServerConfig {
    ServerConfig::default()
}
