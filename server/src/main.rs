#![feature(panic_payload_as_str)]
use anyhow::Result;
use cli::run_cli;
use splash::print_splash;


pub mod cli;
pub mod splash;
pub mod workspace;
pub mod server;

#[rocket::main]
async fn main() -> Result<()> {
    
    #[cfg(debug_assertions)]
    {
        std::env::set_var("RUST_LOG", "debug");
    }
    #[cfg(not(debug_assertions))]
    {
        set_panic_hook();
    }

    env_logger::init();
    print_splash();
    run_cli().await?;

    Ok(())
}
