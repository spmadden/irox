// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::types::Record;
use irox_bits::{Bits, SeekRead};

pub struct ZipScanner<T: Bits + SeekRead> {
    pub inner: T,
    pub pos: u64,
}

impl<T: Bits + SeekRead> ZipScanner<T> {
    pub fn new(inner: T) -> Self {
        Self { inner, pos: 0 }
    }
}

impl<T: Bits + SeekRead> Iterator for ZipScanner<T> {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
