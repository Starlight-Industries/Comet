use anyhow::{anyhow, Context};
use clap::{Command, CommandFactory, Parser, Subcommand};
use colored::Colorize;
use libcomet::request::repo::IdentityRequest;
use std::{error::Error, path::PathBuf, str::FromStr};
use versions::Versioning;
pub mod error;
#[cfg(test)]
pub mod tests;
#[cfg(test)]
pub mod tests_advanced;
#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"),bin_name = bin_name(),about = long_about(),help_template = set_help_template())]

struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short = 'v', long = "verbose", help = "Verbose output")]
    verbose: bool,
}

fn bin_name() -> String {
    String::from("comet")
}

fn long_about() -> String {
    let star = r#"
                                            ...
                                            ....
                                           .......
                                         ...............
                                ......................
                        ...........................
               .........................    ......
       ....................                   ....
..............                                 ..
"#
    .blue()
    .bold();
    let tagline = format!("Comet - Simplifying your packages since 2024");
    let version_str = format!("version {}", env!("CARGO_PKG_VERSION"));
    let val = format!(
        "{}{}\n{}",
        star,
        tagline.cyan().bold(),
        version_str.bright_black()
    );
    val
}

fn set_help_template() -> String {
    let mut help_template = "\
{about}
"
    .to_string();
    help_template.push_str(&format!("\n{}", "Avalible Commands:\n".bright_blue()));
    let subcommands = "{subcommands}";
    help_template.push_str(&subcommands);
    help_template
}
#[derive(Subcommand)]
enum Commands {
    Install {
        package: String,
        version: Option<String>,
    },
    Remove {
        package: String,
    },
    List,
    Validate {
        path: PathBuf,
    },
    Add {
        repo_ink: String,
        alias: Option<String>,
    },
}
fn setup_cli() {
    let mut x = Cli::command();
}
pub async fn run_cli() -> anyhow::Result<()> {
    setup_cli();

    let cli = Cli::try_parse();
    match cli {
        Ok(cli) => {
            // let mut  x = cli.display_name("Comet");
            match cli.command {
                Commands::Install { package, version } => {
                    if version.is_some() {
                        let parsed_version =
                            versions::Versioning::from_str(&version.unwrap().as_str());
                        match parsed_version {
                            Ok(version) => {
                                println!(" version {}", version);
                                println!(" ver: {:#?}", version);
                                let eq = Versioning::new("9.0.1").unwrap();
                                if version < eq {
                                    println!("test version is less than input version");
                                } else if eq == version {
                                    println!("test version is equal to input version");
                                } else {
                                    println!("input version is greater than test version");
                                }
                                Ok(())
                            }
                            Err(e) => {
                                println!("Invalid version:{e}");
                                Err(anyhow!("Invalid version"))
                            }
                        }
                    } else {
                        print!("Installing {}", package);
                        Ok(())
                    }
                }
                Commands::Add { repo_ink, .. } => {
                    match reqwest::blocking::get(repo_ink) {
                        Ok(response) => match (response.text(), cli.verbose) {
                            (Ok(ident), _) => {
                                let ident = serde_json::from_str(&ident)
                                    .context("Failed to parse identity")?;
                                let ident = IdentityRequest::from(ident);
                                println!("{ident:#?}");
                            }
                            (Err(e), true) => {
                                println!("Detailed error: {:#}, {:#}", e, e.status().unwrap())
                            }
                            (Err(e), false) => println!(
                                "{}",
                                format!(
                                    "Failed to add repository: {} returned {}",
                                    e.url().unwrap().as_str(),
                                    e.status().unwrap()
                                )
                                .red()
                        ),
                        },
                        Err(e) => {
                            if cli.verbose {
                                println!("Failed to send request: {:#}", e);
                            } else {
                                println!("{}", format!("Failed to send request: {}", e,).red());
                            }
                        }
                    };
                    Ok(())
                }
                Commands::Remove { package } => {
                    println!("Removing {}", package);
                    Ok(())
                }
                Commands::List => {
                    println!("Listing installed packages");
                    Ok(())
                }
                Commands::Validate { path } => {
                    todo!()
                }
            }
        }
        Err(err) => {
            err.print().expect("Failed to print error, aborting");
            
            // clap::error::ErrorKind::InvalidValue => todo!(),
            // clap::error::ErrorKind::UnknownArgument => todo!(),
            // clap::error::ErrorKind::InvalidSubcommand => todo!(),
            // clap::error::ErrorKind::NoEquals => todo!(),
            // clap::error::ErrorKind::ValueValidation => todo!(),
            // clap::error::ErrorKind::TooManyValues => todo!(),
            // clap::error::ErrorKind::TooFewValues => todo!(),
            // clap::error::ErrorKind::WrongNumberOfValues => todo!(),
            // clap::error::ErrorKind::ArgumentConflict => todo!(),
            // clap::error::ErrorKind::MissingRequiredArgument => todo!(),
            // clap::error::ErrorKind::MissingSubcommand => todo!(),
            // clap::error::ErrorKind::InvalidUtf8 => todo!(),
            // clap::error::ErrorKind::DisplayHelp => todo!(),
            // clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => todo!(),
            // clap::error::ErrorKind::DisplayVersion => todo!(),
            // clap::error::ErrorKind::Io => todo!(),
            // clap::error::ErrorKind::Format => todo!(),
            // _ => todo!(),
            Ok(())
        }
    }
}
