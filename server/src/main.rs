use anyhow::Result;
use cli::run_cli;
use splash::print_splash;

pub mod api;
pub mod cli;
pub mod splash;
pub mod workspace;

#[rocket::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();
    print_splash();
    run_cli().await?;

    Ok(())
}
