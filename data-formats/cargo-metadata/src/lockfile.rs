// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::error::Error;
use crate::Metadata;
use irox_bits::Bits;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CargoLockfile {
    pub version: Option<usize>,
    #[serde(
        default,
        deserialize_with = "crate::deserialize_if_null",
        rename = "package"
    )]
    pub packages: Vec<LockfilePackage>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LockfilePackage {
    pub name: String,
    pub version: String,
    pub source: Option<String>,
    pub checksum: Option<String>,
    #[serde(default, deserialize_with = "crate::deserialize_if_null")]
    pub dependencies: Vec<String>,
}

impl CargoLockfile {
    pub fn read_file_at<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .create(false)
            .open(path)?;

        let data = file.read_all_str_lossy()?;
        Ok(toml::from_str::<Self>(&data)?)
    }
    pub fn read_from_dir<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let path = path.as_ref().join("Cargo.lock");
        Self::read_file_at(path)
    }
    pub fn read_current_dir() -> Result<Self, Error> {
        let meta = Metadata::read_current_dir()?;
        let file = if let Some(root) = &meta.workspace_root {
            PathBuf::from_str(root).unwrap_or_default()
        } else {
            std::env::current_dir()?
        };

        Self::read_from_dir(file)
    }
}
