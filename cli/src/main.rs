#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
#[macro_use] extern crate rocket;
use std::collections::HashMap;
use anyhow::{Ok, Result};
use cli::run_cli;
use serde::{Deserialize, Serialize};
use workspace::{first_run, get_data_dir, get_working_dir};
pub mod workspace;
pub mod config;
pub mod package;
pub mod cli;



fn main_run() -> Result<()> {
    //let path = get_working_dir()?;
    //println!("{}",path.display());
    //first_run()?;

        
    run_cli();
    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    main_run().ok();
    rocket::build().mount("/", routes![index])
}