// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::collections::VecDeque;
use std::sync::Arc;

use irox_structs::Struct;
use irox_tools::bits::{Bits, Error, ErrorKind};
use crate::data::DataStream;

use crate::Filesystem;
use crate::typed_inode::{BlockDev, CharDev, Directory, Fifo, InodeType, RegFile, Socket, SymLink, TypedInode};

pub mod flags;
pub mod modeflags;
pub mod special_ids;

#[derive(Debug, Eq, PartialEq)]
pub enum FileType {
    FIFO,
    CharDev,
    Directory,
    BlockDev,
    RegularFile,
    SymbolicLink,
    Socket,
}

impl TryFrom<u16> for FileType {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value & modeflags::EXT2_S_IFSOCK == modeflags::EXT2_S_IFSOCK {
            Ok(FileType::Socket)
        } else if value & modeflags::EXT2_S_IFLNK == modeflags::EXT2_S_IFLNK {
            Ok(FileType::SymbolicLink)
        } else if value & modeflags::EXT2_S_IFREG == modeflags::EXT2_S_IFREG {
            Ok(FileType::RegularFile)
        } else if value & modeflags::EXT2_S_IFBLK == modeflags::EXT2_S_IFBLK {
            Ok(FileType::BlockDev)
        } else if value & modeflags::EXT2_S_IFDIR == modeflags::EXT2_S_IFDIR {
            Ok(FileType::Directory)
        } else if value & modeflags::EXT2_S_IFCHR == modeflags::EXT2_S_IFCHR {
            Ok(FileType::CharDev)
        } else if value & modeflags::EXT2_S_IFIFO == modeflags::EXT2_S_IFIFO {
            Ok(FileType::FIFO)
        } else {
            Err(ErrorKind::InvalidData.into())
        }
    }
}

///
/// "Index Node" - each object in the filesystem is represented by an inode.  The inode contains all
/// the metadata for the file except for the name.
#[derive(Struct, Debug, Copy, Clone, PartialEq, Eq)]
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
    /// Filesystem behavior flags, see [`flags::flags`] for values
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
pub struct Inode {
    pub raw_inode: RawInode,
    pub fs: Filesystem,
}
impl Inode {
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

    pub fn data_stream(self: &Arc<Self>) -> Result<DataStream, Error> {
        DataStream::new(self)
    }

    pub fn block_stream(self: &Arc<Self>) -> Result<BlockIter, Error> {
        BlockIter::new(self)
    }

    pub fn as_typed(self: Arc<Self>) -> Result<InodeType, Error> {
        let mode = self.raw_inode.i_mode;
        let mode: FileType = match mode.try_into() {
            Ok(e) => e,
            Err(_e) => return Err(ErrorKind::InvalidData.into()),
        };
        Ok(match mode {
            FileType::FIFO => InodeType::Fifo(TypedInode::<Fifo>::new(self)),
            FileType::CharDev => InodeType::CharDevice(TypedInode::<CharDev>::new(self)),
            FileType::Directory => InodeType::Directory(TypedInode::<Directory>::new(self)),
            FileType::BlockDev => InodeType::BlockDevice(TypedInode::<BlockDev>::new(self)),
            FileType::RegularFile => InodeType::RegularFile(TypedInode::<RegFile>::new(self)),
            FileType::SymbolicLink => InodeType::SymLink(TypedInode::<SymLink>::new(self)),
            FileType::Socket => InodeType::Socket(TypedInode::<Socket>::new(self)),
        })
    }
    pub fn as_directory(self: Arc<Self>) -> Option<Arc<TypedInode<Directory>>> {
        if let InodeType::Directory(dir) = self.as_typed().ok()? {
            return Some(Arc::new(dir));
        }
        None
    }
    pub fn as_regular_file(self: Arc<Self>) -> Option<Arc<TypedInode<RegFile>>> {
        if let InodeType::RegularFile(file) = self.as_typed().ok()? {
            return Some(Arc::new(file));
        }
        None
    }
}

