// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::indexing_slicing)]
#![allow(clippy::unwrap_used)]
use crate::buf::Buffer;
use crate::iterators::LendingIterator;
use crate::options::MaybeMap;
use core::iter::zip;
use core::ops::{Index, IndexMut};
use irox_bits::{BitsError, BitsErrorKind, Error, MutBits, WriteToBEBits};
// pub type StrBuf<const N: usize> = FixedBuf<N, char>;

///
/// Fixed length stack allocated buffer.  Basically a stack-allocated [`std::vec::Vec`]
#[derive(Clone)]
pub struct FixedBuf<const N: usize, T: Sized> {
    buf: [Option<T>; N],
    len: usize,
}
impl<const N: usize, T: Sized> FixedBuf<N, T> {
    const NONE_T: Option<T> = None;

    pub const fn new() -> Self {
        Self {
            buf: [Self::NONE_T; N],
            len: 0,
        }
    }
}
impl<const N: usize, T: Sized> Default for FixedBuf<N, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> core::fmt::Write for FixedBuf<N, u8> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.push(*b).map_err(|_| core::fmt::Error)?;
        }
        Ok(())
    }
}
impl<const N: usize> core::fmt::Write for FixedBuf<N, char> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.chars() {
            self.push(b).map_err(|_| core::fmt::Error)?;
        }
        Ok(())
    }
}

impl<const N: usize, T: Sized> Buffer<T> for FixedBuf<N, T> {
    fn get(&self, index: usize) -> Option<&T> {
        if index >= N || index >= self.len {
            return None;
        }
        self.buf.get(index).maybe_map(Option::as_ref)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= N || index >= self.len {
            return None;
        }
        self.buf.get_mut(index).maybe_map(Option::as_mut)
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

    fn front(&self) -> Option<&T> {
        self.get(0)
    }

    fn front_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    fn back(&self) -> Option<&T> {
        if N == 0 || self.len == 0 {
            return None;
        }
        self.get(self.len - 1)
    }

    fn back_mut(&mut self) -> Option<&mut T> {
        if N == 0 || self.len == 0 {
            return None;
        }
        self.get_mut(self.len - 1)
    }

    fn pop_front(&mut self) -> Option<T> {
        if N == 0 || self.len == 0 {
            return None;
        }
        let out = self.buf[0].take();
        for idx in 1..self.len {
            self.buf[idx - 1] = self.buf[idx].take();
        }
        self.len -= 1;
        out
    }

    fn pop_back(&mut self) -> Option<T> {
        if N == 0 || self.len == 0 {
            return None;
        }
        let idx = self.len - 1;
        self.len -= 1;
        self.buf[idx].take()
    }

    fn push_front(&mut self, value: T) -> Result<(), T> {
        if N == 0 || self.len == N {
            return Err(value);
        }
        for idx in 0..self.len {
            self.buf[idx + 1] = self.buf[idx].take();
        }
        self.buf[0] = Some(value);
        self.len += 1;
        Ok(())
    }

    fn push_back(&mut self, value: T) -> Result<(), T> {
        if N == 0 || self.len == N {
            return Err(value);
        }
        self.buf[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }
}

#[allow(clippy::panic)]
impl<const N: usize, T> Index<usize> for FixedBuf<N, T>
where
    T: Default,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "index {index} >= len {}", self.len);
        let Some(Some(val)) = self.buf.get(index) else {
            panic!("expected value at offset {index} but was empty!");
        };
        val
    }
}
#[allow(clippy::panic)]
impl<const N: usize, T> IndexMut<usize> for FixedBuf<N, T>
where
    T: Default,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < N, "index {index} >= capacity {N}");
        if self.buf[index].is_none() {
            self.len += 1;
            self.buf[index] = Some(Default::default());
        }
        self.buf[index].as_mut().unwrap()
    }
}

impl<const N: usize, T: Sized + Default + Copy> FixedBuf<N, T> {
    pub fn into_buf_default(&mut self) -> [T; N] {
        let mut out = [T::default(); N];
        for (i, o) in zip(self.buf.iter_mut(), out.iter_mut()) {
            if let Some(val) = i.take() {
                *o = val;
            }
        }
        self.clear();
        out
    }
}
impl<const N: usize, T: Sized> FixedBuf<N, T> {
    pub fn iter(&self) -> FixedBufIter<N, T> {
        FixedBufIter { buf: self, idx: 0 }
    }
}
impl<const N: usize, T: Sized + WriteToBEBits> FixedBuf<N, T> {
    pub fn write_to<B: MutBits + ?Sized>(&self, out: &mut B) -> Result<usize, BitsError> {
        for v in self.iter() {
            WriteToBEBits::write_be_to(v, out)?;
        }
        Ok(self.len)
    }
}
pub struct FixedBufIter<'a, const N: usize, T: Sized> {
    buf: &'a FixedBuf<N, T>,
    idx: usize,
}

impl<'a, const N: usize, T: Sized> Iterator for FixedBufIter<'a, N, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.buf.get(self.idx) {
            self.idx += 1;
            return Some(val);
        }
        None
    }
}
impl<const N: usize, T: Sized> DoubleEndedIterator for FixedBufIter<'_, N, T> {
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
impl<const N: usize, T: Sized> ExactSizeIterator for FixedBufIter<'_, N, T> {
    fn len(&self) -> usize {
        self.buf.len()
    }
}

pub struct FixedBufIterMut<'a, const N: usize, T: Sized> {
    buf: &'a mut FixedBuf<N, T>,
    idx: usize,
}

impl<const N: usize, T: Sized> LendingIterator for FixedBufIterMut<'_, N, T> {
    type Item<'b>
        = &'b mut T
    where
        Self: 'b;

    fn next_ref(&mut self) -> Option<Self::Item<'_>> {
        if let Some(val) = self.buf.get_mut(self.idx) {
            self.idx += 1;
            return Some(val);
        }
        None
    }
}

impl<const N: usize> MutBits for &mut FixedBuf<N, u8> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        if self.push_back(val).is_err() {
            return Err(BitsErrorKind::UnexpectedEof.into());
        }
        Ok(())
    }
}
impl<const N: usize> MutBits for FixedBuf<N, u8> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        if self.push_back(val).is_err() {
            return Err(BitsErrorKind::UnexpectedEof.into());
        }
        Ok(())
    }
}
