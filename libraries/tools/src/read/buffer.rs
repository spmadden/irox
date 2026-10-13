// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//
extern crate alloc;

use crate::cfg_feature_std;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::cmp::min;
use core::ops::RangeBounds;
use irox_bits::{Bits, BitsWrapper, Error};

///
/// An effective, infinite cached buffer.  What [`BufRead`] and [`BufReader`]
/// would be if they weren't backed by a fixed buffer.
///
/// Calls to `read()` and `next()` fill the internal buffer & peek the data, but do not consume it.  Call either
/// `drain()` or one of the `consume_*` functions.
pub struct Buffer<'a, T> {
    reader: BitsWrapper<'a, T>,
    buffer: VecDeque<u8>,
}

impl<'a, T: Bits> Buffer<'a, T> {
    /// Creates a new buffer
    pub fn new(reader: BitsWrapper<'a, T>) -> Self {
        Buffer {
            reader,
            buffer: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// The associated iterator call 'next' fills the buffer and returns the
    /// individual byte-by-byte values.  This function returns and clears the
    /// buffer as a consecutive block, up to 'len' items.
    pub fn consume_read_buffer_up_to(&mut self, len: usize) -> Vec<u8> {
        let len = min(len, self.buffer.len());
        self.buffer.drain(0..len).collect()
    }

    /// The associated iterator call 'next' fills the buffer and returns the
    /// individual byte-by-byte values. This function returns the entire read buffer.
    pub fn consume_read_buffer(&mut self) -> VecDeque<u8> {
        core::mem::take(&mut self.buffer)
    }

    /// Removes the specified range and throws it away.
    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) {
        self.buffer.drain(range);
    }

    fn try_fill(&mut self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            self.reader.read_some_into(&mut self.buffer)?;
        }
        Ok(())
    }
    cfg_feature_std! {
        pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
            use std::io::Read;
            self.try_fill()?;
            let read = self.buffer.read(buf)?;
            Ok(read)
        }
    }
}

impl<T: Bits> Bits for Buffer<'_, T> {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        self.try_fill()?;
        Ok(self.buffer.pop_front())
    }
}

impl<T: Bits> Iterator for Buffer<'_, T> {
    type Item = u8;

    // Iteration here does NOT consume the read buffer
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.reader.next_u8().ok()??;
        self.buffer.push_back(val);
        Some(val)
    }
}
cfg_feature_std! {
    impl<T: Bits> std::io::Read for Buffer<'_, T> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
            self.try_fill()?;
            let read = self.buffer.read(buf)?;
            Ok(read)
        }
    }
}
cfg_feature_std! {
    impl<T: irox_bits::BufBits> std::io::BufRead for Buffer<'_, T> {
        fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
            let buf = self.reader.fill_buf()?;
            let len = buf.len();
            self.buffer.extend(buf.iter());
            self.reader.consume(len);
            Ok(self.buffer.make_contiguous())
        }

        fn consume(&mut self, amt: usize) {
            self.consume_read_buffer_up_to(amt);
        }
    }
}
