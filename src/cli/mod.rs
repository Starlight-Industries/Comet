use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Install {
        package: String,
    },
    Remove {
        package: String,
    },
    List,
}

pub fn run_cli() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Install { package }) => {
            println!("Installing {}", package);
            std::process::exit(0);
        }
        Some(Commands::Remove { package }) => {
            println!("Removing {}", package);
            std::process::exit(0);
        }
        Some(Commands::List) => {
            println!("Listing installed packages");
            std::process::exit(0);
        }
        None => {
            println!("No command provided. Use 'comet --help' for usage information.");
            std::process::exit(0);
        }
    }
}