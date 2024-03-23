// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::fs::OpenOptions;
use std::io::ErrorKind;

use irox_ext2::{Filesystem, inode};
use irox_ext2::directory::DirectoryStream;
use irox_ext2::inode::special_ids;
use irox_log::log::Level;
use irox_tools::bits::Error;

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
    let Some(dir_inode) = inode.as_directory() else {
        return Err(ErrorKind::InvalidData.into());
    };
    for blk_id in dir_inode.block_stream()? {
        println!("Root inode block id: {blk_id}")
    }

    let ds = dir_inode.data_stream()?;
    let dirstr = DirectoryStream::new(ds);
    for dir in dirstr {
        println!("{dir:#?}");
    }

    Ok(())
}
