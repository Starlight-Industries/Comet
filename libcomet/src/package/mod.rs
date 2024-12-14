use std::backtrace::Backtrace;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;
use versions::Versioning;

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("An IO Faliure has occured")]
    Io {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[error("Package status could not be parsed. Found: {found:#}")]
    InvalidPackageStatus { found: String },
    #[error(
        "An Unknown package error occured.
        Please report this to https://codeberg.org/Starlight-industries/Comet/issues and include your server.log"
    )]
    Other(#[from] anyhow::Error),
}
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum PackageState {
    Nightly,
    Git,
    #[default]
    Stable,
    Testing,
}

impl PackageState {
    pub fn iter() -> impl Iterator<Item = PackageState> {
        [
            PackageState::Nightly,
            PackageState::Git,
            PackageState::Stable,
            PackageState::Testing,
        ]
        .into_iter()
    }
}

impl std::fmt::Display for PackageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageState::Nightly => writeln!(f, "Nightly"),
            PackageState::Git => writeln!(f, "Git"),
            PackageState::Stable => writeln!(f, "Stable"),
            PackageState::Testing => writeln!(f, "Testing"),
        }
    }
}

impl std::convert::TryFrom<String> for PackageState {
    type Error = PackageError;
    fn try_from(value: String) -> Result<Self, PackageError> {
        match value.as_str() {
            "Nightly" => Ok(PackageState::Nightly),
            "Git" => Ok(PackageState::Git),
            "Stable" => Ok(PackageState::Stable),
            "Testing" => Ok(PackageState::Testing),
            _ => Err(PackageError::InvalidPackageStatus { found: value }),
        }
    }
}
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Bitnes {
    X32,
    #[default]
    X64,
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum ArchitectureType {
    #[default]
    X86,
    RiscV,
    Aarch,
    Loong,
    PowerPC,
}
impl std::fmt::Display for ArchitectureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchitectureType::X86 => writeln!(f, "X86"),
            ArchitectureType::RiscV => writeln!(f, "RiscV"),
            ArchitectureType::Aarch => writeln!(f, "Aarch"),
            ArchitectureType::Loong => writeln!(f, "Loong"),
            ArchitectureType::PowerPC => writeln!(f, "PowerPC"),
        }
    }
}
impl ArchitectureType {
    pub fn iter() -> impl Iterator<Item = ArchitectureType> {
        vec![
            ArchitectureType::X86,
            ArchitectureType::RiscV,
            ArchitectureType::Aarch,
            ArchitectureType::Loong,
            ArchitectureType::PowerPC,
        ]
        .into_iter()
    }
}
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Architecture {
    #[serde(rename = "type")]
    pub typ: ArchitectureType,
    pub bitness: Bitnes,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Package {
    name: String,
    version: Versioning,
    target: Vec<Architecture>,
    meta: Option<PackageMeta>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PackageMeta {
    description: Option<String>,
    author: Option<String>,
    documentation: Option<Url>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io;

    #[test]
    fn test_io_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let package_error: PackageError = io_error.into();

        match package_error {
            PackageError::Io { source, .. } => {
                assert!(source.kind() == io::ErrorKind::NotFound);
                assert!(source.to_string() == "File not found");
            }
            _ => panic!("Expected Io variant"),
        }
    }

    #[test]
    fn test_invalid_package_status_error() {
        let error = PackageError::InvalidPackageStatus {
            found: "InvalidStatus".to_string(),
        };

        assert!(error.to_string().contains("InvalidStatus"));
    }

    #[test]
    fn test_other_error_conversion() {
        let original_error = anyhow::anyhow!("Some random error");
        let package_error: PackageError = original_error.into();

        match package_error {
            PackageError::Other(_) => {
                // Make sure that error conversion works
                // we dont want anything other than this type
            }
            _ => panic!("Expected Other variant"),
        }
    }

    #[test]
    fn test_package_error_implements_error_trait() {
        fn _assert_error_trait<E: Error>(_: &E) {}

        let io_error: PackageError = io::Error::new(io::ErrorKind::Other, "test").into();
        _assert_error_trait(&io_error);
    }

    #[test]
    fn test_package_state_from_string() {
        assert!(matches!(
            PackageState::try_from("Nightly".to_string()),
            Ok(PackageState::Nightly)
        ));
        assert!(matches!(
            PackageState::try_from("Git".to_string()),
            Ok(PackageState::Git)
        ));
        assert!(matches!(
            PackageState::try_from("Stable".to_string()),
            Ok(PackageState::Stable)
        ));
        assert!(matches!(
            PackageState::try_from("Testing".to_string()),
            Ok(PackageState::Testing)
        ));

        // Verify that package error can represent invalid package status
        let result = PackageState::try_from("Invalid".to_string());
        assert!(matches!(
            result,
            Err(PackageError::InvalidPackageStatus { found }) if found == "Invalid"
        ));
    }

    #[test]
    fn test_package_state_display() {
        // Verify that packages display correctly
        assert!(format!("{}", PackageState::Nightly).contains("Nightly"));
        assert!(format!("{}", PackageState::Git).contains("Git"));
        assert!(format!("{}", PackageState::Stable).contains("Stable"));
        assert!(format!("{}", PackageState::Testing).contains("Testing"));
    }
}
