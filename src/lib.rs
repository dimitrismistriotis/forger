use std::path::Path;
use std::process;

use git2::Repository;

const FORGE_UNICODE: &str = "\u{1F528} "; // Space included

fn clone_repo(url: &str, path: &str) -> git2::Repository {
    let repo = match Repository::clone(url, path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };
    println!("Cloned {} to {}", url, path);

    repo
}

pub fn create_new_djforge(
    repository: &'static str,
    target_branch: &'static str,
    target_directory: &str,
) {
    println!("{FORGE_UNICODE} Clone {repository} @ {target_branch} in {target_directory}");

    // Guard Clauses

    let path = Path::new(target_directory);
    if path.is_dir() {
        println!("\nDirectory {target_directory} already exists, exiting...");
        process::exit(1);
    }

    let target_repository = clone_repo(repository, target_directory);
    println!("Empty: {}", target_repository.is_empty().unwrap());
}
