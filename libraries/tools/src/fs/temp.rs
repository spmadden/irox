// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use std::path::{Path, PathBuf};

pub struct TempFilePath {
    path: Option<PathBuf>,
}

impl Drop for TempFilePath {
    fn drop(&mut self) {
        if let Some(path) = self.path.take() {
            let _ = std::fs::remove_file(path);
        }
    }
}
impl TempFilePath {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self {
            path: Some(path.as_ref().into()),
        }
    }
}
macro_rules! impl_from {
    ($dst:ident) => {
        impl From<PathBuf> for $dst {
            fn from(value: PathBuf) -> Self {
                Self { path: Some(value) }
            }
        }
        impl From<&PathBuf> for $dst {
            fn from(value: &PathBuf) -> Self {
                Self {
                    path: Some(value.clone()),
                }
            }
        }
    };
}
impl_from!(TempFilePath);
pub struct TempDirPath {
    path: Option<PathBuf>,
}
impl Drop for TempDirPath {
    fn drop(&mut self) {
        if let Some(path) = self.path.take() {
            let _ = std::fs::remove_dir_all(path);
        }
    }
}
impl TempDirPath {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self {
            path: Some(path.as_ref().into()),
        }
    }
}
impl_from!(TempDirPath);
