use crate::{
    api::run_server,
    workspace::{get_config, is_initalized},
};
use anyhow::{Context, Ok, Result};
use clap::{Parser, Subcommand};
use inquire::Confirm;
use log::{debug, info};
use prompt::create_config_interactive;
use rocket::Config;
pub mod prompt;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    #[arg(short, long)]
    // Disables interactive prompts
    unattended: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short)]
        // automatically accepts defaults
        y: bool,
    },
    Run {
        #[arg(short, long)]
        /// Runs the server as a background process
        daemon: bool,
    },
}

pub async fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { y }) => match (is_initalized(), y) {
            (true, false) => {
                let confirm = Confirm::new(
                    "A configuration file already exists, are you sure you want to overwrite it?",
                )
                .with_default(false)
                .prompt()?;

                match confirm {
                    false => std::process::exit(0),
                    true => {
                        create_config_interactive()?;
                        println!("Use comet-server run to start your newly configured server!");
                        Ok(())
                    }
                }
            }
            (false, false) => {
                if !get_config().is_ok() {
                    let confirm = Confirm::new("You are about to overwrite an invalid configuration. Are you sure this is what you want?")
                        .with_default(false)
                        .prompt()?;
                    if !confirm {
                        std::process::exit(0)
                    }
                }
                create_config_interactive()?;
                Ok(())
            }
            _ => {
                unreachable!();
            }
        },
        Some(Commands::Run { daemon }) => {
            if daemon {
                debug!("Server will be started as a background process")
            }
            info!("Running server");
            let mut config = get_config()?;
            debug!("server_config = {config:#?}");

            match config {
                Some(_) => {
                    debug!("Config found");
                }
                None => match cli.unattended {
                    true => todo!(),
                    false => {
                        debug!("unattended mode is false, prompting user for config");
                        config = Some(
                            prompt::create_config_interactive()
                                .context("Failed to create config")?,
                        );
                    }
                },
            }
            run_server(config.expect("Failed to read config")).await?;
            Ok(())
        }
        None => {
            println!("No command provided. Use 'comet --help' for usage information.");
            std::process::exit(0);
        }
    }
}
