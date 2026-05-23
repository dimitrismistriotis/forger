use std::error::Error;
use std::path::Path;
use std::process::Command;

use git2::Repository;
use inquire::{Confirm, Text};
use owo_colors::OwoColorize;

pub mod port_number_rewrite;

mod common_strings;
pub mod doctor;
mod git_actions;
pub mod context;

use common_strings::FORGE_UNICODE;
use context::ApplicationContext;
use doctor::check_environment;
use git_actions::{clone_repo, commit_to_directory};
use port_number_rewrite::replace_text_in_target_folder;

const MAX_PORT: u16 = 65535;

#[derive(Debug)]
struct PortAndService {
    port_number: u16,
    service_name: &'static str,
    replace_only_mapped: bool,
}

const CONFIGURATION_BRANCH_NAME: &str = "post_checkout_configuration";
const PORTS_AND_SERVICES: [PortAndService; 5] = [
    PortAndService {
        port_number: 5432,
        service_name: "Postgresql, RDBMs used by the project",
        replace_only_mapped: false,
    },
    PortAndService {
        port_number: 1025,
        service_name: "Mailcrab, monitor email dispatches, STMP port",
        replace_only_mapped: false,
    },
    PortAndService {
        port_number: 1080,
        service_name: "Mailcrab, monitor email dispatches, web display port",
        replace_only_mapped: false,
    },
    PortAndService {
        port_number: 6379,
        service_name: "Redis, used for caching and communication with Celery for async tasks",
        replace_only_mapped: false,
    },
    PortAndService {
        port_number: 5050,
        service_name: "PG Admin, Postgresql Administration and web ui",
        replace_only_mapped: true,
    },
];

/// Checkout to post-configuration branch
fn checkout_to_post_configuration_branch(target_repository: Repository) -> Result<(), git2::Error> {
    // Source - https://stackoverflow.com/a/56942868
    // Posted by szignal, modified by community. See post 'Timeline' for change history
    // Retrieved 2026-05-02, License - CC BY-SA 4.0

    let head = target_repository.head()?;
    let oid = head
        .target()
        .ok_or_else(|| git2::Error::from_str("HEAD has no target OID"))?;
    let commit = target_repository.find_commit(oid)?;

    target_repository.branch(CONFIGURATION_BRANCH_NAME, &commit, false)?;

    let branch_ref = "refs/heads/".to_owned() + CONFIGURATION_BRANCH_NAME;
    let obj = target_repository.revparse_single(&branch_ref)?;

    target_repository.checkout_tree(&obj, None)?;
    target_repository.set_head(&branch_ref)?;

    Ok(())
}

pub fn create_new_djforge(
    repository: &str,
    target_branch: &str,
    target_directory: &str,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    println!("{FORGE_UNICODE}Clone {repository} @ {target_branch} in {target_directory}");

    // Guard Clauses

    // First is existence checking, I want to refactor this to
    // check if the target directory is a repository and continue from there
    let path = Path::new(target_directory);
    if path.is_dir() {
        return Err(format!("Directory {} already exists", target_directory.bold()).into());
    }

    let target_repository = clone_repo(repository, target_directory)?;

    checkout_to_post_configuration_branch(target_repository).map_err(|e| {
        format!(
            "Failed to checkout to configuration branch '{CONFIGURATION_BRANCH_NAME}': {e}. \
             The repository at {target_directory} may be in an inconsistent state."
        )
    })?;

    conditionally_replace_ports(target_directory, verbose);

    check_environment(verbose);

    conditionally_create_dot_env(target_directory, verbose);

    println!(
        "{FORGE_UNICODE}Application checked out and parametriesed in {}. Check and merge {}. You are ready to go!",
        target_directory.bold(),
        target_branch.bold()
    );
    Ok(())
}

/// Wraps replace_ports allowing main function to cal a one-liner
fn conditionally_replace_ports(target_directory: &str, verbose: bool) {
    let port_rename_prompt = Confirm::new("Continue with Port rename?")
        .with_default(true)
        .with_help_message("Find/Replace in target directory of default ports for Database and other services. Aims to allow multiple installations running together")
        .prompt();

    match port_rename_prompt {
        Ok(true) => replace_ports(target_directory, verbose),
        Ok(false) => println!("Continuing without port rename"),
        Err(_) => println!("Error with Port renaming"),
    }
}

