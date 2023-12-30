// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_structs::Struct;

pub const MAX_FRAME_LEN: usize = (1 << 24) - 1;

#[derive(Struct)]
pub struct H2Frame {
    pub len: u32,
    pub ty: u8,
    pub flags: u8,
    // reserved is the upper bit of the stream identifier and will always be '0'
    // pub reserved: bool,
    pub stream_identifier: u32,
    pub payload: Vec<u8>,
}
