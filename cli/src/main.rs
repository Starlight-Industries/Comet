#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
use anyhow::{Ok, Result};
use cli::run_cli;
pub mod workspace;
pub mod config;
pub mod package;
pub mod cli;



fn main() -> Result<()> {
    //let path = get_working_dir()?;
    //println!("{}",path.display());
    //first_run()?;

        
    run_cli();
    Ok(())
}

