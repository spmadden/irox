// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

mod shb;

use crate::pcapng::shb::SectionHeaderBlock;
use irox_structs::Struct;

#[derive(Debug, Clone, Struct)]
pub struct Block {
    pub block_type: u32,
    pub block_body: Vec<u8>,
    pub block_total_len_trailer: u32,
}

pub enum BlockType {
    SectionHeader(SectionHeaderBlock),
    InterfaceDescription(),
    EnhancedPacket(),
    SimplePacket(),
    NameResolution(),
    InterfaceStatistics(),
    Custom(),

    #[deprecated]
    PacketBlock(),
}
