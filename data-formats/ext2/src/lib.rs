// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};

use blocks::*;
use inode::*;
use irox_log::log::{debug, trace};
use irox_structs::Struct;
pub use irox_tools::bits::ErrorKind;
pub use irox_tools::bits::{Bits, Error, MutBits};
use superblock::*;

pub mod blocks;
pub mod data;
pub mod directory;
pub mod inode;
pub mod inode_cache;
pub mod ops;
pub mod superblock;
pub mod typed_inode;

pub struct Filesystem {
    inner: Arc<Mutex<FSInner>>,
}

impl Clone for Filesystem {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Filesystem {
    pub fn open(stream: File) -> Result<Filesystem, Error> {
        let mut inner = FSInner::new(stream)?;  
        Ok(Self {
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    pub fn get_superblock(&self) -> Result<Superblock, Error> {
        let Ok(inner) = self.inner.lock() else {
            return Err(ErrorKind::BrokenPipe.into());
        };
        Ok(inner.superblock)
    }

    pub fn find_inode(&self, inode_idx: u32) -> Result<Arc<Inode>, Error> {
        let sb = self.get_superblock()?;
        let bg_id = (inode_idx) / sb.s_inodes_per_group;
        let local_idx = (inode_idx) % sb.s_inodes_per_group;
        let inode = {
            let mut inner = self.lock()?;
            let Some(bg) = inner.block_group_table.get(bg_id as usize) else {
                return Err(ErrorKind::BrokenPipe.into());
            };
            if !bg.is_4k_inode_valid(local_idx)? {
                return Err(ErrorKind::PermissionDenied.into());
            }
            let inodes_per_block = sb.get_block_size() >> 7;
            let inode_block_id = (local_idx / inodes_per_block) + bg.raw_bgd.bg_inode_table;
            let inode_block_idx = local_idx % inodes_per_block;
            let inode_seek_offset = inode_block_idx << 7;

            debug!("Found inode {inode_idx} in block {inode_block_id} at table index {inode_block_idx} and blk offset {inode_seek_offset}");

            let block = inner.read_raw_4k_block(inode_block_id)?;
            let mut subset = block.split_at(inode_seek_offset as usize).1;
            RawInode::parse_from(&mut subset)?
        };

        Ok(Arc::new(Inode {
            raw_inode: inode,
            fs: self.clone(),
        }))
    }

    pub fn lock(&self) -> Result<MutexGuard<FSInner>, Error> {
        self.inner.lock().map_err(|_| ErrorKind::BrokenPipe.into())
    }

    pub fn read_raw_4k_block(&self, block_id: u32) -> Result<[u8; 4096], Error> {
        let mut inner = self.lock()?;
        inner.read_raw_4k_block(block_id)
    }

    pub fn read_4k_bitmap_at(&self, block_id: u32) -> Result<Bitmap<4096>, Error> {
        let mut inner = self.lock()?;
        inner.read_4k_bitmap_at(block_id)
    }
    pub fn write_raw_4k_block(&self, block_id: u32, blk: &[u8; 4096]) -> Result<(), Error> {
        let mut inner = self.lock()?;
        inner.write_raw_4k_block(block_id, blk)
    }
}

pub struct FSInner {
    disk: File,
    superblock: Superblock,
    block_group_table: Vec<BlockGroupDescriptor>,
    block_locks: Rc<RefCell<BTreeSet<u32>>>,
}

impl FSInner {
    pub fn new(mut disk: File) -> Result<Self, Error> {
        disk.seek(SeekFrom::Start(0x400))?;
        let superblock = Superblock::parse_from(&mut disk)?;
        let out = Self {
            disk,
            superblock,
            block_group_table: Vec::new(),
            block_locks: Rc::new(RefCell::new(BTreeSet::new())),
        };
        Ok(out)
    }

    pub fn update_block_group_table(
        &mut self,
        mut block_id: u32,
        filesystem: &Filesystem,
    ) -> Result<(), Error> {
        let superblock = &self.superblock;
        let num_groups = superblock.get_num_block_groups();
        debug!("Reading {num_groups} BGT entries at block {block_id}");
        let mut out = Vec::new();
        let _bgts_per_block = superblock.get_block_size() >> 5;

        let mut block = self.read_raw_4k_block(block_id)?;
        let mut block_ref = block.as_mut_slice();
        for i in 0..num_groups {
            if i > 0 && i & 0x1F == 0 {
                block_id += 1;
                block = self.read_raw_4k_block(block_id)?;
                block_ref = block.as_mut_slice();
            }
            let bgt = RawBlockGroupDescriptor::parse_from(&mut block_ref)?;
            debug!("Read BGT {i}: {bgt:?}");
            out.push(BlockGroupDescriptor::new(bgt, i, filesystem.clone())?);
        }
        self.block_group_table = out;
        Ok(())
    }

    pub fn get_superblock(&self) -> Superblock {
        self.superblock
    }

    pub fn lock_block(&mut self, block_id: u32) -> Result<BlockLock, Error> {
        {
            let Ok(mut locks) = self.block_locks.try_borrow_mut() else {
                return Err(ErrorKind::AddrInUse.into());
            };

            if !locks.insert(block_id) {
                return Err(ErrorKind::PermissionDenied.into());
            }
        }
        Ok(BlockLock {
            locked_id: block_id,
            locks: self.block_locks.clone(),
        })
    }

    pub fn read_raw_4k_block(&mut self, block_id: u32) -> Result<[u8; 4096], Error> {
        let offset = block_id * self.superblock.get_block_size();
        debug!("seeking to offset {offset} for block id read {block_id}");
        self.disk.seek(SeekFrom::Start(offset as u64))?;
        let mut block: [u8; 4096] = [0; 4096];
        self.disk.read_exact_into(4096, &mut block.as_mut_slice())?;
        trace!("{block:?}");

        Ok(block)
    }

    pub fn read_raw_8k_block(&mut self, block_id: u32) -> Result<[u8; 8192], Error> {
        let offset = block_id * self.superblock.get_block_size();
        self.disk.seek(SeekFrom::Start(offset as u64))?;
        let mut block: [u8; 8192] = [0; 8192];
        self.disk.read_exact_into(8192, &mut block.as_mut_slice())?;
        Ok(block)
    }

    pub fn read_4k_bitmap_at(&mut self, block_id: u32) -> Result<Bitmap<4096>, Error> {
        debug!("Reading 4k bitmap table at: {block_id}");
        Ok(Bitmap {
            block: self.read_raw_4k_block(block_id)?,
        })
    }

    pub fn write_raw_4k_block(&mut self, block_id: u32, blk: &[u8; 4096]) -> Result<(), Error> {
        let offset = block_id * self.superblock.get_block_size();
        debug!("seeking to offset {offset} for block id write {block_id}");
        self.disk.seek(SeekFrom::Start(offset as u64))?;
        self.disk.write_all_bytes(blk)?;
        Ok(())
    }
}
pub struct BlockLock {
    locked_id: u32,
    locks: Rc<RefCell<BTreeSet<u32>>>,
}
impl Drop for BlockLock {
    fn drop(&mut self) {
        if let Ok(mut locks) = self.locks.try_borrow_mut() {
            locks.remove(&self.locked_id);
        }
    }
}
