// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::msgs::ChronycRequestCommandType;
use irox_bits::{Bits, Error};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChronycResponse {
    pub version: u8,
    pub packet_type: u8,
    pub reserved1: u16,
    pub command: ChronycRequestCommandType,
    pub reply_format: u16,
    pub status: u16,
    pub pad1: u16,
    pub pad2: u16,
    pub pad3: u16,
    pub sequence: u32,
    pub pad4: u32,
    pub pad5: u32,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ChronycResponsePayload {
    None,
    NSources(u32),
}
impl ChronycResponsePayload {
    pub fn read_from<T: Bits>(_inp: &mut T) -> Result<Self, Error> {
        todo!()
    }
}
