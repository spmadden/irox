// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use irox_structs::Struct;

#[derive(Struct, Copy, Clone,  Debug, PartialEq, Eq)]
#[little_endian]
pub struct Superblock {
    pub s_inodes_count: u32,
    pub s_blocks_count: u32,
    pub s_r_blocks_count: u32,
    pub s_free_blocks_count: u32,
    pub s_free_inodes_count: u32,
    pub s_first_data_block: u32,
    pub s_log_block_size: u32,
    pub s_log_frag_size: i32,
    pub s_blocks_per_group: u32,
    pub s_frags_per_group: u32,
    pub s_inodes_per_group: u32,
    pub s_mtime: u32,
    pub s_wtime: u32,
    pub s_mnt_count: u16,
    pub s_max_mnt_count: u16,
    pub s_magic: u16,
    pub s_state: u16,
    pub s_errors: u16,
    pub s_minor_rev_level: u16,
    pub s_lastcheck: u32,
    pub s_checkinterval: u32,
    pub s_creator_os: u32,
    pub s_rev_level: u32,
    pub s_def_resuid: u16,
    pub s_def_resgid: u16,
    pub s_first_ino: u32,
    pub s_inode_size: u16,
    pub s_block_group_nr: u16,
    pub s_feature_compat: u32,
    pub s_feature_incompat: u32,
    pub s_feature_ro_compat: u32,
    pub s_uuid: [u8;16],

    pub s_volume_name: [u8;16],

    pub s_last_mounted: [u8; 64],
    pub s_algo_bitmap: u32,
    pub s_prealloc_blocks: u8,
    pub s_prealloc_dir_blocks: u8,
    pub _pad1: u16,

    pub s_journal_uuid: [u8;16],
    pub s_journal_inum: u32,
    pub s_journal_dev: u32,
    pub s_last_orphan: u32,

    pub s_hash_seed: [u32;4],
    pub s_def_hash_version: u8,
    pub _pad2: [u8;3],

    pub s_default_mount_options: u32,
    pub s_first_meta_bg: u32,
    pub _pad3: [u8;760]
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
    use std::io::{Seek, SeekFrom};
    use irox_tools::bits::Error;
    use irox_structs::Struct;
    use crate::superblock::Superblock;

    #[test]
    pub fn test1() ->Result<(), Error> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("doc/test.ext2")?;
        file.seek(SeekFrom::Current(0x400))?;
        let block = Superblock::parse_from(&mut file)?;
        println!("{block:#?}");
        Ok(())
    }
}