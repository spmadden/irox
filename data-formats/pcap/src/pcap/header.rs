// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_structs::Struct;

#[derive(Debug, Copy, Clone, Struct)]
pub struct Header {
    pub magic: u32,
    pub major_ver: u16,
    pub minor_ver: u16,
    pub reserved_1: u32,
    pub reserved_2: u32,
    pub snap_len: u32,
    pub link_type: u32,
}
