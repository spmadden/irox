// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Superblock structure

pub mod features;
pub mod flags;

use irox_structs::Struct;

#[derive(Struct, Copy, Clone, Debug, PartialEq, Eq)]
#[little_endian]
pub struct Superblock {
    /// Total # of inodes in the system incl used and free
    pub s_inodes_count: u32,
    /// Total # of blocks in the system incl used, free and resvd
    pub s_blocks_count: u32,
    /// Total # of reserved blocks in the system by the superuser
    pub s_r_blocks_count: u32,
    /// Total # of free blocks in the system
    pub s_free_blocks_count: u32,
    /// Total # of free inodes in the system
    pub s_free_inodes_count: u32,
    /// Block ID of the main superblock structure
    pub s_first_data_block: u32,
    /// Bit-shift factor to get the main block size.
    /// `block size = 1024 << s_log_block_size`
    pub s_log_block_size: u32,
    /// Bit-shift factor to get the main fragmnet size
    /// ```plain
    /// fragment size = 1024 << s_log_frag_size // if positive
    /// fragment size = 1024 >> s_log_frag_size // if negative
    /// ```
    pub s_log_frag_size: i32,
    /// Total number of blocks per block group
    pub s_blocks_per_group: u32,
    /// Total number of fragments per block group
    pub s_frags_per_group: u32,
    /// Total number of inodes per block group
    pub s_inodes_per_group: u32,
    /// Last filesystem mount time, unix timestamp (seconds)
    pub s_mtime: u32,
    /// Last filesystem write time, uinx timestamp (seconds)
    pub s_wtime: u32,
    /// Number of filesystem mounts since it was last fully verified.
    pub s_mnt_count: u16,
    /// Number of filesystem mounts before a full check is required
    pub s_max_mnt_count: u16,
    /// Filesystem Magic, always `0xEF53(le)`
    pub s_magic: u16,
    /// Filesystem State Flags, see the `STATE` variants in [`superblock::flags`]
    pub s_state: u16,
    /// Filesystem error instructions, see the `ERROR` variants in [`superblock::flags`]
    pub s_errors: u16,
    /// Minor revision level
    pub s_minor_rev_level: u16,
    /// Last filesystem check time, unix timestamp (seconds)
    pub s_lastcheck: u32,
    /// Filesystem check interval, unix timestamp (seconds)
    pub s_checkinterval: u32,
    /// Identifier of the OS that created the filesystem, see the `OS` variants in [`superblock::flags`]
    pub s_creator_os: u32,
    /// Revision level.  0 = Rev0, 1 = Rev1 w/ variable inode sizes, ext attrs, etc
    pub s_rev_level: u32,
    /// Default user ID for reserved blocks, usually 0
    pub s_def_resuid: u16,
    /// Default group ID for reserved blocks, usually 0
    pub s_def_resgid: u16,
    /// Index to the first inode usable for standard files, usually `11`
    pub s_first_ino: u32,
    /// Size of the inode structure, usually `128` but always a power of 2
    pub s_inode_size: u16,
    /// Superblocks are copied to each block group header, this superblock was read
    /// from this specific block group header
    pub s_block_group_nr: u16,
    /// Bitmask of compatible features
    pub s_feature_compat: u32,
    /// Bitmask of incompatible features
    pub s_feature_incompat: u32,
    /// Bitmask of compatible features for a RO filesystem
    pub s_feature_ro_compat: u32,
    /// UUID of this filesystem
    pub s_uuid: [u8; 16],

    /// Volume name of this filesystem
    pub s_volume_name: [u8; 16],

    /// Directory path where this filesystem was last mounted, should be zero terminated
    pub s_last_mounted: [u8; 64],
    /// Specific compression algorithm used
    pub s_algo_bitmap: u32,
    /// Number of blocks the implementation should attempt to pre-allocate when creating
    /// a new file
    pub s_prealloc_blocks: u8,
    /// Numbre of blocks the implementatoin should attempt to pre-allocate when creating
    /// a new directory
    pub s_prealloc_dir_blocks: u8,

    pub _pad1: u16,

    /// UUID of the journal superblock
    pub s_journal_uuid: [u8; 16],
    /// Inode number of the journal file
    pub s_journal_inum: u32,
    /// Device number of the journal file
    pub s_journal_dev: u32,
    /// Inode number pointing to the first inode in the list of inodes to delete
    pub s_last_orphan: u32,

    /// Seeds for the hash algorithm for directory indexing
    pub s_hash_seed: [u32; 4],
    /// Default hash version used for directory indexing
    pub s_def_hash_version: u8,
    pub _pad2: [u8; 3],

    /// Default mount options for this filesystem
    pub s_default_mount_options: u32,
    /// Block Group IDof the first meta block group.
    pub s_first_meta_bg: u32,
    pub _pad3: [u8; 760],
}

impl Superblock {
    #[must_use]
    pub const fn get_block_size(&self) -> u32 {
        1024 << self.s_log_block_size
    }

    #[must_use]
    pub const fn get_fragment_size(&self) -> u32 {
        if self.s_log_frag_size > 0 {
            1024 << self.s_log_frag_size
        } else {
            1024 >> -self.s_log_frag_size
        }
    }

    #[must_use]
    pub const fn get_num_block_groups(&self) -> u32 {
        let rem = if self.s_blocks_count % self.s_blocks_per_group > 0 {
            1
        } else {
            0
        };

        self.s_blocks_count / self.s_blocks_per_group + rem
    }
}

#[cfg(test)]
mod tests {
    use crate::superblock::Superblock;
    use irox_structs::Struct;
    use irox_tools::bits::Error;
    use std::io::{Seek, SeekFrom};

    #[test]
    pub fn test1() -> Result<(), Error> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("doc/test.256m.ext2")?;
        file.seek(SeekFrom::Current(0x400))?;
        let block = Superblock::parse_from(&mut file)?;
        println!("{block:#?}");
        Ok(())
    }
}
