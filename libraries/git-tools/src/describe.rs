// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use std::path::Path;

use git2::{DiffOptions, ObjectType, StatusOptions, StatusShow, Tag};

use crate::error::Error;

pub struct DescribeResult {
    pub found_commit_hash: String,
    pub description: String,
    pub is_dirty: bool,
    pub tag_name: Option<String>,
    // pub
}

pub fn describe<T: AsRef<Path>>(directory: T, prefix: Option<&str>) -> Result<String, Error> {
    let start_path = directory.as_ref();
    let repo = crate::discover_repo_or_worktree_at(start_path)?;
    let work_dir = repo.workdir().unwrap_or(repo.path());
    let start_rel_path = start_path.strip_prefix(work_dir).ok();

    let head = crate::get_head_for_repo(&repo)?;
    let Ok(head_tree) = head.tree() else {
        return Error::no_tree_for_commit_err(&repo, &head);
    };

    let Ok(mut walk) = repo.revwalk() else {
        return Error::revwalk_err(&repo);
    };
    if let Err(_e) = walk.push(head.id()) {
        return Error::id_not_in_repo_err(&repo, &head);
    }

    let status = repo
        .statuses(Some(
            StatusOptions::new()
                .show(StatusShow::IndexAndWorkdir)
                .update_index(true)
                .include_untracked(true),
        ))
        .map_err(|e| Error::CommandError {
            path: repo.path().display().to_string(),
            cmd: "status".to_string(),
            error: e.to_string(),
        })?;
    let is_dirty = !status.is_empty();
    let dirty = if is_dirty { "-dirty" } else { "" };
    let prefix = prefix.map(|p| format!("{p}-")).unwrap_or_default();

    let mut tags: Vec<Tag> = Vec::new();
    repo.tag_foreach(|oid, _name| {
        if let Ok(tag) = repo.find_tag(oid) {
            tags.push(tag);
        }
        true
    })
    .map_err(|e| Error::CommandError {
        path: repo.path().display().to_string(),
        cmd: "tag_foreach".to_string(),
        error: e.to_string(),
    })?;

    for elem in walk {
        let elem = elem.map_err(|_e| Error::revwalk(&repo))?;
        let commit_obj = repo
            .find_object(elem, Some(ObjectType::Commit))
            .map_err(|_e| Error::id_not_in_repo(&repo, &elem))?;
        let commit = commit_obj
            .peel_to_commit()
            .map_err(|_e| Error::id_not_in_repo(&repo, &commit_obj.id()))?;
        let older_tree = commit
            .tree()
            .map_err(|_e| Error::no_tree_for_commit(&repo, &commit))?;
        let short_id = commit_obj
            .short_id()
            .ok()
            .map(|idbuf| String::from_utf8_lossy(idbuf.as_ref()).to_string());
        let id = short_id.unwrap_or("?".to_string());
        let mut opts = DiffOptions::new();
        if let Some(pathspec) = start_rel_path {
            opts.pathspec(pathspec);
        }

        let diff = repo
            .diff_tree_to_tree(Some(&older_tree), Some(&head_tree), Some(&mut opts))
            .map_err(|_e| Error::diff_tree(&repo, &older_tree, &head_tree))?;
        if diff.get_delta(0).is_some() {
            // found it.

            let res = format!("{prefix}g{id}{dirty}");
            return Ok(res);
        }
    }
    Ok("Unable to describe repo".to_string())
}
