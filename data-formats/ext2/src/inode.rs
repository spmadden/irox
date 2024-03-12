// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::collections::VecDeque;
use std::io::Seek;
use std::path::Iter;
use std::sync::Mutex;

use irox_structs::Struct;
use irox_tools::bits::{Bits, Error, ErrorKind};

use crate::Filesystem;

pub mod special_ids {
    /// Bad Blocks Inode ID
    pub const EXT2_BAD_INO: u32 = 1;
    /// Root directory Inode ID
    pub const EXT2_ROOT_INO: u32 = 2;
    /// ACL Index Inode ID (deprecated)
    #[deprecated]
    pub const EXT2_ACL_IDX_INO: u32 = 3;
    /// ACL Data Inode ID (deprecated)
    #[deprecated]
    pub const EXT2_ACL_DATA_INO: u32 = 4;
    /// Boot Loader Inode ID
    pub const EXT2_BOOT_LOADER_INO: u32 = 5;
    /// Undelete directory Inode ID
    pub const EXT2_UNDEL_DIR_INO: u32 = 6;
}

pub mod modeflags {
    /// Socket
    pub const EXT2_S_IFSOCK: u16 = 0xC000;
    /// Symbolic Link
    pub const EXT2_S_IFLNK: u16 = 0xA000;
    /// Regular File
    pub const EXT2_S_IFREG: u16 = 0x8000;
    /// Block Device
    pub const EXT2_S_IFBLK: u16 = 0x6000;
    /// Directory
    pub const EXT2_S_IFDIR: u16 = 0x4000;
    /// Character Device
    pub const EXT2_S_IFCHR: u16 = 0x2000;
    /// FIFO Queue
    pub const EXT2_S_IFIFO: u16 = 0x1000;

    /// Set process UID bit
    pub const EXT2_S_ISUID: u16 = 0x0800;
    /// Set process GID bit
    pub const EXT2_S_ISGID: u16 = 0x0400;
    /// Sticky bit
    pub const EXT2_S_ISVTX: u16 = 0x0200;

    /// User Read
    pub const EXT2_S_IRUSR: u16 = 0x0100;
    /// User Write
    pub const EXT2_S_IWUSR: u16 = 0x0080;
    /// User Execute
    pub const EXT2_S_IXUSR: u16 = 0x0040;
    /// Group Read
    pub const EXT2_S_IRGRP: u16 = 0x0020;
    /// Group Write
    pub const EXT2_S_IWGRP: u16 = 0x0010;
    /// Group Execute
    pub const EXT2_S_IXGRP: u16 = 0x0008;
    /// Other Read
    pub const EXT2_S_IROTH: u16 = 0x0004;
    /// Other WRite
    pub const EXT2_S_IWOTH: u16 = 0x0002;
    /// Other Execute
    pub const EXT2_S_IXOTH: u16 = 0x0001;
}

pub mod flags {
    /// Secure Deletion
    pub const EXT2_SECRM_FL: u32 = 0x0000_0001;
    /// Record for undelete
    pub const EXT2_UNRM_FL: u32 = 0x0000_0002;
    /// Compressed file
    pub const EXT2_COMPR_FL: u32 = 0x0000_0004;
    /// synchronous updates
    pub const EXT2_SYNC_FL: u32 = 0x0000_0008;
    /// immutable file
    pub const EXT2_IMMUTABLE_FL: u32 = 0x0000_0010;
    /// append only
    pub const EXT2_APPEND_FL: u32 = 0x0000_0020;
    /// do not dump/delete file
    pub const EXT2_NODUMP_FL: u32 = 0x0000_0040;
    /// do nut update `.i_atime`
    pub const EXT2_NOATIME_FL: u32 = 0x0000_0080;

    /// Dirty (modified)
    pub const EXT2_DIRTY_FL: u32 = 0x0000_0100;
    /// compressed blocks
    pub const EXT2_COMPRBLK_FL: u32 = 0x0000_0200;
    /// access raw compressed data
    pub const EXT2_NOCOMPR_FL: u32 = 0x0000_0400;
    /// compression error
    pub const EXT2_ECOMPR_FL: u32 = 0x0000_0800;

    /// b-tree format directory
    pub const EXT2_BTREE_FL: u32 = 0x0000_1000;
    /// hash indexed directory
    pub const EXT2_INDEX_FL: u32 = 0x0000_1000;
    /// AFS directory
    pub const EXT2_IMAGIC_FL: u32 = 0x0000_2000;
    /// journal file data
    pub const EXT2_JOURNAL_DATA_FL: u32 = 0x0000_4000;
    /// reserved for ext2 library
    pub const EXT2_RESERVED_FL: u32 = 0x8000_0000;
}

#[derive(Debug, Eq, PartialEq)]
pub enum FileType {
    FIFO,
    CharDev,
    Directory,
    BlockDev,
    RegularFile,
    SymbolicLink,
    Socket
}

impl TryFrom<u16> for FileType {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value & modeflags::EXT2_S_IFSOCK > 0 {
            Ok(FileType::Socket)
        } else if value & modeflags::EXT2_S_IFLNK > 0 {
            Ok(FileType::SymbolicLink)
        } else if value & modeflags::EXT2_S_IFREG > 0 {
            Ok(FileType::RegularFile)
        } else if value & modeflags::EXT2_S_IFBLK > 0 {
            Ok(FileType::BlockDev)
        } else if value & modeflags::EXT2_S_IFDIR > 0 {
            Ok(FileType::Directory)
        } else if value & modeflags::EXT2_S_IFCHR > 0 {
            Ok(FileType::CharDev)
        } else if value & modeflags::EXT2_S_IFIFO > 0 {
            Ok(FileType::FIFO)
        } else {
            Err(ErrorKind::InvalidData.into())
        }
    }
}

