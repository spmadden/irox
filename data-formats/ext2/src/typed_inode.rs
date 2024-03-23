// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

use irox_tools::bits::Error;

use crate::data::DataStream;
use crate::directory::DirectoryStream;
use crate::inode::{BlockIter, Inode};
use crate::ops::{DirOps, MutDirOps};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GenericInode;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Directory;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RegFile;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Socket;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BlockDev;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CharDev;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SymLink;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Fifo;

pub struct TypedInode<T> {
    inode: Arc<Inode>,
    _ph: PhantomData<T>,
}

impl<T> TypedInode<T> {
    pub fn new(inode: Arc<Inode>) -> Self {
        TypedInode {
            inode,
            _ph: Default::default(),
        }
    }

    pub fn block_stream(self: &Arc<Self>) -> Result<BlockIter, Error> {
        Inode::block_stream(&self.inode)
    }
    pub fn data_stream(self: &Arc<Self>) -> Result<DataStream, Error> {
        Inode::data_stream(&self.inode)
    }
}

impl<T> Deref for TypedInode<T> {
    type Target = Inode;

    fn deref(&self) -> &Self::Target {
        &self.inode
    }
}

pub enum InodeType {
    Socket(TypedInode<Socket>),
    SymLink(TypedInode<SymLink>),
    RegularFile(TypedInode<RegFile>),
    BlockDevice(TypedInode<BlockDev>),
    Directory(TypedInode<Directory>),
    CharDevice(TypedInode<CharDev>),
    Fifo(TypedInode<Fifo>),
}

impl DirOps for TypedInode<Directory> {
    fn list(self: &Arc<Self>) -> Result<DirectoryStream, Error> {
        let datastream = self.data_stream()?;
        Ok(DirectoryStream::new(datastream))
    }
}

impl MutDirOps for TypedInode<Directory> {
    fn new_file(&self, _filename: &str) -> Result<TypedInode<RegFile>, Error> {
        // 1. find a new/unused Inode number for the file
        // 2. Create the new directory linked list structure with appended dirent
        // 3. Return a struct that's MutBits to append data to the file
        todo!()
    }

    fn new_directory(&self, _dirname: &str) -> Result<TypedInode<Directory>, Error> {
        todo!()
    }
}