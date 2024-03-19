// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::directory::DirectoryStream;
use crate::{Error, Inode};

pub trait DirOps<T> {
    fn list(&self) -> DirectoryStream<T>;
}

pub trait MutDirOps<T> {
    fn new_file(&self, filename: &str) -> Result<Inode<T>, Error>;
    fn new_directory(&self, dirname: &str) -> Result<Inode<T>, Error>;
}
