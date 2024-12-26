pub mod packages;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use libcomet::package::Architecture;
use libcomet::package::ArchitectureType;
use libcomet::package::Bitnes;
use libcomet::package::PackageState;
use libcomet::workspace::get_working_dir;
use log::debug;
use log::error;
use rocket::http::hyper::server::Server;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub repo_name: String,
    // UID must be a reverse dns url, eg. io.github.starlight-industries.comet
    pub uid: String,
    // A vector of package types that the current hosted server provides
    // eg, the server has both stable and nightly variants for Packages
    pub repo_type: Vec<PackageState>,
    pub port: u16,
    // The types of packages this repo will host
    // The architectures the current repository supports
    pub architectures: Vec<Architecture>,
    // Allow the server to skip discovery of packages that dont have variants
    // Eg. server provides blender 4 X86_64 but not blender 4 aarch64.
    // Defaults to TRUE
    pub allow_missing_variants: bool,
    // Base url the repository is hosted at
    pub baseurl: Url,
    // Define if the server has a cert
    pub use_http: bool,
    pub protected: bool,
    pub log_level: log::LevelFilter,
    pub log_path: Option<PathBuf>,
    // Allow others to automatically create backups of this servers binaries
    // locally on their machines, for archival or otherwise.
    // Defaults to FALSE
    pub allow_external_mirrors: bool,
    // Automatically attempt to build packages for all selected platforms
    // if its build configuration is avalible.
    // Defaults to FALSE
    pub use_auto_build: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let repo_type = vec![PackageState::Stable];
        let default_architecture = Architecture {
            typ: ArchitectureType::X86,
            bitness: Bitnes::X64,
        };
        let architectures = vec![default_architecture];
        Self {
            repo_type,
            architectures,
            allow_missing_variants: false,
            port: 8000,
            baseurl: Url::from_str("https://0.0.0.0:8000").unwrap(),
            use_http: true,
            protected: false,
            log_level: log::LevelFilter::Info,
            log_path: Some(PathBuf::from_str("./").unwrap()),
            allow_external_mirrors: false,
            use_auto_build: false,
            repo_name: String::from("MyRepo"),
            uid: String::from("io.github.john.myrepo"),
        }
    }
}

pub fn get_config() -> Result<ServerConfig> {
    let mut config_path = get_working_dir()?;
    if !config_path.exists() {
        debug!("the Comet config path did not yet exist, this is likely the first run");
        return Err(anyhow!("The server has not been initialized"));
    }
    config_path.push("server/config.server.yml");
    if !config_path.exists() {
        debug!("the server config did not exist: {config_path:#?}");
        return Err(anyhow!("There is not a configuration n the server directory"));
    }
    let config =
        fs::read_to_string(config_path).context("Failed to read configuration file to string")?;
    match serde_yml::from_str(&config) {
        Ok(config) => {
            debug!("server config has been serialized: Config: {config:#?}");
            return Ok(config);
        }
        Err(e) => {
            error!("Could not serialize configuration: {e}");
            return Err(e.into());
        }
    };
}

pub fn is_initalized() -> bool {
    let mut config_path = get_working_dir().unwrap();
    config_path.push("server/config.server.yml");

    match get_config() {
        Ok(config) => {
            true
        }
        Err(e) => {
            error!("An error has occured, assuming we are not initalized: {e}");
            false
        }
    }
}

pub fn write_to_path(path: &PathBuf, content: String, file_name: &str) -> Result<()> {
    if !path.exists() {
        debug!("Path did not yet exist: {path:#?}");
        fs::create_dir_all(path)?;
    }
    let mut new_path = path.clone();
    new_path.push(file_name);
    debug!("Full file path: {new_path:#?}");
    let mut file = File::create(&new_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
