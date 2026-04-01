use gix::{
    config::Source,
    refs::transaction::{Change, LogChange},
};
use std::{
    error::Error,
    fs::{self, OpenOptions, read_dir},
    path::Path,
};

/// Initializes a new bare Git repository at the specified directory.
///
/// This function creates a new bare repository and configures it with specific settings
/// to facilitate shared access and easier pushing and pulling.
///
/// The following configurations are applied:
/// - `core.sharedRepository`: Set to `2` to enable group read/write permissions.
/// - `receive.denyNonFastforwards`: Set to `false` to allow non-fast-forward pushes.
/// - `receive.denyDeletes`: Set to `false` to allow branch deletions.
/// - `receive.denyCurrentBranch`: Set to `false` to allow pushing to the current branch.
/// - `http.receivepack`: Set to `true` to enable receiving objects via HTTP.
/// - `http.uploadpack`: Set to `true` to enable serving objects via HTTP.
///
/// Additionally, the `post-update` hook is copied from its sample file to enable it.
///
/// # Arguments
///
/// * `directory` - A `Path` to the directory where the bare repository should be initialized.
///
/// This directory will be created if it doesn't exist.
///
/// # Returns
///
/// A `Result` containing the initialized `gix::Repository` object if successful,
/// or a `Box<dyn Error>` if any operation fails.
pub fn init_bare_repository(directory: &Path) -> Result<gix::Repository, Box<dyn Error>> {
    let repo = gix::init_bare(directory)?;

    let repo_path = repo.path();

    // Construct the path to the config file, ensuring it's relative to the repo root
    let config_path = repo_path.join("config");
    let mut config = gix::config::File::from_path_no_includes(config_path, Source::Local)?;

    config.set_raw_value_by("core", None, "sharedrepository", "2")?;
    config.set_raw_value_by("receive", None, "denyNonFastforwards", "false")?;
    config.set_raw_value_by("receive", None, "denyDeletes", "false")?;
    config.set_raw_value_by("receive", None, "denyCurrentBranch", "false")?;
    config.set_raw_value_by("http", None, "receivepack", "true")?;
    config.set_raw_value_by("http", None, "uploadpack", "true")?;

    // Open the config file for writing
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(repo_path.join("config"))?;

    config.write_to(&mut file)?;

    // Copy the post-update hook
    fs::copy(
        repo_path.join("hooks/post-update.sample"),
        repo_path.join("hooks/post-update"),
    )?;

    fs::File::create_new(repo_path.join("info/refs"))?;

    Ok(repo)
}

/// Lists all bare Git repositories found within a given directory.
///
/// Iterates through the entries in the provided `path`. For each entry, it attempts to
/// open it as a Git repository. If the repository is successfully opened and is
/// a bare repository, its name (the directory name) is collected.
///
/// # Arguments
///
/// * `path` - A `Path` to the directory to search for bare repositories.
///
/// # Returns
///
/// A `Result` containing a `Vec<String>` of the names of the bare repositories
/// found, or an error if any I/O operation fails. If the provided `path` is not
/// a directory, an empty `Vec` is returned.
pub fn list_bare_repositories(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    if !path.is_dir() {
        return Ok(Vec::new());
    }

    let repos = read_dir(path)?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let entry_path = entry.path();
            let repo = gix::open(&entry_path).ok()?;
            if repo.is_bare() {
                entry_path.file_name()?.to_str().map(String::from)
            } else {
                None
            }
        })
        .collect();

    Ok(repos)
}

/// Sets the HEAD reference of a Git repository to point to a specific branch
///
/// # Arguments
/// * `repo_path` - The path to the Git repository to modify
/// * `branch` - The name of the branch to set as HEAD
///
/// # Returns
///
/// A `Result` indicating success or failure.
pub fn set_repository_head(repo_path: &Path, branch: &str) -> Result<(), Box<dyn Error>> {
    let repository = gix::open(repo_path)?;

    if !repository.is_bare() {
        return Err(Box::<dyn Error>::from(format!(
            "'{}' is not a bare repository",
            repo_path.display()
        )));
    }

    let branch_full_name = format!("refs/heads/{branch}");
    let edit = gix::refs::transaction::RefEdit {
        change: Change::Update {
            log: LogChange::default(),
            expected: gix::refs::transaction::PreviousValue::Any,
            new: gix::refs::Target::Symbolic(branch_full_name.try_into()?),
        },
        name: "HEAD".try_into()?,
        deref: false,
    };
    repository.edit_reference(edit)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_init_bare_repository() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("test_repo.git");

        let repo = init_bare_repository(&repo_path).unwrap();

        assert!(repo.is_bare());
        assert!(repo_path.exists());
        assert!(repo_path.join("HEAD").exists());
        assert!(repo_path.join("config").exists());
    }

    #[test]
    fn test_list_bare_repositories() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        let bare_repo_path1 = base_path.join("repo1.git");
        init_bare_repository(&bare_repo_path1).unwrap();

        let bare_repo_path2 = base_path.join("repo2.git");
        init_bare_repository(&bare_repo_path2).unwrap();

        let non_bare_repo_path = base_path.join("non_bare");
        gix::init(&non_bare_repo_path).unwrap();

        let regular_dir = base_path.join("regular_dir");
        fs::create_dir(&regular_dir).unwrap();

        let mut repos = list_bare_repositories(base_path).unwrap();
        repos.sort();

        assert_eq!(repos.len(), 2);
        assert_eq!(repos[0], "repo1.git");
        assert_eq!(repos[1], "repo2.git");
    }
}
