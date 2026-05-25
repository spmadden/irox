// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use irox_structs::Struct;

#[derive(Debug, Clone, Struct)]
#[little_endian]
pub struct COFFHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub date_time_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}
