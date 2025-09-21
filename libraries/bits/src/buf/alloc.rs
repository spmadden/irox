// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Bits, BitsWrapper, BufBits, Error, MutBits};
use alloc::collections::VecDeque;

pub struct BitsBuffer<'a, T> {
    inner: BitsWrapper<'a, T>,
    buf: VecDeque<u8>,
    flush_every_n: Option<usize>,
}
impl<'a, T> BitsBuffer<'a, T> {
    pub fn new(inner: BitsWrapper<'a, T>) -> Self {
        Self {
            inner,
            buf: VecDeque::with_capacity(16384),
            flush_every_n: None,
        }
    }
}

impl<'a, T: 'a> Bits for BitsBuffer<'a, T>
where
    BitsWrapper<'a, T>: Bits,
{
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        if let Some(b) = self.buf.pop_front() {
            return Ok(Some(b));
        }

        let _read = self.inner.read_some_into(&mut self.buf)?;

        Ok(self.buf.pop_front())
    }
}
impl<'a, T: 'a> BufBits for BitsBuffer<'a, T>
where
    BitsWrapper<'a, T>: Bits,
{
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
impl<T> BitsBuffer<'_, T> {
    pub fn flush_every_n(&mut self, n: usize) {
        self.flush_every_n = Some(n);
    }
}
impl<'a, T: 'a> MutBits for BitsBuffer<'a, T>
where
    BitsWrapper<'a, T>: MutBits,
{
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.buf.push_back(val);
        if let Some(flush_every) = self.flush_every_n {
            if self.buf.len() >= flush_every {
                self.flush()?;
            }
        }
        Ok(())
    }

    fn write_some_bytes(&mut self, mut val: &[u8]) -> usize {
        let len = val.len();
        let buflen = self.buf.len();
        let mut written = 0;
        if let Some(flush_every) = self.flush_every_n {
            if buflen + len > flush_every {
                let (a, b) = val.split_at(flush_every - buflen);
                val = b;
                let _ = self.buf.write_all_bytes(a);
                if let Err(_e) = self.flush() {
                    return written;
                }
                written += a.len();
            }
        }
        let _ = self.buf.write_all_bytes(val);
        written += val.len();
        if let Some(flush_every) = self.flush_every_n {
            if self.buf.len() == flush_every {
                let _ = self.flush();
            }
        }
        written
    }

    fn flush(&mut self) -> Result<(), Error> {
        if self.buf.is_empty() {
            return Ok(());
        }
        let slice = self.buf.make_contiguous();
        self.inner.write_all_bytes(slice)?;
        self.buf.clear();
        Ok(())
    }
}
