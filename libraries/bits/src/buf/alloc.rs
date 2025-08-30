// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Bits, BitsWrapper, BufBits, Error};
use alloc::collections::VecDeque;

pub struct BitsBuffer<'a, T> {
    inner: BitsWrapper<'a, T>,
    buf: VecDeque<u8>,
}
impl<'a, T> BitsBuffer<'a, T> {
    pub fn new(inner: BitsWrapper<'a, T>) -> Self {
        Self {
            inner,
            buf: VecDeque::with_capacity(16384),
        }
    }
}

impl<T: Bits> Bits for BitsBuffer<'_, T> {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        if let Some(b) = self.buf.pop_front() {
            return Ok(Some(b));
        }

        let _read = self.inner.read_some_into(&mut self.buf)?;

        Ok(self.buf.pop_front())
    }
}
impl<T: Bits> BufBits for BitsBuffer<'_, T> {
    fn fill_buf(&mut self) -> Result<&[u8], Error> {
        if self.buf.is_empty() {
            let _read = self.inner.read_some_into(&mut self.buf)?;
        }
        let (a, b) = self.buf.as_slices();
        if a.is_empty() {
            return Ok(b);
        }
        Ok(a)
    }

    fn consume(&mut self, amt: usize) {
        self.buf.drain(..amt);
    }
}
