// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_structs::Struct;

#[derive(Debug, Struct)]
pub struct SectionHeaderBlock {
    pub byte_order_magic: u32,
    pub major_ver: u16,
    pub minor_ver: u16,
    pub section_length: i64,
    // options parsing?
}
