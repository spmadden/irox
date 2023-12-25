// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use std::path::Path;
use std::rc::Rc;

use cargo::core::Workspace;
use git2::{
    DescribeFormatOptions, DescribeOptions, DiffOptions, Error, ObjectType, Repository,
    StatusOptions, StatusShow, Tag,
};

use crate::do_manifest_log_for_member;

pub struct Context {
    repository: Repository,

    members: Vec<MemberInfo>,
}

pub struct MemberInfo {
    pub prefix: String,
    pub repopath: String,
    pub krate: String,
    pub result: Option<String>,
    pub count_since_result: u32,
    pub found_exact_tag: bool,
    pub found_any_tag: Option<(String, u32)>,
    pub found_id: Option<String>,
    pub describe_result: Option<String>,
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
    let workdir = repo
        .workdir()
        .unwrap_or(Path::new("."))
        .display()
        .to_string()
        .replace('\\', "/")
        .to_string()
        .to_lowercase();

    let mut members = Vec::new();
    if let Some(cur) = wksp.current_opt() {
        if !wksp.is_virtual() {
            let repopath = cur
                .root()
                .display()
                .to_string()
                .replace('\\', "/")
                .strip_prefix(workdir.as_str())
                .map(ToString::to_string)
                .unwrap_or_default()
                .to_lowercase();
            members.push(MemberInfo {
                prefix: do_manifest_log_for_member(cur),
                repopath,
                krate: cur.name().to_string(),
                result: None,
                count_since_result: 0,
                found_exact_tag: false,
                found_any_tag: None,
                found_id: None,
                describe_result: None,
            });
        }
    }
    if members.is_empty() {
        for cur in wksp.members() {
            let repopath = cur
                .root()
                .display()
                .to_string()
                .replace('\\', "/")
                .to_lowercase()
                .strip_prefix(workdir.as_str())
                .map(ToString::to_string)
                .unwrap_or_default();
            members.push(MemberInfo {
                prefix: do_manifest_log_for_member(cur),
                repopath,
                krate: cur.name().to_string(),
                result: None,
                count_since_result: 0,
                found_exact_tag: false,
                found_any_tag: None,
                found_id: None,
                describe_result: None,
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

    let mut tags: Vec<Rc<Tag>> = Vec::new();
    repo.tag_foreach(|oid, _name| {
        if let Ok(tag) = repo.find_tag(oid) {
            tags.push(Rc::new(tag));
        }
        true
    })?;

    let mut newer_tree = head_tree;
    for elem in walk {
        let elem = elem?;

        let commit_obj = repo.find_object(elem, Some(ObjectType::Commit))?;
        let commit = commit_obj.peel_to_commit()?;
        let older_tree = commit.tree()?;
        let short_id = commit_obj.short_id()?;
        let id = short_id.as_str().unwrap_or("?");

        let relevant_tags: Vec<Rc<Tag>> = tags
            .iter()
            .filter(|tag| tag.target_id() == elem)
            .map(Rc::clone)
            .collect();

        let mut still_working = false;
        for cur in &mut context.members {
            if cur.found_exact_tag || cur.describe_result.is_some() {
                continue;
            }

            still_working = true;

            let any_tag = relevant_tags.first().map(Rc::clone);
            let exact_tag = relevant_tags
                .iter()
                .find(|t| String::from_utf8_lossy(t.name_bytes()) == cur.prefix)
                .map(Rc::clone);
            if exact_tag.is_some() {
                cur.found_id = Some(id.to_string());
                cur.found_exact_tag = true;
            } else if let Some(any_tag) = any_tag {
                if cur.found_any_tag.is_none() {
                    let name = String::from_utf8_lossy(any_tag.name_bytes()).to_string();
                    cur.found_any_tag = Some((name, cur.count_since_result));
                }
            }

            let mut opts = DiffOptions::new();
            opts.pathspec(&cur.repopath);

            let diff =
                repo.diff_tree_to_tree(Some(&older_tree), Some(&newer_tree), Some(&mut opts))?;
            if diff.get_delta(0).is_some() {
                // found it.
                if cur.found_id.is_none() {
                    cur.found_id = Some(id.to_string());
                    let mut opts = DescribeOptions::new();
                    opts.describe_all();
                    opts.pattern(cur.prefix.as_str());
                    let mut fmt = DescribeFormatOptions::new();
                    fmt.always_use_long_format(true);
                    if let Ok(desc) = commit_obj.describe(&opts) {
                        if let Ok(desc) = desc.format(Some(&fmt)) {
                            cur.describe_result = Some(desc);
                        }
                    }
                }

                cur.count_since_result += 1;
            }
        }
        newer_tree = older_tree;
        if !still_working {
            break;
        }
    }

    // update the inner results with the found tags.
    for mem in &mut context.members {
        if let Some(desc) = &mem.describe_result {
            mem.result = Some(desc.clone());
            continue;
        }
        let Some(id) = &mem.found_id else {
            mem.result = Some("unknown".to_string());
            continue;
        };
        let prefix = &mem.prefix;
        if mem.found_exact_tag {
            let idx = mem.count_since_result;
            if idx == 0 {
                mem.result = Some(format!("{prefix}-g{id}{dirty}"));
            } else {
                mem.result = Some(format!("{prefix}-{idx}-g{id}{dirty}"));
            }
            continue;
        }
        if let Some((tag, idx)) = &mem.found_any_tag {
            let id = mem.found_id.clone().unwrap_or_default();
            mem.result = Some(format!("{tag}-{idx}-g{id}{dirty}"));
            continue;
        }
        mem.result = Some(format!("{prefix}-g{id}{dirty}"));
    }

    Ok(context.members)
}
