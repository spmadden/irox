// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

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
