use git2::Repository;

fn clone_repo(url: &str, path: &str) {
    let _repo = match Repository::clone(url, path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };
    println!("Cloned {} to {}", url, path);
}


pub fn create_new_djforge(repository: &'static str,target_branch: &'static str, target_directory: &str) {
    println!("Clone {repository} @ {target_branch} in {target_directory}");
    _ = clone_repo(repository, target_directory);
}