///
/// Iterator that walks through the block IDs that an inode references for it's
/// data set.  Keep calling next to exhaust the sequence
pub struct BlockIter {
    inode: Arc<Inode>,
    loaded_block_count: u32,
    total_block_count: u32,
    next_blocks: VecDeque<u32>,
    single_indirects: VecDeque<u32>,
    double_indirects: VecDeque<u32>,
    triple_indirects: VecDeque<u32>,
}
impl BlockIter {
    pub fn new(inode: &Arc<Inode>) -> Result<Self, Error> {
        let total_blocks = inode.get_num_reserved_blocks()?;
        let mut out = BlockIter {
            inode: inode.clone(),
            loaded_block_count: 0,
            total_block_count: 0,
            next_blocks: VecDeque::new(),
            single_indirects: VecDeque::new(),
            double_indirects: VecDeque::new(),
            triple_indirects: VecDeque::new(),
        };
        for (idx, blk_id) in inode.raw_inode.i_block.iter().enumerate() {
            if *blk_id == 0 {
                break;
            }
            let idx = idx as u32;
            if idx >= total_blocks {
                break;
            }
            if idx < 12 {
                // direct block references
                out.next_blocks.push_back(*blk_id);
                out.loaded_block_count += 1;
            } else if idx == 12 {
                // single indirect references, each block ID is a block of direct references
                out.single_indirects.push_back(*blk_id);
            } else if idx == 13 {
                // double indirect references, each block ID is a block of single indirect references
                out.double_indirects.push_back(*blk_id);
            } else if idx == 14 {
                // triple indirect references, each block ID is a block of double indirect references
                out.triple_indirects.push_back(*blk_id);
            }
        }
        Ok(out)
    }

    /// Tries to fill the directblock list by iteratively dereferencing the indirect lists.
    /// If successful, returns how many elements were added.
    pub(crate) fn try_fill_directblocks(&mut self) -> Result<usize, Error> {
        if !self.next_blocks.is_empty() {
            return Ok(0);
        }
        if self.loaded_block_count >= self.total_block_count {
            return Ok(0);
        }
        let mut added = 0;
        while added == 0 {
            if let Some(single) = self.single_indirects.pop_front() {
                let blk = self.inode.fs.read_raw_4k_block(single)?;
                let mut blk_sli = blk.as_slice();
                // loop through the entire 4k block, each u32 is a block id
                for _ in 0..1024 {
                    let blkid = blk_sli.read_le_u32()?;
                    if blkid != 0 {
                        // a zero block ID isn't necessarily the end of the list,
                        // this could be a sparse file, so just skip it.
                        self.next_blocks.push_back(blkid);
                        added += 1;
                        self.loaded_block_count += 1;
                    }
                    if self.loaded_block_count >= self.total_block_count {
                        break;
                    }
                }
                if added > 0 {
                    break;
                }
            } else if let Some(double) = self.double_indirects.pop_front() {
                let blk = self.inode.fs.read_raw_4k_block(double)?;
                let mut blk_sli = blk.as_slice();
                for _ in 0..1024 {
                    let blkid = blk_sli.read_le_u32()?;
                    if blkid != 0 {
                        self.single_indirects.push_back(blkid);
                    }
                }
            } else if let Some(triple) = self.triple_indirects.pop_front() {
                let blk = self.inode.fs.read_raw_4k_block(triple)?;
                let mut blk_sli = blk.as_slice();
                for _ in 0..1024 {
                    let blkid = blk_sli.read_le_u32()?;
                    if blkid != 0 {
                        self.double_indirects.push_back(blkid);
                    }
                }
            }
        }
        Ok(added)
    }
}

impl Iterator for BlockIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next_blocks.pop_front() {
            return Some(next);
        };

        if self.loaded_block_count >= self.total_block_count {
            return None;
        }
        let added = self.try_fill_directblocks().ok()?;
        if added == 0 {
            return None;
        }
        self.next()
    }
}
