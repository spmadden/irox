// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

pub use crate::describe::*;
pub use crate::error::Error;
use git2::{Branch, BranchType, Commit, Repository};
use std::path::Path;

///
/// Attempts to find the repository at the specified location
pub fn discover_repository<T: AsRef<Path>>(directory: T) -> Result<Repository, Error> {
    let starting_point = directory.as_ref();
    Repository::discover(starting_point).map_err(|_e| Error::Discovery {
        path: starting_point.display().to_string(),
    })
}
///
/// If the repository represents a worktree, peels it to find the real `.git` directory, and returns
/// a repository against the real `.git` directory.
pub fn git_repodir_for_repo(repo: Repository) -> Result<Repository, Error> {
    if repo.is_worktree() {
        let repo_path = repo.path();
        return Repository::open(repo_path).map_err(|_e| Error::Worktree {
            path: repo_path.display().to_string(),
        });
    }
    Ok(repo)
}

///
/// Returns a discovered [`Repository`], or if the path is a git worktree, then returns the
/// actual backed `.git` folder for this worktree in the main repository.
pub fn discover_repo_or_worktree_at<T: AsRef<Path>>(directory: T) -> Result<Repository, Error> {
    let repo = discover_repository(directory)?;
    let repo = git_repodir_for_repo(repo)?;
    Ok(repo)
}

///
/// Resolves `HEAD` to a commit for the specified repo.  Returns an error if `HEAD` doesn't exist, or `HEAD` doesn't
/// point to a commit in the provided repository
pub fn get_head_for_repo(repo: &Repository) -> Result<Commit, Error> {
    let Ok(head) = repo.head() else {
        return Error::head(repo.path());
    };
    head.peel_to_commit()
        .map_err(|_| Error::ref_not_in_repo(repo, &head))
}

///
/// Resolved the branch for `HEAD` if one exists
pub fn get_branch_for_head(repo: &Repository) -> Result<Branch, Error> {
    let Ok(reference) = repo.head() else {
        return Error::head(repo.path());
    };
    let name = String::from_utf8_lossy(reference.shorthand_bytes()).to_string();

    let Ok(branch) = repo.find_branch(&name, BranchType::Local) else {
        return Error::branch_err(repo, &name);
    };
    Ok(branch)
}
