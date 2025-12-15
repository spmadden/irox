// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::buf::{Buffer, FixedU8Buf};
use irox_bits::Bits;

pub struct BitsChunks<'a, const N: usize, T: Bits + 'a> {
    inner: &'a mut T,
    buf: FixedU8Buf<N>,
}
impl<'a, const N: usize, T: Bits + 'a> Iterator for BitsChunks<'a, N, T>
where
    &'a mut Self: 'a,
{
    type Item = FixedU8Buf<N>;

    fn next(&mut self) -> Option<FixedU8Buf<N>> {
        for _ in 0..N {
            if let Some(v) = self.inner.next_u8().ok()? {
                let _ = self.buf.push_back(v);
            } else {
                break;
            }
        }
        if self.buf.is_empty() {
            None
        } else {
            Some(core::mem::take(&mut self.buf))
        }
    }
}

pub trait BChunks: Bits {
    fn chunks<const N: usize>(&'_ mut self) -> BitsChunks<'_, N, Self>
    where
        Self: Sized;
}

impl<T> BChunks for T
where
    T: Bits,
{
    fn chunks<const N: usize>(&'_ mut self) -> BitsChunks<'_, N, Self>
    where
        Self: Sized,
    {
        BitsChunks {
            inner: self,
            buf: Default::default(),
        }
    }
}
