// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Superblock flags

/// Filesystem was unmounted cleanly
pub const EXT2_STATE_VALID_FS: u16 = 0x01;
/// Filesystem has errors detected
pub const EXT2_STATE_ERROR_FS: u16 = 0x02;

/// Continue as if nothing happened on errors
pub const EXT2_ERRORS_CONTINUE: u16 = 0x01;
/// Remount read-only on errors
pub const EXT2_ERRORS_RO: u16 = 0x02;
/// Cause a kernel-panic on errors
pub const EXT2_ERRORS_PANIC: u16 = 0x03;

/// Linux Operating System
pub const EXT2_OS_LINUX: u32 = 0x00;
pub const EXT2_OS_HURD: u32 = 0x01;
pub const EXT2_OS_MASIX: u32 = 0x02;
pub const EXT2_OS_FREEBSD: u32 = 0x03;
pub const EXT2_OS_LITES: u32 = 0x04;
pub const EXT2_OS_IROX: u32 = 0x49524F58; // 'IROX'
