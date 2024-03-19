// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//! Superblock feature flags

/// Block pre-allocation for new directories
pub const EXT2_FEATURE_COMPAT_DIR_PREALLOC: u32 = 0x0001;
pub const EXT2_FEATURE_COMPAT_IMAGIC_INODES: u32 = 0x0002;
/// Ext3 journal exists
pub const EXT3_FEATURE_COMPAT_HAS_JOURNAL: u32 = 0x0004;
/// Extended inode attrs are present
pub const EXT2_FEATURE_COMPAT_EXT_ATTR: u32 = 0x0008;
/// Non-standard inode size used
pub const EXT2_FEATURE_COMPAT_RESIZE_INO: u32 = 0x0010;
/// Directory indexing (htree)
pub const EXT2_FEATURE_COMPAT_DIR_INDEX: u32 = 0x0020;

/// Disk/File compression is used
pub const EXT2_FEATURE_INCOMPAT_COMPRESSION: u32 = 0x0001;
pub const EXT2_FEATURE_INCOMPAT_FILETYPE: u32 = 0x0002;
pub const EXT2_FEATURE_INCOMPAT_RECOVER: u32 = 0x0004;
pub const EXT3_FEATURE_INCOMPAT_JOURNAL_DEV: u32 = 0x0008;
pub const EXT2_FEATURE_INCOMPAT_META_BG: u32 = 0x0010;

pub const EXT2_FEATURE_RO_COMPAT_SPARSE_SUPER: u32 = 0x0001;
pub const EXT2_FEATURE_RO_COMPAT_LARGE_FILE: u32 = 0x0002;
pub const EXT2_FEATURE_RO_COMPAT_BTREE_DIR: u32 = 0x0004;
