// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

#![allow(clippy::indexing_slicing)]
#![allow(clippy::unwrap_used)]
use crate::buf::Buffer;
use crate::iterators::LendingIterator;
use core::iter::zip;
use core::ops::{Index, IndexMut};
use irox_bits::{BitsError, BitsErrorKind, Error, MutBits, WriteToBEBits};

///
/// Fixed length stack allocated buffer.  Basically a stack-allocated [`std::vec::Vec`]
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
}
impl<const N: usize> Default for FixedU8Buf<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> core::fmt::Write for FixedU8Buf<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.push(*b).map_err(|_| core::fmt::Error)?;
        }
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

impl<const N: usize> FixedU8Buf<N> {
    pub fn into_buf_default(&mut self) -> [u8; N] {
        let mut out = [0u8; N];
        for (i, o) in zip(self.buf.iter_mut(), out.iter_mut()) {
            *o = *i;
        }
        self.clear();
        out
    }
}
impl<const N: usize> FixedU8Buf<N> {
    pub fn iter(&self) -> FixedU8BufIter<N> {
        FixedU8BufIter { buf: self, idx: 0 }
    }
}
impl<const N: usize> FixedU8Buf<N> {
    pub fn write_to<B: MutBits + ?Sized>(&self, out: &mut B) -> Result<(), BitsError> {
        for v in self.iter() {
            WriteToBEBits::write_be_to(v, out)?;
        }
        Ok(())
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
impl<'a, const N: usize> DoubleEndedIterator for FixedU8BufIter<'a, N> {
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
impl<'a, const N: usize> ExactSizeIterator for FixedU8BufIter<'a, N> {
    fn len(&self) -> usize {
        self.buf.len()
    }
}

pub struct FixedU8BufIterMut<'a, const N: usize> {
    buf: &'a mut FixedU8Buf<N>,
    idx: usize,
}

impl<'a, const N: usize> LendingIterator<'a> for FixedU8BufIterMut<'a, N> {
    type Item<'b> = &'a mut u8 where Self: 'b;

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
