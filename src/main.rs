use clap::{Parser, Subcommand};

use forger::context::ApplicationContext;
use forger::doctor::check_environment;
use forger::{
    create_dot_env, create_new_djforge, replace_network_in_docker_compose, replace_ports,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates new repository in <target> directory end to end
    New { target: String },
    /// Replaces ports for services for cloned repository in <target> directory
    ReplacePorts { target: String },
    /// Repplaces network tag inside docer-compose.yml in <target> directory
    ReplaceNetworkInDockerCompose { target: String },
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
            if let Err(e) = create_new_djforge(
                REPOSITORY_LOCATION,
                REPOSITORY_DEFAULT_BRANCH,
                target,
                cli.verbose,
            ) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        Commands::Doctor => {
            check_environment(cli.verbose);
        }
        Commands::ReplacePorts { target } => replace_ports(target, cli.verbose),
        Commands::ReplaceNetworkInDockerCompose { target } => {
            let context = ApplicationContext::new(target.to_string(), cli.verbose);

            replace_network_in_docker_compose(context);
        }
        Commands::GenerateDotEnv { target } => {
            create_dot_env(target, cli.verbose);
        }
    }
}
