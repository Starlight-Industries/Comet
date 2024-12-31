#![feature(panic_payload_as_str)]
use anyhow::Result;
use cli::run_cli;
use libcomet::request::package::GetRequest;
use splash::print_splash;
use versions::SemVer;
use crate::server::panic::set_panic_hook;


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
    let test = GetRequest { package_name: String::from("ashkjdfs"), package_version: String::from("0.0.1")};
    let json = serde_json::to_string_pretty(&test).unwrap();
    println!("{:#}", json);

    env_logger::init();
    print_splash();
    // run_cli().await?;

    Ok(())
}

