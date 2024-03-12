// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]
#![allow(warnings)]

use git2::{FetchOptions, RemoteCallbacks};
use std::path::Path;

use irox_progress::irox_time::Duration;
use irox_progress::{ProgressPrinter, Task};
use log::Level;

pub const REPO_URL: &str = "https://github.com/microsoft/winget-pkgs.git";
pub const LOCAL_REPO: &str = ".winget_repo";

pub fn main() {
    irox_log::init_console_level(Level::Info);
    let progress =
        irox_progress::console::ConsoleProgressPrinter::new_update_rate(Duration::from_millis(100));
    let _repo = git2::Repository::open(LOCAL_REPO).unwrap_or_else(|_| {
        log::info!("Cloning {REPO_URL} into {LOCAL_REPO}");

        let mut remotecb = RemoteCallbacks::new();
        let packtask = Task::new_infinite_named("Packfile".to_string());
        progress.track_task_progress(&packtask);
        remotecb.pack_progress(move |stage, current, total| {
            packtask.mark_started();
            packtask.set_max_elements(total as u64);
            packtask.set_current_progress_count(current as u64);
            if current == total {
                packtask.mark_all_completed();
            }
        });
        let dltask = Task::new_infinite_named("download".to_string());
        progress.track_task_progress(&dltask);
        remotecb.transfer_progress(move |prg| {
            dltask.mark_started();

            // prg.

            dltask.is_cancelled()
        });
        let mut fetch = FetchOptions::new();
        fetch.remote_callbacks(remotecb);

        git2::build::RepoBuilder::new()
            .bare(true)
            .fetch_options(fetch)
            .clone(REPO_URL, Path::new(LOCAL_REPO))
            .unwrap()
    });
}
