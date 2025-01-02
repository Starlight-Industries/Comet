use crate::*;
use assert_cmd::{cargo::CommandCargoExt, Command};
use std::{path::PathBuf, primitive::str};
#[test]
fn test_main() {
    let mut cmd = Command::cargo_bin("comet").unwrap();

    cmd.assert().success();
}
#[test]
fn install() {
    let mut cmd = Command::cargo_bin("comet").unwrap();
    cmd.arg("install").arg("blender");
    cmd.assert().success();
}

#[test]
fn install_with_version() {
    let mut cmd = Command::cargo_bin("comet").unwrap();
    cmd.arg("install").arg("blender").arg("4.0.1");
    cmd.assert().success();
}

#[test]
fn test_empty_command() {
    Command::cargo_bin("comet").unwrap().assert().success();
}

#[test]
fn test_install_basic() {
    Command::cargo_bin("comet")
        .unwrap()
        .arg("install")
        .arg("blender")
        .assert()
        .success();
}

#[test]
fn test_install_with_version() {
    Command::cargo_bin("comet")
        .unwrap()
        .arg("install")
        .arg("blender")
        .arg("4.0.1")
        .assert()
        .success();
}

#[test]
fn test_add_repo() {
    // not yet implemented but the route should exist
    Command::cargo_bin("comet")
        .unwrap()
        .arg("add")
        .arg("https://example.com/repo.json")
        .assert()
        .failure();
}

#[test]
fn test_list_packages() {
    Command::cargo_bin("comet")
        .unwrap()
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_remove_package() {
    Command::cargo_bin("comet")
        .unwrap()
        .arg("remove")
        .arg("blender")
        .assert()
        .success();
}

use predicates::prelude::*;
#[test]
fn test_help_output() {
    Command::cargo_bin("comet")
        .unwrap()
        .arg("--help")
        .assert()
        .stdout(predicate::str::contains("Avalible Commands"))
        .success();
}
#[test]
fn test_install_invalid_version() {
    Command::cargo_bin("comet")
        .unwrap()
        .arg("install")
        .arg("blender")
        .arg("invalid.version-53smdf-;[[''[")
        .assert()
        .failure();
}

#[test]
fn test_install_without_version() {
    Command::cargo_bin("comet")
        .unwrap()
        .arg("install")
        .arg("blender")
        .assert()
        .success();
}

#[test]
fn test_verbose_output() {
    Command::cargo_bin("comet")
        .unwrap()
        .arg("-v")
        .arg("add")
        .arg("https://example.com/repo.json")
        .assert()
        .failure();
}

#[test]
fn test_validate_nonexistent_path() {
    Command::cargo_bin("comet")
        .unwrap()
        .arg("validate")
        .arg("/nonexistent/path")
        .assert()
        .failure();
}

#[test]
fn test_unknown_command() {
    // Verify that an unknown command returns a successful exit code
    // #[cfg(debug_assertions)]
    // Command::cargo_bin("comet")
    //     .unwrap()
    //     .arg("unknown")
    //     .assert()
    //     .failure();
    // Verify that an unknown command returns a successful exit code in release mode
    // #[cfg(not(debug_assertions))]
    Command::cargo_bin("comet")
        .unwrap()
        .arg("unknown")
        .assert()
        .success();
}

#[test]
fn test_version_output() {
    Command::cargo_bin("comet")
        .unwrap()
        .arg("--version")
        .assert()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")))
        .success();
}

#[test]
fn test_install_multiple_args() {
    Command::cargo_bin("comet")
        .unwrap()
        .arg("install")
        .arg("blender")
        .arg("4.0.1")
        .arg("--verbose")
        .assert()
        .success();
}
#[test]
fn read_valid_package() {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dir = dir.parent().unwrap();
    let dir = dir.join("tests/spec/example.package.toml");
    let package_str = std::fs::read_to_string(dir).unwrap();
    let package = toml::from_str::<PackageConfig>(&package_str);
    assert!(package.is_ok());
}


// #[test]
// fn read_valid_repo() {
//     let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//     let dir = dir.parent().unwrap();
//     let dir = dir.join("tests/spec/example.repository.toml");
//     let package_str = std::fs::read_to_string(dir).unwrap();
//     let package = toml::from_str::<PackageConfig>(&package_str);
//     assert!(package.is_ok());
// }