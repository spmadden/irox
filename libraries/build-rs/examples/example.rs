// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use git2::{BranchType, DescribeFormatOptions, DescribeOptions, StatusOptions};

pub fn main() -> Result<(), git2::Error> {
    let mut repo = git2::Repository::discover(".")?;

    {
        let head = repo.head()?;

        println!("{:?}", head.name());
        println!("{:?}", head.target());
        println!("{:?}", head.kind());
        println!("{:?}", head.peel_to_commit());
    }
    println!("worktree: {}", repo.is_worktree());
    println!("path: {:?}", repo.path());
    let is_worktree = repo.is_worktree();
    if is_worktree {
        repo = git2::Repository::open(repo.path())?;
    }

    let desc = repo.describe(&DescribeOptions::new().describe_all())?;
    let desc = desc.format(Some(
        DescribeFormatOptions::new()
            .dirty_suffix("-dirty")
    ))?;
    println!("{:?}", desc);

    let reference = repo.head()?;
    println!("{:?}", reference.name());
    let name = reference.shorthand().unwrap();

    let branch = repo.find_branch(name, BranchType::Local)?;
    println!("{:?}", branch.name()?);
    println!("{:?}", branch.upstream()?.name()?);

    let upstream = repo.branch_upstream_remote(reference.name().unwrap())?;
    let remote_name = String::from_utf8_lossy(&upstream);
    println!("{:?}", remote_name);

    let remote = repo.find_remote(remote_name.as_ref())?;
    println!("{:?}", remote.name());
    println!("{:?}", remote.url());

    let commit = reference.peel_to_commit()?;
    let author = commit.author();
    println!("{:?}", author.name());
    println!("{:?}", author.email());
    println!("{:?}", author.when());

    let statuses = repo.statuses(Some(StatusOptions::new().update_index(true)))?;
    for status in statuses.into_iter() {
        println!("{:?}", status.status());
        println!("{:?}", status.path());
    }

    Ok(())
}