#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
use anyhow::Result;
use cli::run_cli;
use libcomet::package::{Package, PackageConfig};
pub mod workspace;
pub mod config;
pub mod package;
pub mod cli;



fn main() -> Result<()> {
    // let test_str = std::fs::read_to_string("example.package.toml")?;
    // let test_package = toml::from_str::<PackageConfig>(&test_str)?;
    // println!("{test_package:#?}");
    run_cli()?;
    Ok(())
}

