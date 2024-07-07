// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use crate::{BuildEnvironment, BuildVariable, Error, VariableSource, VariableType};
use git2::{StatusOptions, StatusShow};
use irox_time::datetime::UTCDateTime;
use irox_time::format::iso8601::EXTENDED_DATE_TIME_FORMAT;
use irox_time::Duration;

#[allow(dead_code)]
pub static GIT_VARIABLES: [&str; 9] = [
    "GIT_COMMIT_AUTHOR",
    "GIT_COMMIT_DATETIME",
    "GIT_COMMIT_TIMESTAMP_SECS",
    "GIT_COMMIT_TZ_OFFSET_SECS",
    "GIT_COMMIT_FULLHASH",
    "GIT_COMMIT_SHORTHASH",
    "GIT_BRANCH",
    "GIT_DESCRIBE",
    "GIT_IS_CLEAN",
];

macro_rules! add_str_varbl {
    ($name:literal, $env:ident, $val:ident) => {
        $env.variables.insert(
            $name.to_string(),
            BuildVariable {
                source: VariableSource::Git,
                name: $name.to_string(),
                value: VariableType::String($val.to_string()),
            },
        );
    };
}
macro_rules! add_bool_varbl {
    ($name:literal, $env:ident, $val:ident) => {
        $env.variables.insert(
            $name.to_string(),
            BuildVariable {
                source: VariableSource::Git,
                name: $name.to_string(),
                value: VariableType::Bool($val),
            },
        );
    };
}

macro_rules! add_int_varbl {
    ($name:literal, $env:ident, $val:ident) => {
        $env.variables.insert(
            $name.to_string(),
            BuildVariable {
                source: VariableSource::Git,
                name: $name.to_string(),
                value: VariableType::Integer($val),
            },
        );
    };
}

#[cfg(feature = "git")]
pub fn load_git_variables(env: &mut BuildEnvironment) -> Result<(), Error> {
    let start_dir = std::env!("CARGO_MANIFEST_DIR");
    let prefix = std::env!("CARGO_PKG_NAME");
    let repo = irox_git_tools::discover_repo_or_worktree_at(start_dir)?;

    let head = irox_git_tools::get_head_for_repo(&repo)?;
    let full_hash = head.id().to_string();
    let short_hash = head
        .as_object()
        .short_id()
        .map(|v| String::from_utf8_lossy(&v).to_string())
        .unwrap_or_default();
    add_str_varbl!("GIT_COMMIT_FULLHASH", env, full_hash);
    add_str_varbl!("GIT_COMMIT_SHORTHASH", env, short_hash);

    let branch = irox_git_tools::get_branch_for_head(&repo)?;
    let branch = String::from_utf8_lossy(branch.name_bytes()?).to_string();
    add_str_varbl!("GIT_BRANCH", env, branch);

    let author = head.author();
    let name = author.name().unwrap_or_default();
    let email = author.email().unwrap_or_default();

    let committer = format!("{name} <{email}>");
    add_str_varbl!("GIT_COMMIT_AUTHOR", env, committer);

    let when = author.when();
    let seconds = when.seconds();
    let offset_seconds = when.offset_minutes() as i64 * 60;
    add_int_varbl!("GIT_COMMIT_TIMESTAMP_SECS", env, seconds);
    add_int_varbl!("GIT_COMMIT_TZ_OFFSET_SECS", env, offset_seconds);

    let time = irox_time::epoch::UnixTimestamp::from_offset(Duration::from_seconds(seconds as u64));
    let time = Into::<UTCDateTime>::into(time).format(&EXTENDED_DATE_TIME_FORMAT);
    add_str_varbl!("GIT_COMMIT_DATETIME", env, time);

    let describe = irox_git_tools::describe(start_dir, Some(prefix))
        .ok()
        .unwrap_or_default();
    add_str_varbl!("GIT_DESCRIBE", env, describe);

    let status = repo.statuses(Some(
        StatusOptions::new()
            .show(StatusShow::IndexAndWorkdir)
            .update_index(true)
            .include_untracked(true),
    ))?;
    let clean = status.is_empty();

    add_bool_varbl!("GIT_IS_CLEAN", env, clean);

    Ok(())
}
