use std::{env, time::Duration};

use anyhow::{anyhow, Ok, Result};
use log::{debug, error, info, warn};
use rocket::{
    http,
    serde::{Deserialize, Serialize},
};
pub mod workspace;
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/get/<package>/<version>")]
fn give_stuff(package: &str, version: &str) -> String {
    info!("requested package name: '{package}' requested package version: '{version}'");
    let response = format!("{package}V{version}");
    response
}

async fn run_server() -> Result<()> {
    let _server = rocket::build()
        .mount("/", routes![index, give_stuff])
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
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Server started");
    let (a, b) = tokio::join!(test_stuff(), run_server(),);

    //run_server().await?;

    Ok(())
}
