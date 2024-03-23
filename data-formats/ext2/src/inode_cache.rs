// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::collections::BTreeMap;

use irox_structs::Struct;
use irox_tools::bits::{Error, ErrorKind};

use crate::Filesystem;
use crate::inode::RawInode;

pub struct InodeCacheImpl {
    pub fs: Filesystem,
    pub cache: BTreeMap<u32, RawInode>,
}

impl InodeCacheImpl {
    pub fn get_inode(&mut self, id: u32) -> Result<RawInode, Error> {
        if !self.cache.contains_key(&id) {
            let inode = self.fs.find_inode(id)?;
            self.cache.insert(id, inode.raw_inode);
        }
        let Some(inode) = self.cache.get(&id) else {
            return Err(ErrorKind::NotFound.into())
        };
        Ok(*inode)
    }

    pub fn update_inode(&mut self, id: u32, inode: &RawInode) -> Result<(), Error>{
        if let Some(_prev) =  self.cache.insert(id, *inode) {

        };
        // serialize block
        let mut blk:[u8;4096] = [0; 4096];
        inode.write_to(&mut blk.as_mut_slice())?;
        self.fs.write_raw_4k_block(id, &blk)?;
        Ok(())
    }
}