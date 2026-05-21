use clap::{Parser, Subcommand};

use forger::doctor::check_environment;
use forger::{create_dot_env, create_new_djforge, replace_ports};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates new repository in <target> directory
    New { target: String },
    /// Replaces ports for services for cloned repository in <target> directory
    ReplacePorts { target: String },
    /// Checks that commands needed to run DJForge exist in the system
    Doctor,
    /// Generate prelimilary .env with Secret Key so that the application can run
    GenerateDotEnv { target: String },
}

const REPOSITORY_LOCATION: &str = "https://github.com/dimitrismistriotis/djforge.git";
const REPOSITORY_DEFAULT_BRANCH: &str = "master";

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { target } => {
            if let Err(e) = create_new_djforge(REPOSITORY_LOCATION, REPOSITORY_DEFAULT_BRANCH, target) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        Commands::Doctor => {
            check_environment();
        }
        Commands::ReplacePorts { target } => replace_ports(target),
        Commands::GenerateDotEnv { target } => {
            create_dot_env(target);
        }
    }
}
