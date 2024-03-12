// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]

use std::io::{Seek, SeekFrom};
use std::sync::{Arc, Mutex, MutexGuard};

use blocks::*;
use directory::*;
use inode::*;
use irox_log::log::{debug, trace};
use irox_structs::Struct;
pub use irox_tools::bits::{Bits, Error, MutBits};
pub use irox_tools::bits::ErrorKind;
use superblock::*;

pub mod blocks;
pub mod directory;
pub mod inode;
pub mod superblock;

pub struct Filesystem<T> {
    inner: Arc<Mutex<FSInner<T>>>,
}

impl<T> Clone for Filesystem<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

impl<T: Bits + Seek> Filesystem<T> {
    pub fn open(stream: T) -> Result<Filesystem<T>, Error> {
        Ok(Self {
            inner: Arc::new(Mutex::new(FSInner::open_ro(stream)?))
        })
    }
    pub fn read_bg_descriptor_table_at(&self, mut block_id: u32) -> Result<Vec<BlockGroupDescriptor<T>>, Error> {
        let Ok(mut inner) = self.inner.lock() else {
            return Err(ErrorKind::BrokenPipe.into());
        };

        let num_groups = inner.superblock.get_num_block_groups();
        debug!("Reading {num_groups} BGT entries at block {block_id}");
        let mut out = Vec::new();
        let bgts_per_block = inner.superblock.get_block_size() >> 5;

        let mut block = inner.read_raw_4k_block(block_id)?;
        let mut block_ref = block.as_mut_slice();
        for i in 0..num_groups {
            if i > 0 && i & 0x1F == 0 {
                block_id += 1;
                block = inner.read_raw_4k_block(block_id)?;
                block_ref = block.as_mut_slice();
            }
            let bgt = RawBlockGroupDescriptor::parse_from(&mut block_ref)?;
            debug!("Read BGT {i}: {bgt:?}");
            out.push(BlockGroupDescriptor {
                raw_bgd: bgt,
                fs: self.clone(),
            });
        }

        Ok(out)
    }

    pub fn get_superblock(&self) -> Result<Superblock, Error> {
        let Ok(inner) = self.inner.lock() else {
            return Err(ErrorKind::BrokenPipe.into());
        };
        Ok(inner.superblock)
    }

    pub fn get_cached_bg_table(&self) -> Result<Arc<Vec<BlockGroupDescriptor<T>>>, Error> {
        let bgt = {
            self.lock()?.block_group_table.clone()
        };
        Ok(match bgt {
            Some(bgt) => bgt,
            None => {
                let bgt = Arc::new(self.read_bg_descriptor_table_at(1)?);
                self.lock()?.block_group_table = Some(bgt.clone());
                bgt
            }
        }
        )
    }

    pub fn find_inode(&self, inode_idx: u32) -> Result<Inode<T>, Error> {
        let sb = self.get_superblock()?;
        let bg_id = (inode_idx) / sb.s_inodes_per_group;
        let local_idx = (inode_idx) % sb.s_inodes_per_group;

        let bgt = self.get_cached_bg_table()?;
        let Some(bg) = bgt.get(bg_id as usize) else {
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

        let block = self.lock()?.read_raw_4k_block(inode_block_id)?;
        let mut subset = block.split_at(inode_seek_offset as usize).1;
        let inode = RawInode::parse_from(&mut subset)?;

        Ok(Inode {
            raw_inode: inode,
            fs: self.clone(),
        })
    }

    pub fn lock(&self) -> Result<MutexGuard<FSInner<T>>, Error> {
        self.inner.lock().map_err(|_| ErrorKind::BrokenPipe.into())
    }
}

pub struct FSInner<T> {
    disk: Box<T>,
    superblock: Superblock,
    block_group_table: Option<Arc<Vec<BlockGroupDescriptor<T>>>>,
}

impl<T> FSInner<T> {
    pub fn get_superblock(&self) -> Superblock {
        self.superblock
    }
}

impl<T: Bits + Seek> FSInner<T> {
    pub fn open_ro(mut stream: T) -> Result<FSInner<T>, Error> {
        debug!("Reading superblock at 0x400");
        stream.seek(SeekFrom::Start(0x400))?;
        let superblock = Superblock::parse_from(&mut stream)?;
        trace!("Found superblock: {superblock:#?}");
        Ok(Self {
            disk: Box::new(stream),
            superblock,
            block_group_table: None,
        })
    }

    pub fn read_raw_4k_block(&mut self, block_id: u32) -> Result<[u8; 4096], Error> {
        let offset = block_id * self.superblock.get_block_size();
        debug!("seeking to offset {offset} for block id {block_id}");
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
            block: self.read_raw_4k_block(block_id)?
        })
    }
}

impl<T: Bits + MutBits + Seek> FSInner<T> {
    pub fn open_rw(mut stream: T) -> Result<FSInner<T>, Error> {
        stream.seek(SeekFrom::Start(0x400))?;
        let superblock = Superblock::parse_from(&mut stream)?;
        Ok(Self {
            disk: Box::new(stream),
            superblock,
            block_group_table: None,
        })
    }
}