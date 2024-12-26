use std::time::Duration;

use crate::{
    api::run_server,
    workspace::{get_config, is_initalized},
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use inquire::Confirm;
use log::{debug, info};
use prompt::create_config_interactive;
use rocket::futures::FutureExt;
use tokio::{task::JoinError, time::Instant};
pub mod prompt;

#[derive(Parser)]
#[command(name = "comet-server", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
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
            
            let mut attempts: u16 = 0;
            let mut last_attempt_time = Instant::now();
        
            let signal = tokio::spawn(async {
                tokio::signal::ctrl_c().await.expect("Failed gracefully terminate");
                println!("Received termination signal");
                std::process::exit(0);
            });
            loop {
                let config = get_config().expect("Failed to obtain server configuration");
                let result = std::panic::catch_unwind(|| {
                    tokio::spawn(async move  {
                        run_server(&config).await.expect("Failed to run server");
                    })
                });
                match result {
                    Ok(handle) => {
                        match handle.await {
                            Ok(_) => {
                                println!("Server exited gracefully");
                                std::process::exit(0);
                            }
                            Err(e) => {
                                println!("Server crashed: {}", e);
                                if !handle_crash(&mut attempts, &mut last_attempt_time) {
                                    std::process::exit(0);
                                }
                            }
                        }
                    },
                    Err(e) => {
                        if handle_crash(&mut attempts, &mut last_attempt_time) {
                            std::process::exit(0);
                        } else {
                            println!("Server crashed: {:#?}. attempting to restart", e);
                        }
                    },
                }
            }

            let _ = signal.now_or_never();
        }
        None => {
            println!("No command provided. Use 'comet --help' for usage information.");
            std::process::exit(0);
        }
    }
}

fn handle_crash(attempts: &mut u16, last_attempt_time: &mut Instant) -> bool {
    let now = Instant::now();
    if now.duration_since(*last_attempt_time) > Duration::from_secs(10) {
        // Reset the attempts counter if the last crash was more than 10 seconds ago
        *attempts = 0;
    }
    *last_attempt_time = now;

    *attempts += 1;
    if *attempts > 5 {
        false // Exit after 5 rapid failures
    } else {
        println!("Server failed. Retrying... (Attempt {attempts}/5)");
        true
    }
}