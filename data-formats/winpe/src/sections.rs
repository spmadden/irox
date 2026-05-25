// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

mod coff_header;
mod msdosstub;
mod optional_header;

pub use coff_header::*;
use core::fmt::{Debug, Formatter};
use irox_structs::Struct;
pub use msdosstub::*;
pub use optional_header::*;

#[derive(Clone, Struct)]
#[little_endian]
pub struct SectionHeader {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

impl Debug for SectionHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SectionHeader")
            .field("name", &str::from_utf8(&self.name).unwrap_or_default())
            .field("virtual_size", &self.virtual_size)
            .field("virtual_address", &self.virtual_address)
            .field("size_of_raw_data", &self.size_of_raw_data)
            .field("pointer_to_raw_data", &self.pointer_to_raw_data)
            .field("pointer_to_relocations", &self.pointer_to_relocations)
            .field("pointer_to_linenumbers", &self.pointer_to_linenumbers)
            .field("number_of_relocations", &self.number_of_relocations)
            .field("number_of_linenumbers", &self.number_of_linenumbers)
            .field("characteristics", &self.characteristics)
            .finish()
    }
}
