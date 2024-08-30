use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },
    Install {
        package: Option<String>,
    },
    Remove {
        package: Option<String>,
        #[arg(short, long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(ref config_path) = cli.config {
        println!("Value for config: {config_path:?}");
    }

    println!(
        "{}",
        match cli.debug {
            0 => "Debug mode is off",
            1 => "Debug mode is kind of on",
            2 => "Debug mode is on",
            _ => "Don't be crazy",
        }
    );

    match cli.command {
        Some(Commands::Test { list }) => {
            if list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Install { package: None }) => {
            print!("Package must be specified!");
        }
        Some(Commands::Install {
            package: Some(ref package),
        }) => {
            print!("your package is: {}", package)
        }
        Some(Commands::Remove {
            package: None,
            force: _,
        }) => {
            print!("No package to remove")
        }
        Some(Commands::Remove {
            package: Some(ref package),
            force,
        }) => {
            print!("Removing {}", package);
            if force {
                print!("\nForce flag specified, Removing...")
            }
        }
        None => {}
    }
}