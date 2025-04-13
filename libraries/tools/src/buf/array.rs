// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::indexing_slicing)]

use core::ops::{Index, IndexMut};
use irox_bits::{Error, ErrorKind, MutBits};

pub type U32ArrayBuf<const N: usize> = ArrayBuf<N, 4, u32>;
pub type U64ArrayBuf<const N: usize> = ArrayBuf<N, 8, u64>;
pub struct ArrayBuf<const N: usize, const O: usize, T: Default + Copy + Sized> {
    buf: [T; N],
    b2: [u8; O],
    b2_used: usize,
    size: usize,
}
impl<const N: usize, const O: usize, T: Default + Copy> Default for ArrayBuf<N, O, T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const N: usize, const O: usize, T: Default + Copy> ArrayBuf<N, O, T> {
    pub fn new() -> Self {
        Self {
            buf: [T::default(); N],
            b2: [0u8; O],
            size: 0,
            b2_used: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn rem_align(&self) -> usize {
        O - self.b2_used
    }
}

impl<const N: usize> ArrayBuf<N, 4, u32> {
    pub fn push_back(&mut self, val: u8) -> Result<(), Error> {
        self.write_u8(val)
    }
    pub fn is_full(&self) -> bool {
        self.size == N
    }

    pub fn push_prim(&mut self, val: u32) -> Result<(), Error> {
        let size = self.size;
        if size == N {
            return Err(ErrorKind::OutOfMemory.into());
        }
        self.buf[size] = val;
        self.size += 1;
        Ok(())
    }
    pub fn write_le_u8(&mut self, val: u8) -> Result<(), Error> {
        let size = self.size;
        if size == N {
            return Err(ErrorKind::OutOfMemory.into());
        }
        self.b2[self.b2_used] = val;
        self.b2_used += 1;

        if self.b2_used >= 4 {
            self.b2_used = 0;
            self.buf[size] = u32::from_le_bytes(self.b2);
            self.size += 1;
        }
        Ok(())
    }
    pub fn take_le_buf(&mut self) -> [u32; N] {
        if self.b2_used > 0 {
            self.b2[self.b2_used..].fill(0);
            self.b2_used = 0;
            self.buf[self.size] = u32::from_le_bytes(self.b2);
        }
        self.size = 0;
        let out = self.buf;
        self.buf.fill(0);
        out
    }
    pub fn take_be_buf(&mut self) -> [u32; N] {
        if self.b2_used > 0 {
            self.b2[self.b2_used..].fill(0);
            self.b2_used = 0;
            self.buf[self.size] = u32::from_be_bytes(self.b2);
        }
        self.size = 0;
        let out = self.buf;
        self.buf.fill(0);
        out
    }
}

impl<const N: usize> MutBits for ArrayBuf<N, 4, u32> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        let size = self.size;
        if size == N {
            return Err(ErrorKind::OutOfMemory.into());
        }
        self.b2[self.b2_used] = val;
        self.b2_used += 1;

        if self.b2_used >= 4 {
            self.b2_used = 0;
            self.buf[self.size] = u32::from_be_bytes(self.b2);
            self.size += 1;
        }
        Ok(())
    }

    fn write_be_u32(&mut self, val: u32) -> Result<(), Error> {
        let size = self.size;
        if size == N {
            return Err(ErrorKind::OutOfMemory.into());
        }
        self.buf[self.size] = val;
        self.size += 1;
        Ok(())
    }
}

impl<const N: usize, const O: usize> Index<usize> for ArrayBuf<N, O, u32> {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl<const N: usize, const O: usize> IndexMut<usize> for ArrayBuf<N, O, u32> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}

impl<const N: usize> ArrayBuf<N, 8, u64> {
    pub fn push_back(&mut self, val: u8) -> Result<(), Error> {
        self.write_u8(val)
    }
    pub fn is_full(&self) -> bool {
        self.size == N
    }

    pub fn push_prim(&mut self, val: u64) -> Result<(), Error> {
        let size = self.size;
        if size == N {
            return Err(ErrorKind::OutOfMemory.into());
        }
        self.buf[self.size] = val;
        self.size += 1;
        Ok(())
    }
    pub fn write_le_u8(&mut self, val: u8) -> Result<(), Error> {
        let size = self.size;
        if size == N {
            return Err(ErrorKind::OutOfMemory.into());
        }
        self.b2[self.b2_used] = val;
        self.b2_used += 1;

        if self.b2_used >= 8 {
            self.b2_used = 0;
            self.buf[self.size] = u64::from_le_bytes(self.b2);
            self.size += 1;
        }
        Ok(())
    }
    pub fn take_le_buf(&mut self) -> [u64; N] {
        if self.b2_used > 0 {
            self.b2[self.b2_used..].fill(0);
            self.b2_used = 0;
            self.buf[self.size] = u64::from_le_bytes(self.b2);
        }
        self.size = 0;
        let out = self.buf;
        self.buf.fill(0);
        out
    }
    pub fn take_be_buf(&mut self) -> [u64; N] {
        if self.b2_used > 0 {
            self.b2[self.b2_used..].fill(0);
            self.b2_used = 0;
            self.buf[self.size] = u64::from_be_bytes(self.b2);
        }
        self.size = 0;
        let out = self.buf;
        self.buf.fill(0);
        out
    }
}

impl<const N: usize> MutBits for ArrayBuf<N, 8, u64> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        let size = self.size;
        if size == N {
            return Err(ErrorKind::OutOfMemory.into());
        }
        self.b2[self.b2_used] = val;
        self.b2_used += 1;

        if self.b2_used >= 8 {
            self.b2_used = 0;
            self.buf[self.size] = u64::from_be_bytes(self.b2);
            self.size += 1;
            self.b2.fill(0);
        }
        Ok(())
    }

    fn write_be_u64(&mut self, val: u64) -> Result<(), Error> {
        let size = self.size;
        if size == N {
            return Err(ErrorKind::OutOfMemory.into());
        }
        self.buf[self.size] = val;
        self.size += 1;
        Ok(())
    }
}

impl<const N: usize, const O: usize> Index<usize> for ArrayBuf<N, O, u64> {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl<const N: usize, const O: usize> IndexMut<usize> for ArrayBuf<N, O, u64> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}
