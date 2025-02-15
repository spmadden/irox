// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::indexing_slicing)]
#![allow(clippy::unwrap_used)]
use crate::buf::Buffer;
use crate::iterators::LendingIterator;
use core::ops::{Index, IndexMut};
use core::str::Utf8Error;
use irox_bits::{Bits, BitsErrorKind, Error, MutBits, ReadFromBEBits, WriteToBEBits};

pub type StrBuf<const N: usize> = FixedU8Buf<N>;

///
/// Fixed length stack allocated buffer.  Basically a stack-allocated [`Vec`]
#[derive(Clone)]
pub struct FixedU8Buf<const N: usize> {
    buf: [u8; N],
    len: usize,
}
impl<const N: usize> FixedU8Buf<N> {
    pub const fn new() -> Self {
        Self {
            buf: [0u8; N],
            len: 0,
        }
    }

    ///
    /// Returns a copy of the full buffer
    pub fn as_buf_default(&mut self) -> [u8; N] {
        let out = core::mem::replace(&mut self.buf, [0u8; N]);
        self.clear();
        out
    }

    ///
    /// Returns an iterator to the data in this buffer
    pub fn iter(&self) -> FixedU8BufIter<N> {
        FixedU8BufIter { buf: self, idx: 0 }
    }

    ///
    /// Returns the used length of this buffer.
    pub const fn len(&self) -> usize {
        self.len
    }

    ///
    /// Returns true if this buffer is empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    ///
    /// Returns a slice to only the used portion of this buffer
    pub fn as_ref_used(&self) -> &[u8] {
        self.as_ref()
    }

    ///
    /// Returns a mut slice to the full capacity of this buffer.  If you change the used
    /// length of the internal buffer, ensure you call [`update_length`] to correct the
    /// internal length tracking.
    pub fn as_mut_full(&mut self) -> &mut [u8] {
        self.as_mut()
    }

    ///
    /// Returns a mut slice to only the used capacity of this buffer.
    pub fn as_mut_used(&mut self) -> &mut [u8] {
        &mut self.buf[0..self.len]
    }

    ///
    /// Manually updates the "used length" of the buffer.  This is expected to be used in
    /// concert with [`as_mut_full`] when loading data into the buffer.
    pub fn update_length(&mut self, new_len: usize) -> Result<(), Error> {
        if new_len > N {
            return Err(BitsErrorKind::OutOfMemory.into());
        }
        self.len = new_len;
        Ok(())
    }

    ///
    /// Pushes a single unicode character into this buffer.
    pub fn push_char(&mut self, c: char) -> Result<(), Error> {
        let mut buf = [0u8; 4];
        let used = c.encode_utf8(&mut buf).len();
        if self.len + used > N {
            return Err(BitsErrorKind::OutOfMemory.into());
        }

        Ok(())
    }

    ///
    /// Appends the specified slice to this buffer.
    pub fn append(&mut self, buf: &[u8]) -> Result<(), Error> {
        if self.len + buf.len() > N {
            return Err(BitsErrorKind::OutOfMemory.into());
        }
        for b in buf {
            let _ = self.push_back(*b);
        }
        Ok(())
    }

    ///
    /// Returns the contents of this buffer as a str
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        core::str::from_utf8(self.as_ref_used())
    }
    ///
    /// Returns the contents of this buffer as a mutable str
    pub fn as_str_mut(&mut self) -> Result<&mut str, Utf8Error> {
        core::str::from_utf8_mut(self.as_mut_used())
    }

    ///
    /// Basic in-place swap.
    pub fn reverse(&mut self) {
        let mut i = 0;
        let mut j = self.len - 1;
        while i < j {
            self.buf.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}
impl<const N: usize> WriteToBEBits for FixedU8Buf<N> {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_be_u32_blob(self.as_ref_used())?;
        Ok(self.len + 4)
    }
}
impl<const N: usize> ReadFromBEBits for FixedU8Buf<N> {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        let mut out = Self::new();
        inp.read_u32_blob_into(&mut out)?;
        Ok(out)
    }
}

impl<const N: usize> AsRef<[u8]> for FixedU8Buf<N> {
    #[allow(clippy::indexing_slicing)]
    fn as_ref(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}
impl<const N: usize> AsMut<[u8]> for FixedU8Buf<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.buf
    }
}
impl<const N: usize> Default for FixedU8Buf<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> core::fmt::Write for FixedU8Buf<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.append(s.as_bytes()).map_err(|_| core::fmt::Error)?;
        Ok(())
    }
}

