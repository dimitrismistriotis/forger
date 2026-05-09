use std::path::Path;
use std::process;

use git2::Repository;
use inquire::{Confirm, Text};

pub mod port_number_rewrite;

mod common_strings;

use common_strings::FORGE_UNICODE;
use port_number_rewrite::replace_text_in_target_folder;

#[derive(Debug)]
struct PortAndService {
    port_number: i16,
    service_name: &'static str,
}

const CONFIGURATION_BRANCH_NAME: &str = "post_checkout_configuration";
const PORTS_AND_SERVICES: [PortAndService; 1] = [PortAndService {
    port_number: 5432,
    service_name: "Postgresql, RDBMs used by the project",
}];

fn clone_repo(url: &str, path: &str) -> git2::Repository {
    let repo = match Repository::clone(url, path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };
    println!("Cloned {} to {}", url, path);

    repo
}

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
    println!("{FORGE_UNICODE} Clone {repository} @ {target_branch} in {target_directory}");

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

/// Replace Standar Ports
/// Wraps call to sed with replace parameters
pub fn replace_ports(target_directory: &str) {
    println!("{FORGE_UNICODE} Optional find and replace for service ports in {target_directory}");
    for port_and_service in PORTS_AND_SERVICES {
        println!(
            "{} on port {}",
            port_and_service.service_name, port_and_service.port_number
        );

        let port_prompt = format!(
            "Which port should be used for {}?",
            port_and_service.service_name
        );
        // Shadow to string:
        let target_port_number = format!("{}", port_and_service.port_number);
        let port_for_service = Text::new(&port_prompt)
            .with_default(&target_port_number)
            .prompt();

        match port_for_service {
            Ok(provided_port_from_user) => {
                println!("Provided: {}", provided_port_from_user);
                replace_text_in_target_folder(
                    &target_directory,
                    &target_port_number,
                    &provided_port_from_user,
                );
            }
            Err(_) => println!("An error happened when asking for target port."),
        }

        // TODO
        // Write a message about changing
        // Call a process wrapping sed to do a find and replace for the port number
        // Commit the changes
    }
}
