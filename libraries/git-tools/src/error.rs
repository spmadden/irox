// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use git2::{Commit, Oid, Reference, Repository, Tree};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum Error {
    Discovery {
        path: String,
    },
    Worktree {
        path: String,
    },
    Head {
        path: String,
    },
    ReferenceNotInRepo {
        path: String,
        reference: String,
    },
    NoTreeForCommit {
        path: String,
        commit: String,
        tree: String,
    },
    Revwalk {
        path: String,
    },
    CommandError {
        path: String,
        cmd: String,
        error: String,
    },
    Diff {
        path: String,
        tree1: String,
        tree2: String,
    },
    Branch {
        path: String,
        name: String,
    },
}

impl Error {
    pub fn discovery<T>(path: &Path) -> Result<T, Error> {
        Err(Error::Discovery {
            path: path.display().to_string(),
        })
    }
    pub fn worktree<T>(path: &Path) -> Result<T, Error> {
        Err(Error::Worktree {
            path: path.display().to_string(),
        })
    }
    pub fn head<T>(path: &Path) -> Result<T, Error> {
        Err(Error::Head {
            path: path.display().to_string(),
        })
    }
    pub fn ref_not_in_repo(repo: &Repository, reference: &Reference) -> Self {
        Error::ReferenceNotInRepo {
            path: repo.path().display().to_string(),
            reference: String::from_utf8_lossy(reference.name_bytes()).to_string(),
        }
    }
    pub fn ref_not_in_repo_err<T>(repo: &Repository, reference: &Reference) -> Result<T, Error> {
        Err(Error::ref_not_in_repo(repo, reference))
    }
    pub fn id_not_in_repo(repo: &Repository, id: &Oid) -> Self {
        Error::ReferenceNotInRepo {
            path: repo.path().display().to_string(),
            reference: id.to_string(),
        }
    }
    pub fn id_not_in_repo_err<T>(repo: &Repository, commit: &Commit) -> Result<T, Error> {
        Err(Error::id_not_in_repo(repo, &commit.id()))
    }
    pub fn no_tree_for_commit(repo: &Repository, commit: &Commit) -> Self {
        Error::NoTreeForCommit {
            path: repo.path().display().to_string(),
            commit: commit.id().to_string(),
            tree: commit.tree_id().to_string(),
        }
    }
    pub fn no_tree_for_commit_err<T>(repo: &Repository, commit: &Commit) -> Result<T, Error> {
        Err(Error::no_tree_for_commit(repo, commit))
    }
    pub fn revwalk_err<T>(repo: &Repository) -> Result<T, Error> {
        Err(Error::revwalk(repo))
    }

    pub fn revwalk(repo: &Repository) -> Error {
        Error::Revwalk {
            path: repo.path().display().to_string(),
        }
    }
    pub fn command_err<T>(
        repo: &Repository,
        cmd: &'static str,
        err: &git2::Error,
    ) -> Result<T, Error> {
        Err(Error::CommandError {
            path: repo.path().display().to_string(),
            cmd: cmd.to_string(),
            error: format!("{err}"),
        })
    }
    pub fn diff_tree(repo: &Repository, tree1: &Tree, tree2: &Tree) -> Self {
        Error::Diff {
            path: repo.path().display().to_string(),
            tree1: tree1.id().to_string(),
            tree2: tree2.id().to_string(),
        }
    }
    pub fn diff_tree_err<T>(repo: &Repository, tree1: &Tree, tree2: &Tree) -> Result<T, Self> {
        Err(Error::diff_tree(repo, tree1, tree2))
    }
    pub fn branch_err<T>(repo: &Repository, name: &str) -> Result<T, Error> {
        Err(Error::Branch {
            path: repo.path().display().to_string(),
            name: name.to_string(),
        })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Discovery { path } => write!(f, "Unable to discover repo at {path}"),
            Error::Worktree { path } => write!(f, "Unable to repo worktree at {path}"),
            Error::Head { path } => write!(f, "Unable to find HEAD for repo at {path}"),
            Error::ReferenceNotInRepo { path, reference } => write!(
                f,
                "Unable to find commit for HEAD:{reference} within repo at {path}"
            ),
            Error::NoTreeForCommit { path, commit, tree } => write!(
                f,
                "Unable to find tree for commit {commit} in repo {path} with id {tree}"
            ),
            Error::Revwalk { path } => write!(f, "Error creating/executing revwalk in {path}"),

            Error::CommandError { path, cmd, error } => {
                write!(f, "Error running command {cmd} in repo {path}: {error}")
            }
            Error::Diff { path, tree1, tree2 } => write!(
                f,
                "Error generating diff between {tree1} and {tree2} in repo {path}"
            ),
            Error::Branch { path, name } => {
                write!(f, "Unable to find local branch {name} in repo {path}")
            }
        }
    }
}

impl std::error::Error for Error {}
