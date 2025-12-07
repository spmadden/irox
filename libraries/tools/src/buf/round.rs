// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
#![allow(clippy::indexing_slicing)]
#![allow(clippy::unwrap_used)]

use crate::buf::Buffer;
use crate::iterators::Moderator;
use core::ops::{Index, IndexMut};
use irox_bits::{Bits, BitsError, BitsErrorKind, Error, ErrorKind, MutBits};

///
/// Double-ended circular Buffer.  Basically a fixed size [`std::collections::VecDeque`]
#[derive(Debug, Clone)]
pub struct RoundBuffer<const N: usize, T: Sized> {
    buf: [Option<T>; N],
    head: usize,
    tail: usize,
    size: usize,
    mod_count: u32,
}

impl<const N: usize, T: Default + Copy + Sized> RoundBuffer<N, T> {
    pub fn pop_n_front<const L: usize>(&mut self) -> Option<[T; L]> {
        if self.size < L || N < L {
            return None;
        }
        self.size -= L;
        self.mod_count = self.mod_count.wrapping_add(1);
        let mut out = [T::default(); L];
        for out in out.iter_mut().take(L) {
            *out = self.buf[self.head].take().unwrap_or_default();
            // move the head pointer forward one
            // unless head == tail
            if self.head != self.tail {
                self.head += 1;
            }
            // if head >= N, then wrap around
            if self.head >= N {
                self.head = 0;
            }
        }
        Some(out)
    }
}

/// Circular buffer iterator, just calls `pop_front()` repeatedly
pub struct RoundBufferIter<const N: usize, T: Sized> {
    buf: RoundBuffer<N, T>,
}

impl<const N: usize, T: Sized> Iterator for RoundBufferIter<N, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.pop_front()
    }
}

impl<const N: usize, T: Sized> IntoIterator for RoundBuffer<N, T> {
    type Item = T;
    type IntoIter = RoundBufferIter<N, T>;

    fn into_iter(self) -> Self::IntoIter {
        RoundBufferIter { buf: self }
    }
}
pub struct Iter<'a, const N: usize, T: Sized> {
    buf: &'a RoundBuffer<N, T>,
    iter: Moderator,
}
impl<'a, const N: usize, T: Sized> Iterator for Iter<'a, N, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.iter.next()?;
        self.buf.get(idx)
    }
}
impl<const N: usize, T: Sized> RoundBuffer<N, T> {
    const VAL: Option<T> = None;

    pub fn new() -> Self {
        RoundBuffer {
            buf: [Self::VAL; N],
            head: 0,
            tail: 0,
            size: 0,
            mod_count: 0,
        }
    }
    pub fn iter(&self) -> Iter<'_, N, T> {
        let iter = Moderator::new_limited(self.head, N, self.size);
        Iter { buf: self, iter }
    }
    pub fn moderator(&self) -> Moderator {
        Moderator::new_limited(self.head, N, self.size)
    }
    pub fn capacity(&self) -> usize {
        N
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn clear(&mut self) {}
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn is_full(&self) -> bool {
        self.size == N
    }

    ///
    /// Provides access to the raw internal buffer.
    pub fn raw_buf<R, F: FnMut(&[Option<T>]) -> R>(&self, mut f: F) -> R {
        f(&self.buf)
    }
}

impl<const N: usize, T: Sized> Default for RoundBuffer<N, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, T> Drop for RoundBuffer<N, T> {
    fn drop(&mut self) {
        for val in &mut self.buf {
            if let Some(v) = val.take() {
                drop(v);
            }
        }
    }
}

impl<const N: usize, T> Buffer<T> for RoundBuffer<N, T> {
    fn get(&self, index: usize) -> Option<&T> {
        if index >= N || index >= self.size {
            return None;
        }
        self.buf[index].as_ref()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= N || index >= self.size {
            return None;
        }
        self.buf[index].as_mut()
    }

    fn capacity(&self) -> usize {
        N
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.size = 0;
        self.mod_count = self.mod_count.wrapping_add(1);
    }

    fn front(&self) -> Option<&T> {
        self.buf[self.head].as_ref()
    }

    fn front_mut(&mut self) -> Option<&mut T> {
        self.buf[self.head].as_mut()
    }

    fn back(&self) -> Option<&T> {
        self.buf[self.tail].as_ref()
    }

    fn back_mut(&mut self) -> Option<&mut T> {
        self.buf[self.tail].as_mut()
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.size == 0 || N == 0 {
            return None;
        }
        self.size -= 1;
        self.mod_count = self.mod_count.wrapping_add(1);

        let out = self.buf[self.head].take();
        // move the head pointer forward one
        // unless head == tail
        if self.head != self.tail {
            self.head += 1;
        }
        // if head >= N, then wrap around
        if self.head >= N {
            self.head = 0;
        }
        out
    }

    fn pop_back(&mut self) -> Option<T> {
        if self.size == 0 || N == 0 {
            // empty
            return None;
        }
        let out = self.buf[self.tail].take();
        self.mod_count = self.mod_count.wrapping_add(1);

        self.size -= 1;
        // move the tail pointer back
        // unless head == tail
        if self.head != self.tail {
            // if tail == 0, wrap around
            if self.tail == 0 {
                self.tail = N - 1;
            } else {
                self.tail -= 1;
            }
        }
        out
    }