/// Replace Standard/Dafault Ports
/// Wraps call to sed with replace parameters, commits after each replace.
pub fn replace_ports(target_directory: &str, verbose: bool) {
    println!(
        "{FORGE_UNICODE}Optional find and replace for service ports in {}",
        target_directory.bold()
    );
    for port_and_service in PORTS_AND_SERVICES {
        if verbose {
            println!(
                "{} on port {} replace only mapped port: {}",
                port_and_service.service_name.bold(),
                port_and_service.port_number.blue(),
                port_and_service.replace_only_mapped,
            );
        } else {
            println!(
                "{} on port {}",
                port_and_service.service_name.bold(),
                port_and_service.port_number.blue(),
            );
        }

        let port_prompt = format!(
            "Which port should be used for {}? Range: 1-{}",
            port_and_service.service_name, MAX_PORT
        );

        // Shadow to string:
        let target_port_number = format!("{}", port_and_service.port_number);
        let port_for_service = Text::new(&port_prompt)
            .with_default(&target_port_number)
            .prompt();

        match port_for_service {
            Ok(provided_port_from_user) => {
                if verbose {
                    println!("Provided: {}", provided_port_from_user);
                }

                let is_numeric = provided_port_from_user.chars().all(|c| c.is_ascii_digit());
                if !is_numeric {
                    eprintln!(
                        "Please provide a digit only value for target port, provided: {}",
                        provided_port_from_user.red()
                    );

                    break;
                }

                let parse_port_number = provided_port_from_user.parse::<u16>();

                match parse_port_number {
                    Ok(port_number) => {
                        if port_number < 1 || port_number > MAX_PORT {
                            eprintln!(
                                "Please provide a value within range 1 - {}, provided: {}",
                                MAX_PORT,
                                provided_port_from_user.red()
                            );

                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Could not parse target port, provided: {}, Error: {}",
                            provided_port_from_user.red(),
                            e
                        );

                        break;
                    }
                }

                let replace_happened = replace_text_in_target_folder(
                    &target_directory,
                    &target_port_number,
                    &provided_port_from_user,
                    port_and_service.replace_only_mapped,
                    verbose,
                );

                if replace_happened {
                    let commit_message = format!(
                        "Replaced {} port for {} with {}",
                        &port_and_service.port_number,
                        &port_and_service.service_name.bold(),
                        &provided_port_from_user
                    );

                    match commit_to_directory(target_directory, &commit_message) {
                        Ok(_) => println!("Committed: {}", commit_message),
                        Err(e) => println!("Failed to commit: {}", e),
                    }
                }
            }
            Err(_) => println!("An error happened when asking for target port."),
        }
    }
}

/// Wraps create_dot_env allowing main function to cal a one-liner
fn conditionally_create_dot_env(target_directory: &str, verbose: bool) {
    let port_rename_prompt = Confirm::new("Continue with .env creation?")
        .with_default(true)
        .with_help_message("Generates a skeletal .env so that the appplication can run, you can skip and create one on your own or run `make create_dot_env_file`")
        .prompt();

    match port_rename_prompt {
        Ok(true) => create_dot_env(target_directory, verbose),
        Ok(false) => println!("Continuing without .env creation"),
        Err(_) => println!("Error with .env creation"),
    }
}

/// Creates .env using DJForge's make if not there in target_directory
pub fn create_dot_env(target_directory: &str, verbose: bool) {
    println!(
        "{FORGE_UNICODE}Create .env in {} if not there, using DJForge's make command",
        target_directory.bold()
    );

    let dot_env_location = format!(
        "{}/{}",
        target_directory
            .strip_suffix("/")
            .unwrap_or(target_directory),
        ".env"
    );
    let dot_env_path = Path::new(&dot_env_location);
    if dot_env_path.exists() {
        println!("{} already exists, continuing.", dot_env_location.bold());

        return;
    }

    // if not cd there and run `make create_dot_env_file`
    let make_output = Command::new("make")
        .current_dir(target_directory)
        .arg("create_dot_env_file")
        .output()
        .expect("Failed to execute make create_dot_env_file command");

    // Check exit status
    if make_output.status.success() {
        if verbose {
            let stdout = String::from_utf8_lossy(&make_output.stdout);
            println!("Success! Output:\n{}", stdout);
        }

        // Optionally handle stderr
        let stderr = String::from_utf8_lossy(&make_output.stderr);
        if !stderr.is_empty() {
            eprintln!("Warnings/Errors:\n{}", stderr);
        }
    } else {
        eprintln!("Make command failed with status: {}", make_output.status);

        let error_message = String::from_utf8_lossy(&make_output.stderr);
        eprintln!("Make command failed with error: {}", error_message);
    }
}

pub fn replace_network_in_docker_compose(context: ApplicationContext) {
    println!("Context: {:?}", context);

    if context.verbose {
        println!("Working with {}", context.target);
    }

    // TODO:
    // SED docker compose yaml in tearget to rename the newtwork avoiding conflicts
    // Add this method to the base run-all one
}
