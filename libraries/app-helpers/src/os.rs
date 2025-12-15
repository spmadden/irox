// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

cfg_windows! {
    pub mod win;
}

use irox_tools::cfg_windows;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OperatingSystem {
    Wasm,
    Windows,
    Linux,
    Other(&'static str),
}

impl OperatingSystem {
    pub const fn identify_by_target() -> Self {
        if cfg!(target_arch = "wasm32") {
            OperatingSystem::Wasm
        } else if cfg!(target_os = "windows") {
            OperatingSystem::Windows
        } else if cfg!(target_os = "linux") {
            OperatingSystem::Linux
        } else {
            OperatingSystem::Other(std::env::consts::OS)
        }
    }

    pub fn home_dir(&self) -> Option<PathBuf> {
        None
    }

    pub fn appdata_local(&self) -> Option<PathBuf> {
        None
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ShellFolder {
    Cache,
    Desktop,
    Documents,
    Downloads,
    Pictures,
    Home,
    Screenshots,

    AppDataLocal,
    AppDataRoaming,
}

impl ShellFolder {
    pub fn get_path(&self) -> Option<PathBuf> {
        let os = OperatingSystem::identify_by_target();
        match self {
            ShellFolder::Home => {
                return os.home_dir();
            }
            ShellFolder::AppDataLocal => {
                return os.appdata_local();
            }
            _ => {}
        }
        None
    }
}
