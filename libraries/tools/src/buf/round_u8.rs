// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//
#![allow(clippy::indexing_slicing)]
#![allow(clippy::unwrap_used)]

use crate::buf::Buffer;
use core::ops::Index;
use irox_bits::{Bits, BitsError, BitsErrorKind, Error, ErrorKind, MutBits};

///
/// Double-ended circular Buffer.  Basically a fixed size [`std::collections::VecDeque`]
pub struct RoundU8Buffer<const N: usize> {
    buf: [u8; N],
    head: usize,
    tail: usize,
    size: usize,
    mod_count: u32,
}

impl<const N: usize> RoundU8Buffer<N> {
    pub fn pop_n_front<const L: usize>(&mut self) -> Option<[u8; L]> {
        if self.size < L || N < L {
            return None;
        }
        self.size -= L;
        self.mod_count = self.mod_count.wrapping_add(1);
        let mut out = [0u8; L];
        for out in out.iter_mut().take(L) {
            *out = self.buf[self.head];
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

    /// Provides the function with a mutable ref to the inner buffer.  The function
    /// MUST return the updated "used" size of the buffer.
    pub fn as_ref_mut<F: FnMut(usize, &mut [u8]) -> Result<usize, BitsError>>(
        &mut self,
        mut func: F,
    ) -> Result<(), BitsError> {
        let used = func(self.size, &mut self.buf)?;
        self.size = used;
        Ok(())
    }

    pub fn limit(&mut self, limit: usize) -> Result<(), BitsError> {
        if limit >= N || self.size < limit {
            return BitsErrorKind::InvalidInput.err("Invalid limit");
        }
        self.size = limit;
        Ok(())
    }
}

impl<const N: usize> AsRef<[u8]> for RoundU8Buffer<N> {
    fn as_ref(&self) -> &[u8] {
        self.buf.split_at(self.size).0
    }
}

/// Circular buffer iterator, just calls `pop_front()` repeatedly
pub struct RoundU8BufferIter<const N: usize> {
    buf: RoundU8Buffer<N>,
}

impl<const N: usize> Iterator for RoundU8BufferIter<N> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.pop_front()
    }
}

impl<const N: usize> IntoIterator for RoundU8Buffer<N> {
    type Item = u8;
    type IntoIter = RoundU8BufferIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        RoundU8BufferIter { buf: self }
    }
}

impl<const N: usize> Default for RoundU8Buffer<N> {
    fn default() -> Self {
        RoundU8Buffer {
            buf: [Default::default(); N],
            head: 0,
            tail: 0,
            size: 0,
            mod_count: 0,
        }
    }
}

impl<const N: usize> Buffer<u8> for RoundU8Buffer<N> {
    fn get(&self, index: usize) -> Option<&u8> {
        if index >= N || index >= self.size {
            return None;
        }
        Some(&self.buf[index])
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut u8> {
        if index >= N || index >= self.size {
            return None;
        }
        Some(&mut self.buf[index])
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

    fn front(&self) -> Option<&u8> {
        Some(&self.buf[self.head])
    }

    fn front_mut(&mut self) -> Option<&mut u8> {
        Some(&mut self.buf[self.head])
    }

    fn back(&self) -> Option<&u8> {
        Some(&self.buf[self.tail])
    }

    fn back_mut(&mut self) -> Option<&mut u8> {
        Some(&mut self.buf[self.tail])
    }

    fn pop_front(&mut self) -> Option<u8> {
        if self.size == 0 || N == 0 {
            return None;
        }
        self.size -= 1;
        self.mod_count = self.mod_count.wrapping_add(1);

        let out = Some(self.buf[self.head]);
        // move the head pointer forward one
        if self.size > 0 {
            self.head += 1;
        }
        // if head >= N, then wrap around
        if self.head >= N {
            self.head = 0;
        }
        out
    }

    fn pop_back(&mut self) -> Option<u8> {
        if self.size == 0 || N == 0 {
            // empty
            return None;
        }
        let out = Some(self.buf[self.tail]);
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

    fn push_front(&mut self, value: u8) -> Result<(), u8> {
        if self.size == N || N == 0 {
            // full
            Err(value)
        } else if self.size == 0 {
            self.mod_count = self.mod_count.wrapping_add(1);

            self.head = 0;
            self.tail = 0;
            self.buf[0] = value;
            self.size = 1;
            Ok(())
        } else {
            self.mod_count = self.mod_count.wrapping_add(1);

            if self.head == 0 {
                self.head = N - 1;
            }
            self.buf[self.head] = value;
            self.size += 1;
            Ok(())
        }
    }

    fn push_back(&mut self, value: u8) -> Result<(), u8> {
        if self.size == N || N == 0 {
            // full
            Err(value)
        } else if self.size == 0 {
            self.mod_count = self.mod_count.wrapping_add(1);

            // empty
            self.head = 0;
            self.tail = 0;
            self.size = 1;
            self.buf[0] = value;
            Ok(())
        } else {
            self.mod_count = self.mod_count.wrapping_add(1);

            // mixed
            self.size += 1;
            self.tail += 1;
            if self.tail == N {
                self.tail = 0;
            }
            self.buf[self.tail] = value;
            Ok(())
        }
    }
}

impl<const N: usize> Bits for RoundU8Buffer<N> {
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
impl<const N: usize> Bits for &mut RoundU8Buffer<N> {
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

impl<const N: usize> MutBits for RoundU8Buffer<N> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.push_back(val)
            .map_err(|_| ErrorKind::OutOfMemory.into())
    }
}

impl<const N: usize> MutBits for &mut RoundU8Buffer<N> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.push_back(val)
            .map_err(|_| ErrorKind::OutOfMemory.into())
    }
}

#[allow(clippy::panic)]
impl<const N: usize> Index<usize> for RoundU8Buffer<N> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size, "{index} >= {}", self.size);
        let mut offset = self.head + index;
        if offset >= N {
            offset -= N;
        }
        let Some(val) = self.buf.get(offset) else {
            panic!("expected value at offset {offset} but was empty!");
        };
        val
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
        let mut buf = crate::buf::RoundU8Buffer::<3>::default();
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
