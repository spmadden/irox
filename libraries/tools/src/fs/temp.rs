// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use core::fmt::{Debug, Formatter};
use crate::cfg_feature_std;
use std::path::{Path, PathBuf};

pub struct TempFilePath {
    path: PathBuf,
}
impl Debug for TempFilePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "TempFile({:?})", self.path)
    }
}

impl Drop for TempFilePath {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
impl TempFilePath {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self {
            path: path.as_ref().into(),
        }
    }
    /// Returns the temporary path.
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    cfg_feature_std! {
        /// Creates a new temporary file with the default prefix of `.tmpirxf_`
        /// that will be deleted when this object is dropped.
        pub fn new_tempfile() -> Result<Self, std::io::Error> {
            Self::new_tempfile_prefixed(".tmpirxf_")
        }
        /// Creates a new temporary file with the provided prefix that will be
        /// deleted when this object is dropped.
        pub fn new_tempfile_prefixed(prefix: &str) -> Result<Self, std::io::Error> {
            let prefix = crate::fs::clean_filename(&prefix);
            let mut tmpdir = std::env::temp_dir();
            tmpdir.push(format!("{prefix}{}", crate::uuid::UUID::new_random()));
            let p = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .truncate(true)
                .open(&tmpdir)?;
            drop(p);
            Ok(TempFilePath {
                path: tmpdir
            })
        }
        pub fn open(&self, opts: std::fs::OpenOptions) -> Result<std::fs::File, std::io::Error> {
            opts.open(&self.path)
        }
        pub fn open_write_buffered(&self, opts: std::fs::OpenOptions) -> Result<std::io::BufWriter<std::fs::File>, std::io::Error> {
            Ok(std::io::BufWriter::new(self.open(opts)?))
        }
        pub fn open_read_buffered(&self, opts: std::fs::OpenOptions) -> Result<std::io::BufReader<std::fs::File>, std::io::Error> {
            Ok(std::io::BufReader::new(self.open(opts)?))
        }
    }
}
macro_rules! impl_from {
    ($dst:ident) => {
        impl From<PathBuf> for $dst {
            fn from(value: PathBuf) -> Self {
                Self { path: value }
            }
        }
        impl From<&PathBuf> for $dst {
            fn from(value: &PathBuf) -> Self {
                Self {
                    path: value.clone(),
                }
            }
        }
    };
}
impl_from!(TempFilePath);
pub struct TempDirPath {
    path: PathBuf,
}
impl Drop for TempDirPath {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
impl Debug for TempDirPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "TempDir({:?})", self.path)
    }
}
impl TempDirPath {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self {
            path: path.as_ref().into(),
        }
    }
    /// Returns the temporary path.
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    cfg_feature_std! {
        /// Creates a new temporary directory that will be removed upon dropping
        /// of this object with the default prefix of `.tmpirx_` and then a
        /// random UUID.
        pub fn new_temp_dir() -> Result<Self, std::io::Error> {
            Self::new_temp_dir_prefixed(".tmpirxd_")
        }
        /// Creates a new temporary directory that will be removed upon dropping
        /// of this object with the provided prefix and then a random UUID.
        pub fn new_temp_dir_prefixed(prefix: &str) -> Result<Self, std::io::Error> {
            let prefix = crate::fs::clean_filename(&prefix);
            let mut tmpdir = std::env::temp_dir();
            tmpdir.push(format!("{prefix}{}", crate::uuid::UUID::new_random()));
            std::fs::create_dir_all(&tmpdir)?;
            Ok(Self {
                path: tmpdir
            })
        }
        /// Creates a new temporary file within this directory and returns the
        /// path to that file.
        pub fn new_temp_file(&self) -> Result<TempFilePath, std::io::Error> {
            let mut f = self.path.clone();
            f.push(crate::uuid::UUID::new_random().to_string());
            let p = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .truncate(true)
                .open(&f)?;
            drop(p);
            Ok(TempFilePath {
                path: f
            })
        }

    }
}
impl_from!(TempDirPath);
