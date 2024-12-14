use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use libcomet::package::Architecture;
use libcomet::package::ArchitectureType;
use libcomet::package::Bitnes;
use libcomet::package::PackageState;
use serde::Deserialize;
use serde::Serialize;
use url::Url;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    // A vector of package types that the current hosted server provides
    // eg, the server has both stable and nightly variants for Packages
    repo_type: Vec<PackageState>,
    // The types of packages this repo will host
    // The architectures the current repository supports
    architectures: Vec<Architecture>,
    // Allow the server to skip discovery of packages that dont have variants
    allow_missing_variants: bool,
    // Base url the repository is hosted at
    baseurl: Url,
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
            baseurl: Url::from_str("https://0.0.0.0:8000").unwrap(),
        }
    }
}
