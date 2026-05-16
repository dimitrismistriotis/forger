use std::path::Path;
use std::process;

use git2::Repository;
use inquire::{Confirm, Text};
use owo_colors::OwoColorize;

pub mod port_number_rewrite;

mod common_strings;
pub mod doctor;
mod git_actions;

use common_strings::FORGE_UNICODE;
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
fn checkout_to_post_configuration_branch(target_repository: Repository) {
    // Source - https://stackoverflow.com/a/56942868
    // Posted by szignal, modified by community. See post 'Timeline' for change history
    // Retrieved 2026-05-02, License - CC BY-SA 4.0

    let head = target_repository.head().unwrap();
    let oid = head.target().unwrap();
    let commit = target_repository.find_commit(oid).unwrap();

    let _branch = target_repository.branch(CONFIGURATION_BRANCH_NAME, &commit, false);

    let obj = target_repository
        .revparse_single(&("refs/heads/".to_owned() + CONFIGURATION_BRANCH_NAME))
        .unwrap();

    _ = target_repository.checkout_tree(&obj, None);

    _ = target_repository.set_head(&("refs/heads/".to_owned() + CONFIGURATION_BRANCH_NAME));
}

pub fn create_new_djforge(
    repository: &'static str,
    target_branch: &'static str,
    target_directory: &str,
) {
    println!("{FORGE_UNICODE}Clone {repository} @ {target_branch} in {target_directory}");

    // Guard Clauses

    // First is existence checking, I want to refactor this to
    // check if the target directory is a repository and continue from there
    let path = Path::new(target_directory);
    if path.is_dir() {
        println!("\nDirectory {target_directory} already exists, exiting...");
        process::exit(1);
    }

    let target_repository = clone_repo(repository, target_directory);
    println!("Empty: {}", target_repository.is_empty().unwrap());

    checkout_to_post_configuration_branch(target_repository);

    conditionally_replace_ports(target_directory);

    check_environment();

    println!(
        "{FORGE_UNICODE}Application checked out and parametriesed in {target_directory}. Check and merge {target_branch}. You are ready to go!"
    );
}

/// Wraps replace_ports allowing main function to cal a one-liner
fn conditionally_replace_ports(target_directory: &str) {
    let port_rename_prompt = Confirm::new("Continue with Port rename?")
        .with_default(true)
        .with_help_message("Find/Replace in target directory of default ports for Database and other services. Aims to allow multiple installations running together")
        .prompt();

    match port_rename_prompt {
        Ok(true) => replace_ports(target_directory),
        Ok(false) => println!("Continuing without port rename"),
        Err(_) => println!("Error with Port renaming"),
    }
}

/// Replace Standard/Dafault Ports
/// Wraps call to sed with replace parameters, commits after each replace.
pub fn replace_ports(target_directory: &str) {
    println!("{FORGE_UNICODE}Optional find and replace for service ports in {target_directory}");
    for port_and_service in PORTS_AND_SERVICES {
        println!(
            "{} on port {}, replace only mapped port: {}",
            port_and_service.service_name.bold(),
            port_and_service.port_number,
            port_and_service.replace_only_mapped,
        );

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
                println!("Provided: {}", provided_port_from_user);
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
                );

                if replace_happened {
                    // Commit the changes
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
