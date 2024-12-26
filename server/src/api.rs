use core::arch;

use anyhow::anyhow;
use anyhow::Result;
use libcomet::package::Architecture;
use libcomet::request::repo::IdentityRequest;
use log::error;
use log::info;
use rocket::config::Ident;
use rocket::figment::Provider;
use rocket::get;
use rocket::routes;
use rocket::serde::json::Json;

use crate::workspace::get_config;
use crate::workspace::ServerConfig;
#[get("/")]
fn identity() -> Json<IdentityRequest> {
    let architectures = get_config().unwrap().architectures;
    let t: Option<String> = None;
    t.unwrap();
    IdentityRequest {
        uid: "io.github.comet".to_string(),
        display_name: "Comet".to_string(),
        architectures,
        legacy_support: true,
    }
    .into()
}

#[get("/get/<package>/<version>")]
fn get_package(package: &str, version: &str) -> String {
    info!("requested package name: '{package}' requested package version: '{version}'");
    let response = format!("{package}V{version}");
    response
}

pub async fn run_server(config: &ServerConfig) -> Result<()> {
    let log_level = &config.log_level.as_str().to_lowercase();
    println!("Starting comet server. Current log level: {log_level}");
    log::set_max_level(config.log_level);
    let _server = rocket::build()
        .configure(rocket::config::Config {
            port: config.port,
            ident: Ident::try_new("Comet").unwrap(),

            ..Default::default()
        })
        .mount("/", routes![identity, get_package])
        .launch()
        .await
        .map_err(|e| {
            error!("An error occured. {e}");
            anyhow!(format!("Failed to start rocket server: {e:#}"))
        });
    Ok(())
}
