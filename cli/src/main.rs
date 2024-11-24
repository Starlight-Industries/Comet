#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
#[macro_use] extern crate rocket;
use anyhow::{Ok, Result};
use cli::run_cli;
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