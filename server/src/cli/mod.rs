use std::{sync::atomic::AtomicU8, time::Duration};

use crate::{
    server::{
        api::run_server,
        panic::{render_backtrace, write_backtrace},
    },
    workspace::{get_config, is_initalized},
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use inquire::Confirm;
use libcomet::workspace::get_working_dir;
use log::{debug, info};
use prompt::create_config_interactive;
use tokio::time::Instant;
pub mod prompt;
use lazy_static::lazy_static;
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
        #[arg(short, long)]
        port: Option<u16>,
    },
}
#[derive(Debug,Clone, Copy)]
pub struct Overrides {
    pub port: Option<u16>,
}

impl Overrides {
    pub fn builder() -> OverridesBuilder {
        OverridesBuilder::default()
    }
}
#[derive(Debug,Clone, Copy)]
pub struct OverridesBuilder {
    port: Option<u16>,
}

impl Default for OverridesBuilder {
    fn default() -> Self {
        Self { port: None }
    }
}

impl OverridesBuilder {
    pub fn port(mut self, port: Option<u16>) -> Self {
        self.port = Some(port.unwrap());
        self
    }

    pub fn build(self) -> Option<Overrides> {
        self.port.map(|port| Overrides { port: Some(port) })
    }
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
                if get_config().is_err() {
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
        Some(Commands::Run { daemon, port }) => {
            let mut overrides = Overrides::builder();
            if port.is_some() {
                overrides = overrides.port(port);
            }

            if daemon {
                debug!("Server will be started as a background process")
            }
            info!("Running server");

            let mut last_attempt_time = Instant::now();

            loop {
                let config =
                    get_config().unwrap_or_else(|_| match prompt::create_config_interactive() {
                        Ok(generated_config) => {
                            return generated_config;
                        }
                        Err(e) => {
                            println!("Failed to create config: {e}");
                            std::process::exit(1);
                        }
                    });
                    
                let result = std::panic::catch_unwind(|| {
                    tokio::spawn(async move {
                        std::thread::sleep(Duration::new(5, 0));
                        let arg = overrides.build();
                        run_server(&config,arg).await.expect("Failed to run server");
                    })
                });
                match result {
                    Ok(handle) => match handle.await {
                        Ok(_) => {
                            println!("Server exited gracefully");
                            std::process::exit(0);
                        }
                        Err(e) => {
                            println!("Server crashed: {}", e);
                            handle_crash(&mut last_attempt_time, format!("Server crashed: {}", e));
                        }
                    },
                    Err(_) => {
                        handle_crash(
                            &mut last_attempt_time,
                            "Server crashed: An unknown error has occured".to_string(),
                        );
                    }
                }
            }
        }
        None => {
            println!("No command provided. Use 'comet --help' for usage information.");
            std::process::exit(0);
        }
    }
}
const MAX_ATTEMPTS: u8 = 3;
lazy_static! {
    pub static ref ATTEMPTS: AtomicU8 = AtomicU8::new(0);
}

fn handle_crash(last_attempt_time: &mut Instant, error_string: String) {
    let now = Instant::now();
    if now.duration_since(*last_attempt_time) > Duration::from_secs(10) {
        // Reset the attempts counter if the last crash was more than 10 seconds ago
        ATTEMPTS.store(0, std::sync::atomic::Ordering::SeqCst);
        println!("resetting attempts");
    }
    *last_attempt_time = now;

    let attempts = ATTEMPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
    if attempts > MAX_ATTEMPTS {
        let log_path = get_working_dir().expect("Failed to get working directory");
        if !log_path.exists() {
            std::fs::create_dir_all(&log_path).expect("Failed to create log directory");
        }
        let log_path = log_path.join("panic.log");
        write_backtrace(render_backtrace(), error_string, &log_path);
        println!(
            "{}",
            "Server failed too many times. Exiting..."
                .red()
                .bold()
                .italic()
                .underline()
        );
        std::process::exit(0);
    } else {
        println!("Server failed. Retrying... (Attempt {attempts}/{MAX_ATTEMPTS})");
    }
}
