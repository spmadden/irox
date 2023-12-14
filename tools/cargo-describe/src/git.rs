// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use std::path::{Path, PathBuf};

use cargo::core::Workspace;
use git2::{DiffOptions, Error, ObjectType, Repository, StatusOptions, StatusShow};

use crate::do_manifest_log_for_member;

pub struct Context {
    repository: Repository,

    members: Vec<MemberInfo>,
}

pub struct MemberInfo {
    pub prefix: String,
    pub repopath: PathBuf,
    pub krate: String,
    pub result: Option<String>,
}

pub fn do_git_log(wksp: &Workspace) -> Vec<MemberInfo> {
    let path = wksp.root();
    let Ok(repo) = Repository::discover(path) else {
        return Vec::new();
    };
    let repo = if repo.is_worktree() {
        let repo_path = repo.path();
        let Ok(repo) = Repository::open(repo_path) else {
            return Vec::new();
        };
        repo
    } else {
        repo
    };
    let workdir = repo.workdir().unwrap_or(Path::new("."));

    let mut members = Vec::new();
    if let Some(cur) = wksp.current_opt() {
        if !wksp.is_virtual() {
            let repopath = cur.root();
            let repopath = repopath.strip_prefix(workdir).unwrap_or(repopath);
            members.push(MemberInfo {
                prefix: do_manifest_log_for_member(cur),
                repopath: repopath.into(),
                krate: cur.name().to_string(),
                result: None,
            });
        }
    }
    if members.is_empty() {
        for cur in wksp.members() {
            let repopath = cur.root();
            let repopath = repopath.strip_prefix(workdir).unwrap_or(repopath);
            members.push(MemberInfo {
                prefix: do_manifest_log_for_member(cur),
                repopath: repopath.into(),
                krate: cur.name().to_string(),
                result: None,
            });
        }
    }
    let context = Context {
        repository: repo,
        members,
    };
    try_do_git_log_for_members(context).unwrap_or_default()
}

pub fn try_do_git_log_for_members(mut context: Context) -> Result<Vec<MemberInfo>, Error> {
    let repo = &context.repository;

    let head_tree = repo.head()?.peel_to_commit()?.tree()?;

    let mut walk = repo.revwalk()?;
    walk.push_head()?;

    let status = repo.statuses(Some(
        StatusOptions::new()
            .show(StatusShow::IndexAndWorkdir)
            .update_index(true)
            .include_untracked(true),
    ))?;
    let dirty = if status.is_empty() {
        String::from("-dirty")
    } else {
        String::new()
    };

    for (idx, elem) in walk.enumerate() {
        let elem = elem?;

        let commit_obj = repo.find_object(elem, Some(ObjectType::Commit))?;
        let commit = commit_obj.peel_to_commit()?;
        let older_tree = commit.tree()?;
        let short_id = commit_obj.short_id()?;
        let id = short_id.as_str().unwrap_or("?");

        let mut still_working = false;
        for cur in &mut context.members {
            if cur.result.is_some() {
                continue;
            }
            still_working = true;
            let mut opts = DiffOptions::new();
            opts.pathspec(&cur.repopath);

            let diff =
                repo.diff_tree_to_tree(Some(&older_tree), Some(&head_tree), Some(&mut opts))?;
            if diff.get_delta(0).is_some() {
                // found it.

                let res = format!("{}-{idx}-g{id}{dirty}", cur.prefix);
                cur.result = Some(res);
            }
        }
        if !still_working {
            break;
        }
    }

    for cur in &context.members {
        if let Some(res) = &cur.result {
            println!("{res}")
        }
    }

    Ok(context.members)
}
