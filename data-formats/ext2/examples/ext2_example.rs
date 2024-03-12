// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::fs::OpenOptions;
use irox_ext2::{Filesystem, inode};
use irox_log::log::Level;
use irox_tools::bits::Error;

fn main() -> Result<(), Error> {
    irox_log::init_console_level(Level::Debug);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("data-formats/ext2/doc/test.256m.ext2")?;
    let mut fs = Filesystem::open(file)?;
    let mut bgt = fs.read_bg_descriptor_table_at(1)?;

    for bgd in &mut bgt {
        let blk_bitmap = bgd.read_4k_block_bitmap()?;
        println!("Used blocks: {:?}", blk_bitmap.get_used_elements());
        let inode_bitmap = bgd.read_4k_inode_bitmap()?;
        println!("Used inodes: {:?}", inode_bitmap.get_used_elements());
    }

    let inode = fs.find_inode(inode::special_ids::EXT2_ROOT_INO)?;
    println!("Root Inode data: {:#?}", inode.raw_inode);


    Ok(())
}
