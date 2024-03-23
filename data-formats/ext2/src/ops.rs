// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::directory::DirectoryStream;
use crate::typed_inode::{RegFile, TypedInode, Directory};
use crate::{Error};
use std::sync::Arc;

pub trait DirOps {
    fn list(self: &Arc<Self>) -> Result<DirectoryStream, Error>;
}

pub trait MutDirOps{
    fn new_file(&self, filename: &str) -> Result<TypedInode<RegFile>, Error>;
    fn new_directory(&self, dirname: &str) -> Result<TypedInode<Directory>, Error>;
}

