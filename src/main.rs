mod cli;
use versions::Versioning;

fn main() {
    let cli = cli::Cli::parse();

    use cli::Command as C;
    for command in cli.commands {
        match command {
            C::Install { package } => {
                println!(
                    "Install Package: {}={}",
                    package.name,
                    package
                        .version
                        .unwrap_or_else(|| Versioning::new("*").unwrap())
                )
            }
            C::Remove { package, force, .. } => {
                println!(
                    "Remove Package: {}={}, Force: {force}",
                    package.name,
                    package
                        .version
                        .unwrap_or_else(|| Versioning::new("*").unwrap())
                )
            }
            C::Sync => {
                println!("Sync DB")
            }
            #[allow(unreachable_patterns)]
            _ => {}
        }
    }
}
