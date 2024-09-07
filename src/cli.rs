use clap::Parser;
use versions::Versioning;

#[derive(Clone, Debug)]
pub struct PackageIdentifier {
    pub version: Option<Versioning>,
    pub name: String,
}

impl From<&str> for PackageIdentifier {
    fn from(value: &str) -> Self {
        match value.split_once('=') {
            Some((a, b)) => Self {
                name: a.to_string(),
                version: Versioning::new(b),
            },
            _ => Self {
                name: value.to_string(),
                version: None,
            },
        }
    }
}

#[derive(Parser, Debug, Clone)]
pub enum Command {
    #[command(about = "Install a package")]
    Install { package: PackageIdentifier },
    #[command(about = "Uninstall a package")]
    Remove {
        package: PackageIdentifier,
        #[arg(short, long)]
        force: bool,
        #[arg(short, action, default_value_t = false)]
        recurse: bool,
    },
    #[command(about = "Refresh the package database")]
    Sync,
}

pub struct Cli {
    pub commands: Vec<Command>,
}

impl Cli {
    pub fn parse() -> Self {
        let args: String = std::env::args()
            .skip(1)
            .flat_map(|s| (s + " ").chars().collect::<Vec<_>>())
            .collect();
        Self {
            commands: args
                .split(',')
                .map(|cmd| {
                    let mut cmd: Vec<&str> = cmd.trim().split(" ").collect();
                    cmd.insert(0, "");
                    Command::parse_from(cmd)
                })
                .collect::<Vec<Command>>(),
        }
    }
}
