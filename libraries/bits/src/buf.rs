// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Bits, Error};

///
/// Buffered bits - semantically equivalent to BufRead
pub trait BufBits: Bits {
    fn fill_buf(&mut self) -> Result<&[u8], Error>;
    fn consume(&mut self, amt: usize);

    fn has_data_left(&mut self) -> Result<bool, Error> {
        self.fill_buf().map(|b| !b.is_empty())
    }
}

cfg_feature_alloc! {
    mod alloc;
    pub use alloc::*;
}
