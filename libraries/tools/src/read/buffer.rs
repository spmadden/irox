// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use alloc::collections::VecDeque;
use core::cmp::min;
use core::ops::RangeBounds;
use std::io::{BufRead, BufReader, Read};

///
/// An effective, infinite cached buffer.  What [`BufRead`] and [`BufReader`]
/// would be if they weren't backed by a fixed buffer.
///
/// Calls to `read()` and `next()` fill the internal buffer & peek the data, but do not consume it.  Call either
/// `drain()` or one of the `consume_*` functions.
#[derive()]
pub struct Buffer<T> {
    reader: BufReader<T>,
    buffer: VecDeque<u8>,
}

impl<T: Read> Buffer<T> {
    ///
    pub fn new(reader: T) -> Self {
        Buffer {
            reader: BufReader::new(reader),
            buffer: VecDeque::new(),
        }
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
}

impl<T: Read> Read for Buffer<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.buffer.is_empty() {
            let mut buf: [u8; 4096] = [0; 4096];
            let read = self.reader.read(&mut buf)?;
            self.buffer.extend(buf.get(0..read).unwrap_or_default());
        }
        let read = self.buffer.read(buf)?;
        Ok(read)
    }
}

impl<T: Read> Iterator for Buffer<T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: [u8; 1] = [0; 1];
        let Ok(val) = self.reader.read(&mut buf) else {
            return None;
        };
        if val == 1 {
            self.buffer.push_back(buf[0]);
            return Some(buf[0]);
        }
        None
    }
}

impl<T: Read> BufRead for Buffer<T> {
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
