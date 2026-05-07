use std::path::Path;
use std::process;

use git2::Repository;
use yes_or_no::yes_or_no;
use inquire::Text;

#[derive(Debug)]
struct PortAndService {
    port_number: i16,
    service_name: &'static str,
}

const FORGE_UNICODE: &str = "\u{1F528} "; // Space included
const CONFIGURATION_BRANCH_NAME: &str = "post_checkout_configuration";
const PORTS_AND_SERVICES: [PortAndService; 1] = [PortAndService {
    port_number: 111,
    service_name: "111f",
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

    match yes_or_no("{FORGE_UNICODE} Proceed to port number replace?", true) {
        Ok(Some(true)) => replace_ports(target_directory),
        Ok(Some(false)) => println!("{FORGE_UNICODE} Proceeding without port replacement"),
        Ok(None) => println!(""),
        Err(_) => {
            process::exit(1);
        }
    }
}

pub fn replace_ports(target_directory: &str) {
    println!("{FORGE_UNICODE} Optional find and replace for service ports in {target_directory}");
    for port_and_service in PORTS_AND_SERVICES {
        println!(
            "{} ==> {}",
            port_and_service.service_name, port_and_service.port_number
        );

        let port_for_service = Text::new("What is your name?").prompt();

        match port_for_service {
            Ok(name) => println!("Hello {}", name),
            Err(_) => println!("An error happened when asking for your name, try again later."),
        }

        // TODO
        // Write a message about changing
        // Call a process wrapping sed to do a find and replace for the port number
        // Commit the changes
    }
}
