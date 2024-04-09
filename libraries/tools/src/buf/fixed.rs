// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

#![allow(clippy::indexing_slicing)]
#![allow(clippy::unwrap_used)]
use crate::options::MaybeMap;
use crate::Buffer;
use core::ops::{Index, IndexMut};

pub type StrBuf<const N: usize> = FixedBuf<N, char>;
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
