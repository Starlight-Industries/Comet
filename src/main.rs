use core::str;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    Install {
        //#[arg(short,long)]
        package: Option<String>
    },
    Remove {
        package: Option<String>,
        #[arg(short, long)]
        force: bool
    }
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list}) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Install { package: None }) => {
            //print!("your package name is: {}", possible_package)
            print!("Package must be specified!");
        }
        Some(Commands::Install { package: Some(package) }) => {
            print!("your package is: {}",package)
        }
        Some(Commands::Remove { package: None, force: _ }) => {
            print!("No package to remove")
        }
        Some(Commands::Remove { package: Some(package), force }) => {
            print!("Removing {}",package);
            if *force {
                print!("\nForce flag specified, Removing...")
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}