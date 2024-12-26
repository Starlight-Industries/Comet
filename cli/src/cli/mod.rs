use clap::{Parser, Subcommand};
use libcomet::request::repo::IdentityRequest;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Install { package: String },
    Remove { package: String },
    List,
    Add { repo_ink: String, alias: Option<String> },
}

pub fn run_cli() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Install { package }) => {
            println!("Installing {}", package);
            std::process::exit(0);
        }
        Some(Commands::Add { repo_ink, .. }) => {
            println!("Adding repo with ink {}", repo_ink);
            let response = reqwest::blocking::get(repo_ink)?.text()?;
            let ident = serde_json::from_str::<IdentityRequest>(&response)?;
            
            println!("server identity: {ident:#?}");

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