///
/// "Index Node" - each object in the filesystem is represented by an inode.  The inode contains all
/// the metadata for the file except for the name.
#[derive(Struct, Debug, PartialEq, Eq)]
#[little_endian]
pub struct RawInode {
    /// Mode flags, see [`modeflags`] for values
    pub i_mode: u16,
    /// owner user ID
    pub i_uid: u16,
    /// lower 32 bits of the size/length of the referenced element
    pub i_size: u32,
    /// Access time, unix timestamp seconds
    pub i_atime: u32,
    /// Creation time, unix timestamp seconds
    pub i_ctime: u32,
    /// Modification time, unix timestamp seconds
    pub i_mtime: u32,
    /// Deletion time, unix timestamp seconds
    pub i_dtime: u32,
    /// owning group id
    pub i_gid: u16,
    /// number of hard links referencing this inode
    pub i_links_count: u16,
    /// total number of 512-byte blocks reserved for this inode's data.  Specific blocks are
    /// indicated in [`Inode.i_blocks`]
    pub i_blocks: u32,
    /// Filesystem behavior flags, see [`flags`] for values
    pub i_flags: u32,
    /// OS-dependent value
    pub i_osd1: u32,

    ///
    /// List of referenced/owned blocks.
    ///
    /// * Index 0 -> 11: Direct block identifiers
    /// * Index 12: Indirect block, a full block with a series of Direct block identifiers
    /// * Index 13: Doubly-Indirect block, a full block with a series of Indirect block identifiers
    /// * Index 14: Triply-Indirect block, a full block with a series of Doubly-Indirect block identifiers
    ///
    /// This scheme gives a maximum data length of:
    /// * 1k block size: 17.3 GB
    /// * 2k block size: 275.4 GB
    /// * 4k block size: 4.4 TB
    /// * 8k block size: 70.4 TB
    pub i_block: [u32; 15],

    /// file version
    pub i_generation: u32,
    /// block number containing extended attributes
    pub i_file_acl: u32,

    /// * If a directory: Block number containing extended attributes
    /// * Otherwise: upper 32 bits of the data length
    pub i_dir_acl: u32,
    /// File fragment block number
    pub i_faddr: u32,
    /// OS-Dependent data
    pub i_osd2: [u8; 12],
}
pub struct Inode<T> {
    pub raw_inode: RawInode,
    pub fs: Filesystem<T>
}
impl<T: Bits+Seek> Inode<T> {

    pub fn get_num_reserved_blocks(&self) -> Result<u32, Error> {
        let Ok(inner) = self.fs.lock() else {
            return Err(ErrorKind::BrokenPipe.into());
        };
        let num = self.raw_inode.i_blocks / (2 << inner.superblock.s_log_block_size);
        Ok(num)
    }
    pub fn get_data_length(&self) -> Result<u64, Error> {
        let mut len: u64 = self.raw_inode.i_size as u64;
        if let FileType::RegularFile = FileType::try_from(self.raw_inode.i_mode)? {
            len |= (self.raw_inode.i_dir_acl as u64) << 32;
        };
        Ok(len)
    }

    pub fn data_stream<'a>(&'a self) -> Result<DataStream<'a, T>, Error> {
        todo!()
    }
}

pub struct DataStream<'a, T> {
    inode: &'a Inode<T>,
    position: u64,
    length: u64,
    block_iter: BlockIter<'a, T>,
}
impl<'a, T: Bits+Seek> DataStream<'a, T> {
    pub fn new(inode: &'a Inode<T>) -> Result<Self, Error> {
        let total_blocks = inode.get_num_reserved_blocks()?;
        let length = inode.get_data_length()?;
        let block_iter = BlockIter::new(inode)?;
        let ds = DataStream {
            inode,
            position: 0,
            length,
            block_iter
        };
        Ok(ds)
    }
}
impl<'a, T: Bits+Seek> Iterator for DataStream<'a, T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.length {
            return None;
        }
        
        todo!()
    }
}


pub struct BlockIter<'a, T> {
    inode: &'a Inode<T>,
    loaded_block_count: u32,
    total_block_count: u32,
    next_blocks: VecDeque<u32>,
    single_indirects: VecDeque<u32>,
    double_indirects: VecDeque<u32>,
}
impl<'a, T: Bits+Seek> BlockIter<'a, T> {
    pub fn new(inode: &'a Inode<T>) -> Result<Self, Error> {
        let total_blocks = inode.get_num_reserved_blocks()?;
        let mut out = BlockIter {
            inode,
            loaded_block_count: 0,
            total_block_count: 0,
            next_blocks: VecDeque::new(),
            single_indirects: VecDeque::new(),
            double_indirects: VecDeque::new(),
        };
        for (idx, blk_id) in inode.raw_inode.i_block.iter().enumerate() {
            let idx = idx as u32;
            if idx >= total_blocks {
                break;
            }
            if idx < 12 {
                
            }
        }
        todo!()
    }
}

impl<'a, T: Bits+Seek> Iterator for BlockIter<'a, T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next_blocks.pop_front() {
            return Some(next);
        };
        
        if self.loaded_block_count >= self.total_block_count {
            return None;
        }
        
        let sb = self.inode.fs.get_superblock().ok()?;
        let block_size = sb.get_block_size();
        let num_block_ids_per_indirect = block_size >> 2;


        todo!()
    }
}
