// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

extern crate alloc;
use alloc::collections::BTreeMap;
use irox_bits::{BitsWrapper, MutBits};

pub struct LZWEncoder<T: MutBits> {
    strtable: BTreeMap<String, u16>,
    delegate: BitsWrapper<T>
}
