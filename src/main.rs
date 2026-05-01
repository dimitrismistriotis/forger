use clap::{Parser, Subcommand};

use forger::create_new_djforge;

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
}

const REPOSITORY_LOCATION: &str = "https://github.com/dimitrismistriotis/djforge.git";
const REPOSITORY_DEFAULT_BRANCH: &str = "master";

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { target } => {
            println!("Aiming to create new installation in {target:?}");

            create_new_djforge(REPOSITORY_LOCATION, REPOSITORY_DEFAULT_BRANCH, target);
        }
    }
}
