// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use irox_structs::Struct;
use irox_tools::bits::{Error, ErrorKind};

use crate::Filesystem;

#[derive(Struct, Copy, Clone, Debug, PartialEq, Eq)]
#[little_endian]
pub struct RawBlockGroupDescriptor {
    pub bg_block_bitmap: u32,
    pub bg_inode_bitmap: u32,
    pub bg_inode_table: u32,
    pub bg_free_blocks_count: u16,
    pub bg_free_inodes_count: u16,
    pub bg_used_dirs_count: u16,
    pub _pad: u16,
    pub _reserved: [u8; 12],
}

pub struct BlockGroupDescriptor {
    pub raw_bgd: RawBlockGroupDescriptor,
    pub bgt_id: u32,
    pub fs: Filesystem,
    pub block_bitmap: Bitmap<4096>,
    pub inode_bitmap: Bitmap<4096>,
    pub dirty: bool
}

impl BlockGroupDescriptor {
    pub fn new(
        raw_bgd: RawBlockGroupDescriptor,
        bgt_id: u32,
        fs: Filesystem,
    ) -> Result<Self, Error> {
        let block_bitmap = fs.read_4k_bitmap_at(raw_bgd.bg_block_bitmap)?;
        let inode_bitmap = fs.read_4k_bitmap_at(raw_bgd.bg_inode_bitmap)?;
        Ok(Self {
            raw_bgd,
            bgt_id,
            fs,
            block_bitmap, 
            inode_bitmap,
            dirty: false,
        })
    }

    pub fn is_4k_inode_valid(&self, local_id: u32) -> Result<bool, Error> {
        self.inode_bitmap.is_element_used(local_id)
    }

    pub fn try_write_4k_block(&mut self, block: &[u8; 4096]) -> Result<u32, BGDError> {
        let Some(block_id) = self.block_bitmap.take_next_unused_element() else {
            return Err(BGDError::BGFull);
        };

        self.fs.write_raw_4k_block(block_id, block)?;
        self.raw_bgd.bg_free_blocks_count = self.raw_bgd.bg_free_blocks_count.saturating_sub(1);
        self.dirty = true;

        Ok(block_id)
    }
}

pub enum BGDError {
    BGFull,
    IOError(Error),
}

impl From<Error> for BGDError {
    fn from(value: Error) -> Self {
        Self::IOError(value)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Bitmap<const N: usize> {
    pub block: [u8; N],
}
impl<const N: usize> Bitmap<N> {
    pub fn is_element_used(&self, element_id: u32) -> Result<bool, Error> {
        let Some(element) = self.block.get(element_id as usize >> 3) else {
            return Err(ErrorKind::UnexpectedEof.into());
        };
        let bit = element >> (element_id & 0x7);
        Ok((bit & 0x01) > 0)
    }

    pub fn get_used_elements(&self) -> Vec<u32> {
        let mut out: Vec<u32> = Vec::new();
        for (idx, elem) in self.block.iter().enumerate() {
            if *elem == 0x0 {
                // it's empty, no need to check every bit.
                continue;
            }
            let idx = (idx as u32) << 3;
            for off in 0..8 {
                let bit = (elem >> off) & 0x01;
                if bit > 0 {
                    out.push(idx | off);
                }
            }
        }
        out
    }

    pub fn next_unused_element(&self) -> Option<u32> {
        for (idx, elem) in self.block.iter().enumerate() {
            let idx = (idx as u32) << 3;
            if *elem == 0xFF {
                // it's full, no need to check every individual bit
                continue;
            }
            for off in 0..8 {
                let bit = (elem >> off) & 0x01;
                if bit == 0 {
                    let val = idx | off;
                    return Some(val);
                }
            }
        }
        None
    }

    pub fn mark_element_used(&mut self, id: u32) -> Result<(), Error> {
        let max = N as u32 * 8;
        if id >= max {
            return Err(ErrorKind::AddrNotAvailable.into());
        }
        let idx = id >> 3;
        if let Some(val) = self.block.get_mut(idx as usize) {
            let elem = 1 << (id & 0x3);
            *val |= elem;
        } else {
            return Err(ErrorKind::UnexpectedEof.into());
        }
        Ok(())
    }

    pub fn take_next_unused_element(&mut self) -> Option<u32> {
        for (idx, elem) in self.block.iter_mut().enumerate() {
            let idx = (idx as u32) << 3;
            if *elem == 0xFF {
                // it's full, no need to check every individual bit
                continue;
            }
            for off in 0..8 {
                let bit = (*elem >> off) & 0x01;
                if bit == 0 {
                    let val = idx | off;
                    *elem |= 1 << off;
                    return Some(val);
                }
            }
        }
        None
    }
}
