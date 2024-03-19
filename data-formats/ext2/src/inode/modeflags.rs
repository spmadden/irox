// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

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
