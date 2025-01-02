
use anyhow::anyhow;
use anyhow::Result;

use libcomet::request::repo::IdentityRequest;
use log::error;
use log::info;
use rocket::config::Ident;
use rocket::get;
use rocket::post;
use rocket::routes;
use rocket::serde::json::Json;

use crate::cli::Overrides;
use crate::workspace::get_config;
use crate::workspace::ServerConfig;
#[get("/")]
fn identity() -> Json<IdentityRequest> {
    let architectures = get_config().unwrap().architectures;
    IdentityRequest {
        uid: "io.github.comet".to_string(),
        display_name: "Comet".to_string(),
        architectures,
        legacy_support: true,
    }
    .into()
}

use libcomet::request::package::GetRequest;

#[post("/get", data = "<package>")]
fn get_package(package: Json<GetRequest>) -> Json<GetRequest> {
    info!("requested package name: '{package:#?}' with version: 'latest'");
    let req = package.into_inner();
    Json(req)
}

pub async fn run_server(config: &ServerConfig, overrides: Option<Overrides>) -> Result<()> {
    let log_level = &config.log_level.as_str().to_lowercase();
    println!("Starting comet server. Current log level: {log_level}");
    log::set_max_level(config.log_level);
    let _server = rocket::build()
        .configure(rocket::config::Config {
            port: match overrides {
                Some(overrides) => match overrides.port {
                    Some(port) => {
                        println!("Overriding port to {port}");
                        port
                    },
                    None => config.port,
                    
                },
                None => config.port,
            },
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
