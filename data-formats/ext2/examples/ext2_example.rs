// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use irox_ext2::directory::DirectoryStream;
use irox_ext2::inode::special_ids;
use irox_ext2::{inode, Filesystem};
use irox_log::log::Level;
use irox_tools::bits::Error;
use std::fs::OpenOptions;

fn main() -> Result<(), Error> {
    irox_log::init_console_level(Level::Debug);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("data-formats/ext2/doc/test.256m.ext2")?;
    let fs = Filesystem::open(file)?;
    let bgt = fs.read_bg_descriptor_table_at(1)?;
    // println!("{:#?}", fs.get_superblock()?);

    let inode = fs.find_inode(special_ids::EXT2_ROOT_INO)?;
    println!("Root Inode data: {:#?}", inode.raw_inode);

    for blk_id in inode.block_stream()? {
        println!("Root inode block id: {blk_id}")
    }

    let ds = inode.data_stream()?;
    let dirstr = DirectoryStream::new(ds);
    for dir in dirstr {
        println!("{dir:#?}");
    }

    Ok(())
}
