// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//


use irox_tools::bits::{Bits};

use crate::data::DataStream;

#[derive(Debug, PartialEq, Eq)]
pub struct DirEnt {
    pub inode: u32,
    pub rec_len: u16,
    pub name_len: u8,
    pub file_type: u8,
    pub name: String,
}

impl DirEnt {
    pub fn parse_from<T: irox_tools::bits::Bits>(
        input: &mut T,
    ) -> Result<Self, irox_tools::bits::Error> {
        let inode = input.read_le_u32()?;
        let rec_len = input.read_le_u16()?;
        let name_len = input.read_u8()?;
        let file_type = input.read_u8()?;
        let name = input.read_str_sized_lossy(name_len as usize)?;
        Ok(DirEnt {
            inode,
            rec_len,
            name_len,
            file_type,
            name,
        })
    }

    pub fn write_to<T: irox_tools::bits::MutBits>(
        &self,
        out: &mut T,
    ) -> Result<(), irox_tools::bits::Error> {
        out.write_le_u32(self.inode)?;
        out.write_le_u16(self.rec_len)?;
        out.write_u8(self.name_len)?;
        out.write_u8(self.file_type)?;
        out.write_all_bytes(self.name.as_bytes())?;
        Ok(())
    }
}

pub struct DirectoryStream {
    data_stream: DataStream,
    done: bool,
}

impl DirectoryStream {
    pub fn new(data_stream: DataStream) -> Self {
        DirectoryStream {
            data_stream,
            done: false,
        }
    }
}

impl Iterator for DirectoryStream {
    type Item = DirEnt;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let ent = DirEnt::parse_from(&mut self.data_stream).ok()?;
        let skip = ent
            .rec_len
            .saturating_sub(8)
            .saturating_sub(ent.name_len as u16);
        if ent.inode == 0 {
            self.done = true;
            return None;
        }
        self.data_stream.advance(skip as usize).ok()?;
        Some(ent)
    }
}
