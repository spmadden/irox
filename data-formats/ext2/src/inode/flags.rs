// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Inode flags

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
