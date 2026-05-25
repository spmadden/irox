// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use irox_structs::Struct;

#[derive(Debug, Clone, Struct)]
pub struct MSDOSStub {
    pub data: [u8; 0x3C],
}
