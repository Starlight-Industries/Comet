use anyhow::anyhow;
use anyhow::Result;
use cli::run_cli;
use libcomet::prelude::*;
use log::{debug, error, info, warn};
use rocket::{
    http,
    serde::{Deserialize, Serialize},
};
use std::{env, time::Duration};
pub mod api;
pub mod cli;
pub mod workspace;

#[rocket::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    run_cli().await?;
    // let (_a, _b) = tokio::join!(test_stuff(), run_server());

    //run_server().await?;

    Ok(())
}
