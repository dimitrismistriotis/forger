use clap::{Parser, Subcommand};

use forger::doctor::check_environment;
use forger::{create_new_djforge, replace_ports};

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
    New {
        target: String,
    },
    /// Replaces ports for services for cloned repository in <target> directory
    ReplacePorts {
        target: String,
    },
    Doctor,
}

const REPOSITORY_LOCATION: &str = "https://github.com/dimitrismistriotis/djforge.git";
const REPOSITORY_DEFAULT_BRANCH: &str = "master";

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { target } => {
            create_new_djforge(REPOSITORY_LOCATION, REPOSITORY_DEFAULT_BRANCH, target);
        }
        Commands::Doctor => {
            println!("Check that everything needed for forger to run is available");

            check_environment();
        }
        Commands::ReplacePorts { target } => replace_ports(target),
    }
}
