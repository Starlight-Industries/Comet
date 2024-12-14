use anyhow::anyhow;
use anyhow::Result;
use log::error;
use log::info;
use rocket::get;
use rocket::routes;
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

pub async fn run_server() -> Result<()> {
    info!("Starting comet server");
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
