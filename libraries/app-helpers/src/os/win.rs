// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

pub mod env;
// pub mod proc;
pub mod osgeo4w;

use crate::ShellFolder;
use irox_tools::irox_bits::Error;
use std::path::PathBuf;

pub fn shell_folderid(_fldr: ShellFolder) -> Result<PathBuf, Error> {
    todo!()
}