    fn push_front(&mut self, value: T) -> Result<(), T> {
        if self.size == N || N == 0 {
            // full
            Err(value)
        } else if self.size == 0 {
            self.mod_count = self.mod_count.wrapping_add(1);

            self.head = 0;
            self.tail = 0;
            self.buf[0] = Some(value);
            self.size = 1;
            Ok(())
        } else {
            self.mod_count = self.mod_count.wrapping_add(1);

            if self.head == 0 {
                self.head = N - 1;
            }
            self.buf[self.head] = Some(value);
            self.size += 1;
            Ok(())
        }
    }

    fn push_back(&mut self, value: T) -> Result<(), T> {
        if self.size == N || N == 0 {
            // full
            Err(value)
        } else if self.size == 0 {
            self.mod_count = self.mod_count.wrapping_add(1);

            // empty
            self.head = 0;
            self.tail = 0;
            self.size = 1;
            self.buf[0] = Some(value);
            Ok(())
        } else {
            self.mod_count = self.mod_count.wrapping_add(1);

            // mixed
            self.size += 1;
            self.tail += 1;
            if self.tail == N {
                self.tail = 0;
            }
            self.buf[self.tail] = Some(value);
            Ok(())
        }
    }
}

impl<const N: usize> Bits for RoundBuffer<N, u8> {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        Ok(self.pop_front())
    }

    fn read_be_u32(&mut self) -> Result<u32, Error> {
        let a = self
            .pop_n_front::<4>()
            .ok_or_else(|| BitsError::new(BitsErrorKind::UnexpectedEof, "EOF"))?;
        Ok(u32::from_be_bytes(a))
    }
}
impl<const N: usize> Bits for &mut RoundBuffer<N, u8> {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        Ok(self.pop_front())
    }

    fn read_be_u32(&mut self) -> Result<u32, Error> {
        let a = self
            .pop_n_front::<4>()
            .ok_or_else(|| BitsError::new(BitsErrorKind::UnexpectedEof, "EOF"))?;
        Ok(u32::from_be_bytes(a))
    }
}

impl<const N: usize> MutBits for RoundBuffer<N, u8> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.push_back(val)
            .map_err(|_| ErrorKind::OutOfMemory.into())
    }
}

impl<const N: usize> MutBits for &mut RoundBuffer<N, u8> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.push_back(val)
            .map_err(|_| ErrorKind::OutOfMemory.into())
    }
}

#[allow(clippy::panic)]
impl<const N: usize, T> Index<usize> for RoundBuffer<N, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size, "{index} >= {}", self.size);
        let mut offset = self.head + index;
        if offset >= N {
            offset -= N;
        }
        let Some(Some(val)) = self.buf.get(offset) else {
            panic!("expected value at offset {offset} but was empty!");
        };
        val
    }
}
#[allow(clippy::panic)]
impl<const N: usize, T> IndexMut<usize> for RoundBuffer<N, T>
where
    T: Default,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < N, "index {index} >= capacity {N}");
        let mut offset = self.head + index;
        if offset >= N {
            offset -= N;
        }
        if self.buf[offset].is_none() {
            self.size += 1;
            self.buf[offset] = Some(Default::default());
        }
        self.buf[offset].as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::buf::Buffer;

    macro_rules! assert_empty {
        ($buf:ident) => {
            assert_eq!(0, $buf.len());
            assert_eq!(None, $buf.pop_front());
            assert_eq!(None, $buf.pop_back());
        };
    }

    macro_rules! assert_full {
        ($buf:ident, $sz:literal, $elem:expr) => {
            assert_eq!($sz, $buf.len());
            assert_eq!(Err($elem), $buf.push_back($elem));
            assert_eq!($sz, $buf.len());
            assert_eq!(Err($elem), $buf.push_front($elem));
            assert_eq!($sz, $buf.len());
        };
    }

    #[test]
    pub fn test_push_some() -> Result<(), u32> {
        let mut buf = crate::buf::RoundBuffer::<3, u32>::default();
        assert_empty!(buf);

        buf.push_back(10)?;
        assert_eq!(1, buf.len());
        assert_eq!(0, buf.head);
        assert_eq!(0, buf.tail);

        buf.push_back(15)?;
        assert_eq!(2, buf.len());
        assert_eq!(0, buf.head);
        assert_eq!(1, buf.tail);

        buf.push_back(20)?;
        assert_eq!(3, buf.len());
        assert_eq!(0, buf.head);
        assert_eq!(2, buf.tail);

        assert_full!(buf, 3, 25);

        assert_eq!(Some(10), buf.pop_front());
        assert_eq!(2, buf.len());
        assert_eq!(1, buf.head);
        assert_eq!(2, buf.tail);

        buf.push_back(30)?;
        assert_eq!(3, buf.len());
        assert_eq!(1, buf.head);
        assert_eq!(0, buf.tail);

        assert_full!(buf, 3, 35);

        assert_eq!(Some(15), buf.pop_front());
        assert_eq!(2, buf.len());
        assert_eq!(2, buf.head);
        assert_eq!(0, buf.tail);

        assert_eq!(Some(20), buf.pop_front());
        assert_eq!(1, buf.len());
        assert_eq!(0, buf.head);
        assert_eq!(0, buf.tail);

        assert_eq!(Some(30), buf.pop_front());
        assert_eq!(0, buf.len());
        assert_eq!(0, buf.head);
        assert_eq!(0, buf.tail);

        assert_empty!(buf);
        assert_empty!(buf);

        Ok(())
    }
}
