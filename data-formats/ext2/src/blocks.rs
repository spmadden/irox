// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::io::Seek;

use irox_structs::Struct;
use irox_tools::bits::{Bits, Error, ErrorKind};

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
    pub _reserved: [u8;12]

}

pub struct BlockGroupDescriptor<T> {
    pub raw_bgd: RawBlockGroupDescriptor,
    pub fs: Filesystem<T>,
}
impl<T> Clone for BlockGroupDescriptor<T> {
    fn clone(&self) -> Self {
        Self {
            raw_bgd: self.raw_bgd,
            fs: self.fs.clone()
        }
    }
}

impl<T:Bits+Seek> BlockGroupDescriptor<T>{
    pub fn read_4k_block_bitmap(&self) -> Result<Bitmap<4096>, Error> {
        let block_id = self.raw_bgd.bg_block_bitmap;
        let Ok(mut rd) = self.fs.lock() else {
            return Err(ErrorKind::BrokenPipe.into());
        };
        rd.read_4k_bitmap_at(block_id)
    }

    pub fn read_4k_inode_bitmap(&self) -> Result<Bitmap<4096>, Error> {
        let block_id = self.raw_bgd.bg_inode_bitmap;
        let Ok(mut rd) = self.fs.lock() else {
            return Err(ErrorKind::BrokenPipe.into());
        };
        rd.read_4k_bitmap_at(block_id)
    }

    pub fn is_4k_inode_valid(&self, local_id: u32) -> Result<bool, Error> {
        self.read_4k_inode_bitmap()?.is_element_used(local_id)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Bitmap<const N:usize> {
    pub block: [u8;N],
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
}