impl<const N: usize> Buffer<u8> for FixedU8Buf<N> {
    fn get(&self, index: usize) -> Option<&u8> {
        if index >= N || index >= self.len {
            return None;
        }
        self.buf.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut u8> {
        if index >= N || index >= self.len {
            return None;
        }
        self.buf.get_mut(index)
    }

    fn capacity(&self) -> usize {
        N
    }

    fn len(&self) -> usize {
        self.len
    }

    fn clear(&mut self) {
        self.len = 0
    }

    fn front(&self) -> Option<&u8> {
        self.get(0)
    }

    fn front_mut(&mut self) -> Option<&mut u8> {
        self.get_mut(0)
    }

    fn back(&self) -> Option<&u8> {
        if N == 0 || self.len == 0 {
            return None;
        }
        self.get(self.len - 1)
    }

    fn back_mut(&mut self) -> Option<&mut u8> {
        if N == 0 || self.len == 0 {
            return None;
        }
        self.get_mut(self.len - 1)
    }

    fn pop_front(&mut self) -> Option<u8> {
        if N == 0 || self.len == 0 {
            return None;
        }
        let out = self.buf[0];
        for idx in 1..self.len {
            self.buf[idx - 1] = self.buf[idx];
        }
        self.len -= 1;
        Some(out)
    }

    fn pop_back(&mut self) -> Option<u8> {
        if N == 0 || self.len == 0 {
            return None;
        }
        let idx = self.len - 1;
        self.len -= 1;
        let val = self.buf[idx];
        self.buf[idx] = 0;
        Some(val)
    }

    fn push_front(&mut self, value: u8) -> Result<(), u8> {
        if N == 0 || self.len == N {
            return Err(value);
        }
        for idx in 0..self.len {
            self.buf[idx + 1] = self.buf[idx];
        }
        self.buf[0] = value;
        self.len += 1;
        Ok(())
    }

    fn push_back(&mut self, value: u8) -> Result<(), u8> {
        if N == 0 || self.len == N {
            return Err(value);
        }
        self.buf[self.len] = value;
        self.len += 1;
        Ok(())
    }
}

#[allow(clippy::panic)]
impl<const N: usize> Index<usize> for FixedU8Buf<N> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "index {index} >= len {}", self.len);
        let Some(val) = self.buf.get(index) else {
            panic!("expected value at offset {index} but was empty!");
        };
        val
    }
}
#[allow(clippy::panic)]
impl<const N: usize> IndexMut<usize> for FixedU8Buf<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < N, "index {index} >= capacity {N}");
        if index >= self.len {
            self.len = index + 1;
        }
        &mut self.buf[index]
    }
}

pub struct FixedU8BufIter<'a, const N: usize> {
    buf: &'a FixedU8Buf<N>,
    idx: usize,
}

impl<'a, const N: usize> Iterator for FixedU8BufIter<'a, N> {
    type Item = &'a u8;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.buf.get(self.idx) {
            self.idx += 1;
            return Some(val);
        }
        None
    }
}
impl<const N: usize> DoubleEndedIterator for FixedU8BufIter<'_, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx >= self.buf.len {
            return None;
        }
        let idx = self.buf.len().saturating_sub(self.idx).saturating_sub(1);

        self.idx += 1;
        if let Some(val) = self.buf.get(idx) {
            return Some(val);
        }
        None
    }
}
impl<const N: usize> ExactSizeIterator for FixedU8BufIter<'_, N> {
    fn len(&self) -> usize {
        self.buf.len()
    }
}

pub struct FixedU8BufIterMut<'a, const N: usize> {
    buf: &'a mut FixedU8Buf<N>,
    idx: usize,
}

impl<'a, const N: usize> LendingIterator<'a> for FixedU8BufIterMut<'a, N> {
    type Item<'b>
        = &'a mut u8
    where
        Self: 'b;

    fn next_ref(&'a mut self) -> Option<Self::Item<'a>> {
        if let Some(val) = self.buf.get_mut(self.idx) {
            self.idx += 1;
            return Some(val);
        }
        None
    }
}

impl<const N: usize> MutBits for &mut FixedU8Buf<N> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        if self.push_back(val).is_err() {
            return Err(BitsErrorKind::UnexpectedEof.into());
        }
        Ok(())
    }
}
impl<const N: usize> MutBits for FixedU8Buf<N> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        if self.push_back(val).is_err() {
            return Err(BitsErrorKind::UnexpectedEof.into());
        }
        Ok(())
    }
}
