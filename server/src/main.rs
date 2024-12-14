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
pub mod cli;
pub mod workspace;
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get/<package>/<version>")]
fn get_package(package: &str, version: &str) -> String {
    info!("requested package name: '{package}' requested package version: '{version}'");
    let response = format!("{package}V{version}");
    response
}

async fn run_server() -> Result<()> {
    let _server = rocket::build()
        .mount("/", routes![index, get_package])
        //.register("/", )
        .launch()
        .await
        .map_err(|e| {
            error!("An error occured");
            anyhow!(format!("Failed to start rocket server: {e:#}"))
        });
    Ok(())
}

async fn test_stuff() -> Result<()> {
    tokio::time::sleep(Duration::new(5, 0)).await;
    info!("First test has completed");
    Ok(())
}

#[rocket::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    run_cli();
    info!("Server started");
    //let (_a, _b) = tokio::join!(test_stuff(), run_server());

    //run_server().await?;

    Ok(())
}
