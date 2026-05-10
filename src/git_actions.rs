use git2::{IndexAddOption, Repository};

pub fn commit_to_directory(target_directory: &str, commit_message: &str) -> Result<git2::Oid, git2::Error> {
    let repo = Repository::open(target_directory)?;

    let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    let signature = repo.signature()?;
    let parent_commit = repo.head()?.peel_to_commit()?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        commit_message,
        &tree,
        &[&parent_commit],
    )
}
