// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Basic Bit Buffer interface
//!

use crate::error::{Error, ErrorKind};
use crate::mutbits::MutBits;

#[cfg(feature = "alloc")]
extern crate alloc;

macro_rules! maybe_next_u8 {
    ($self:ident,$prev:expr) => {{
        let Some(b) = $self.next_u8()? else {
            return Ok(Some($prev));
        };
        b
    }};
}
macro_rules! next_and_shift {
    ($self:ident,$ty:ty,$prev:expr) => {{
        let a = maybe_next_u8!($self, $prev);
        $prev <<= 8;
        $prev |= a as $ty;
    }};
}

///
/// Read methods for the primitive types
///
pub trait Bits {
    /// Reads a single [`u8`]
    fn read_u8(&mut self) -> Result<u8, Error> {
        let Some(val) = self.next_u8()? else {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        };
        Ok(val)
    }

    /// Optionally returns a single [`u8`]
    fn next_u8(&mut self) -> Result<Option<u8>, Error>;

    /// Reads a single [`i8`]
    fn read_i8(&mut self) -> Result<i8, Error> {
        Ok(self.read_u8()? as i8)
    }
    /// Optionally returns a single [`i8`]
    fn next_i8(&mut self) -> Result<Option<i8>, Error> {
        Ok(self.next_u8()?.map(|v| v as i8))
    }

    /// Reads a single bool (u8), returning true if 1, false if 0, or InvalidInput if anything else.
    fn read_bool(&mut self) -> Result<bool, Error> {
        let Some(val) = self.next_bool()? else {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        };
        Ok(val)
    }

    /// Reads a single bool (u8), returning true if 1, false if 0, or InvalidInput if anything else.
    fn next_bool(&mut self) -> Result<Option<bool>, Error> {
        let val = self.next_u8()?;
        let Some(val) = val else { return Ok(None) };
        if val == 0 {
            Ok(Some(false))
        } else if val == 1 {
            Ok(Some(true))
        } else {
            Err(ErrorKind::InvalidInput.into())
        }
    }

    /// Reads 1, 2, 3, or 4 bytes to construct a UTF-8 charpoint.
    fn read_be_utf8_char(&mut self) -> Result<char, Error> {
        Ok(crate::utf::read_be_utf8_char(self)?.0)
    }

    /// Reads a single [`u16`] in big-endian order, 2 bytes, MSB first.
    fn read_be_u16(&mut self) -> Result<u16, Error> {
        let Some(ret) = self.next_be_u16()? else {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        };
        Ok(ret)
    }

    /// Reads a single [`u16`] in little-endian order, 2 bytes, LSB first.
    fn read_le_u16(&mut self) -> Result<u16, Error> {
        Ok(self.read_be_u16()?.swap_bytes())
    }

    /// Optionally reads a single [`u16`] in big-endian order, 2 bytes, MSB first.
    fn next_be_u16(&mut self) -> Result<Option<u16>, Error> {
        let Some(a) = self.next_u8()? else {
            return Ok(None);
        };
        let Some(b) = self.next_u8()? else {
            return Ok(Some(a as u16));
        };
        let out = ((a as u16) << 8) | (b as u16);
        Ok(Some(out))
    }

    /// Optionally reads a single [`u16`] in little-endian order, 2 bytes, LSB first.
    fn next_le_u16(&mut self) -> Result<Option<u16>, Error> {
        Ok(self.next_be_u16()?.map(u16::swap_bytes))
    }

    /// Reads a single [`u32`] in big-endian order, 4 bytes, MSB first.
    fn read_be_u32(&mut self) -> Result<u32, Error> {
        let Some(ret) = self.next_be_u32()? else {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        };
        Ok(ret)
    }

    /// Reads a single [`u32`] in little-endian order, 4 bytes, LSB first.
    fn read_le_u32(&mut self) -> Result<u32, Error> {
        Ok(self.read_be_u32()?.swap_bytes())
    }

    /// Optionally reads a single [`u32`] in big-endian order, 4 bytes, MSB first.
    fn next_be_u32(&mut self) -> Result<Option<u32>, Error> {
        let Some(a) = self.next_u8()? else {
            return Ok(None);
        };
        let mut out: u32 = ((a as u32) << 8) | maybe_next_u8!(self, a as u32) as u32;
        next_and_shift!(self, u32, out);
        next_and_shift!(self, u32, out);

        Ok(Some(out))
    }

    /// Optionally reads a single [`u32`] in little-endian order, 4 bytes, LSB first.
    fn next_le_u32(&mut self) -> Result<Option<u32>, Error> {
        Ok(self.next_be_u32()?.map(u32::swap_bytes))
    }

    /// Reads a single [`u64`] in big-endian order, 8 bytes, MSB first.
    fn read_be_u64(&mut self) -> Result<u64, Error> {
        let Some(ret) = self.next_be_u64()? else {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        };
        Ok(ret)
    }

    /// Reads a single [`u64`] in big-endian order, 8 bytes, MSB first.
    fn read_le_u64(&mut self) -> Result<u64, Error> {
        let Some(ret) = self.next_be_u64()? else {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        };
        Ok(ret.swap_bytes())
    }

    /// Optionally reads a single [`u64`] in big-endian order, 8 bytes, MSB first.
    fn next_be_u64(&mut self) -> Result<Option<u64>, Error> {
        let Some(a) = self.next_u8()? else {
            return Ok(None);
        };
        let mut out: u64 = ((a as u64) << 8) | maybe_next_u8!(self, a as u64) as u64;
        next_and_shift!(self, u64, out);
        next_and_shift!(self, u64, out);
        next_and_shift!(self, u64, out);
        next_and_shift!(self, u64, out);
        next_and_shift!(self, u64, out);
        next_and_shift!(self, u64, out);

        Ok(Some(out))
    }

    /// Optionally reads a single [`u64`] in little-endian order, 4 bytes, LSB first.
    fn next_le_u64(&mut self) -> Result<Option<u64>, Error> {
        Ok(self.next_be_u64()?.map(u64::swap_bytes))
    }

    /// Reads a single [`u128`] in big-endian order, 16 bytes, MSB first.
    fn read_be_u128(&mut self) -> Result<u128, Error> {
        let Some(ret) = self.next_be_u128()? else {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        };
        Ok(ret)
    }

    /// Optionally reads a single [`u128`] in big-endian order, 16 bytes, MSB first.
    fn next_be_u128(&mut self) -> Result<Option<u128>, Error> {
        let Some(a) = self.next_u8()? else {
            return Ok(None);
        };
        let mut out: u128 = ((a as u128) << 8) | maybe_next_u8!(self, a as u128) as u128;
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);
        next_and_shift!(self, u128, out);

        Ok(Some(out))
    }

    /// Reads a single [`i128`] in big-endian order, 16 bytes, MSB first.
    fn read_be_i128(&mut self) -> Result<i128, Error> {
        Ok(self.read_be_u128()? as i128)
    }
    /// Optionally reads a single [`i128`] in big-endian order, 16 bytes, MSB first.
    fn next_be_i128(&mut self) -> Result<Option<i128>, Error> {
        let Some(val) = self.next_be_u128()? else {
            return Ok(None);
        };
        Ok(Some(val as i128))
    }

    /// Reads a single [`f32`], 4 bytes.  Standard IEEE754 encoding
    fn read_f32(&mut self) -> Result<f32, Error> {
        Ok(f32::from_bits(self.read_be_u32()?))
    }

    /// Optionally reads a single [`f32`], 4 bytes.  Standard IEEE754 encoding
    fn next_f32(&mut self) -> Result<Option<f32>, Error> {
        Ok(self.next_be_u32()?.map(f32::from_bits))
    }

    /// Reads a single [`f64`], 8 bytes.  Standard IEEE754 encoding
    fn read_f64(&mut self) -> Result<f64, Error> {
        Ok(f64::from_bits(self.read_be_u64()?))
    }

    /// Optionally reads a single [`f64`], 8 bytes.  Standard IEEE754 encoding
    fn next_f64(&mut self) -> Result<Option<f64>, Error> {
        Ok(self.next_be_u64()?.map(f64::from_bits))
    }

    /// Reads a single [`i16`] in big-endian order, 2 bytes, MSB first.
    fn read_be_i16(&mut self) -> Result<i16, Error> {
        Ok(self.read_be_u16()? as i16)
    }

    /// Reads a single [`i16`] in little-endian order, 2 bytes, LSB first.
    fn read_le_i16(&mut self) -> Result<i16, Error> {
        Ok(self.read_be_u16()?.swap_bytes() as i16)
    }

    /// Optionally reads a single [`i16`] in big-endian order, 2 bytes, MSB first.
    fn next_be_i16(&mut self) -> Result<Option<i16>, Error> {
        Ok(self.next_be_u16()?.map(|v| v as i16))
    }

    /// Optionally reads a single [`i16`] in little-endian order, 2 bytes, LSB first.
    fn next_le_i16(&mut self) -> Result<Option<i16>, Error> {
        Ok(self.next_be_u16()?.map(|v| v.swap_bytes() as i16))
    }

    /// Reads a single [`i32`] in big-endian order, 4 bytes, MSB first.
    fn read_be_i32(&mut self) -> Result<i32, Error> {
        Ok(self.read_be_u32()? as i32)
    }

    /// Reads a single [`i32`] in little-endian order, 4 bytes, LSB first.
    fn read_le_i32(&mut self) -> Result<i32, Error> {
        Ok(self.read_be_u32()?.swap_bytes() as i32)
    }

    /// Optionally reads a single [`i32`] in big-endian order, 4 bytes, MSB first.
    fn next_be_i32(&mut self) -> Result<Option<i32>, Error> {
        Ok(self.next_be_u32()?.map(|v| v as i32))
    }

    /// Optionally reads a single [`i32`] in little-endian order, 4 bytes,LSB first.
    fn next_le_i32(&mut self) -> Result<Option<i32>, Error> {
        Ok(self.next_be_u32()?.map(|v| v.swap_bytes() as i32))
    }

    /// Reads a single [`i64`] in big-endian order, 8 bytes, MSB first.
    fn read_be_i64(&mut self) -> Result<i64, Error> {
        Ok(self.read_be_u64()? as i64)
    }

    /// Reads a single [`i64`] in little-endian order, 8 bytes, LSB first.
    fn read_le_i64(&mut self) -> Result<i64, Error> {
        Ok(self.read_be_u64()?.swap_bytes() as i64)
    }

    /// Optionally reads a single [`i64`] in big-endian order, 8 bytes, MSB first.
    fn next_be_i64(&mut self) -> Result<Option<i64>, Error> {
        Ok(self.next_be_u64()?.map(|v| v as i64))
    }

    /// Optionally reads a single [`i64`] in little-endian order, 8 bytes, LSB first.
    fn next_le_i64(&mut self) -> Result<Option<i64>, Error> {
        Ok(self.next_be_u64()?.map(|v| v.swap_bytes() as i64))
    }

    /// Advances the stream by at most 'len' bytes.  The actual amount of bytes advanced may be
    /// less, and is returned in [`Ok(usize)`]
    fn advance(&mut self, len: usize) -> Result<usize, Error> {
        for _ in 0..len {
            self.read_u8()?;
        }
        Ok(len)
    }

    /// Reads a sized blob, a series of bytes preceded by a [`u8`] declaring the size.
    #[cfg(feature = "alloc")]
    fn read_u8_blob(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.read_u8()?;
        self.read_exact_vec(size as usize)
    }

    /// Reads a sized blob, a series of bytes preceded by a [`u16`] declaring the size.
    #[cfg(feature = "alloc")]
    fn read_be_u16_blob(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.read_be_u16()?;
        self.read_exact_vec(size as usize)
    }

    /// Reads a sized blob, a series of bytes preceded by a [`u16`] declaring the size.
    #[cfg(feature = "alloc")]
    fn read_le_u16_blob(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.read_le_u16()?;
        self.read_exact_vec(size as usize)
    }

    /// Reads a sized blob, a series of bytes preceded by a [`u32`] declaring the size.
    #[cfg(feature = "alloc")]
    fn read_be_u32_blob(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.read_be_u32()?;
        self.read_exact_vec(size as usize)
    }
    /// Reads a sized blob, a series of bytes preceded by a [`u32`] declaring the size.
    #[cfg(feature = "alloc")]
    fn read_le_u32_blob(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.read_le_u32()?;
        self.read_exact_vec(size as usize)
    }

    /// Reads a sized blob, a series of bytes preceded by a [`u64`] declaring the size.
    #[cfg(feature = "alloc")]
    fn read_be_u64_blob(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.read_be_u64()?;
        self.read_exact_vec(size as usize)
    }
    /// Reads a sized blob, a series of bytes preceded by a [`u64`] declaring the size.
    #[cfg(feature = "alloc")]
    fn read_le_u64_blob(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.read_le_u64()?;
        self.read_exact_vec(size as usize)
    }

    /// Reads the specified amount of bytes into a [`Vec<u8>`] and returns it
    #[cfg(feature = "alloc")]
    fn read_exact_vec(&mut self, size: usize) -> Result<alloc::vec::Vec<u8>, Error> {
        let mut buf: alloc::vec::Vec<u8> = alloc::vec::Vec::with_capacity(size);
        self.read_exact_into(size, &mut buf)?;
        Ok(buf)
    }

    /// Reads the specified amount of bytes into a stack-allocated array.
    fn read_exact<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let mut buf = [0u8; N];
        self.read_exact_into(N, &mut buf.as_mut())?;
        Ok(buf)
    }

    /// Reads the specified amount of bytes into the specified target.
    fn read_exact_into<T: MutBits>(&mut self, size: usize, into: &mut T) -> Result<(), Error> {
        for _i in 0..size {
            into.write_u8(self.read_u8()?)?;
        }
        Ok(())
    }

    /// Reads the entire stream into a UTF-8 String, dropping all other bytes.
    #[cfg(feature = "alloc")]
    fn read_all_str_lossy(&mut self) -> Result<alloc::string::String, Error> {
        Ok(String::from_utf8_lossy(&self.read_all_vec()?).to_string())
    }

    /// Reads the specified amount of bytes into a UTF-8 String, dropping all other bytes.
    #[cfg(feature = "alloc")]
    fn read_str_sized_lossy(&mut self, len: usize) -> Result<String, Error> {
        Ok(String::from_utf8_lossy(&self.read_exact_vec(len)?).to_string())
    }

    /// Reads to the end of the stream and returns the data as a [`Vec<u8>`]
    #[cfg(feature = "alloc")]
    fn read_all_vec(&mut self) -> Result<alloc::vec::Vec<u8>, Error> {
        let mut out: alloc::vec::Vec<u8> = vec![];
        self.read_all_into(&mut out)?;
        Ok(out)
    }

    /// Reads to the end of the stream, and writes it into the specified target.
    fn read_all_into<T: MutBits>(&mut self, into: &mut T) -> Result<(), Error> {
        while let Some(val) = self.next_u8()? {
            into.write_u8(val)?;
        }
        Ok(())
    }

    /// Reads some subset of the data into the specified target.
    fn read_some_into<T: MutBits>(&mut self, buf: &mut T) -> Result<usize, Error> {
        let mut read = 0;
        for _ in 0..4096 {
            let Some(val) = self.next_u8()? else {
                return Ok(read);
            };
            buf.write_u8(val)?;
            read += 1;
        }
        Ok(read)
    }

    ///
    /// Reads from the input stream until:
    /// 1. The byte stream represented by 'search' has been found or
    /// 2. The input stream returns 0 bytes read (or errors out)
    /// It returns all bytes read in the interim
    #[cfg(feature = "alloc")]
    fn read_until(&mut self, search: &[u8]) -> Result<alloc::vec::Vec<u8>, Error> {
        let mut ringbuf: alloc::collections::VecDeque<u8> =
            alloc::collections::VecDeque::with_capacity(search.len());

        let mut out = Vec::new();
        loop {
            if ringbuf.iter().eq(search) {
                return Ok(out);
            }

            let Some(val) = self.next_u8()? else {
                return Ok(out);
            };

            if ringbuf.len() == search.len() {
                if let Some(val) = ringbuf.pop_front() {
                    out.push(val);
                }
            }
            ringbuf.push_back(val);
        }
    }

    ///
    /// Consumes data from the input stream until:
    /// 1. The byte stream represented by 'search' has been found or
    /// 2. The input reader returns 0 bytes read (or errors out)
    ///
    /// Note: The input stream position is left JUST AFTER the found search string.
    #[cfg(feature = "alloc")]
    fn consume_until(&mut self, search: &[u8]) -> Result<(), Error> {
        let mut ringbuf: alloc::collections::VecDeque<u8> =
            alloc::collections::VecDeque::with_capacity(search.len());
        self.read_exact_into(search.len(), &mut ringbuf)?;

        loop {
            if ringbuf.iter().eq(search) {
                return Ok(());
            }

            let Some(val) = self.next_u8()? else {
                return Ok(());
            };

            ringbuf.pop_front();
            ringbuf.push_back(val);
        }
    }

    ///
    /// Reads a specific sized string from the stream, a string prefixed by a
    /// 4-byte big-endian length.
    #[cfg(feature = "alloc")]
    fn read_str_u32_blob(&mut self) -> Result<String, Error> {
        let len = self.read_be_u32()?;
        self.read_str_sized_lossy(len as usize)
    }
}

#[allow(unused_macros)]
macro_rules! absorb_eof {
    ($self:ident, $buf:ident) => {
        if let Err(e) = $self.read_exact(&mut $buf) {
            if e.kind() == ErrorKind::UnexpectedEof {
                return Ok(None);
            }
            return Err(e);
        }
    };
}

impl Bits for &[u8] {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        let Some((first, rest)) = self.split_first() else {
            return Ok(None);
        };
        *self = rest;
        Ok(Some(*first))
    }

    fn read_some_into<T: MutBits>(&mut self, into: &mut T) -> Result<usize, Error> {
        Ok(into.write_some_bytes(self))
    }
}

impl Bits for &mut [u8] {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        if let Some((first, rem)) = core::mem::take(self).split_first_mut() {
            *self = rem;
            return Ok(Some(*first));
        }
        Ok(None)
    }

    fn read_some_into<T: MutBits>(&mut self, into: &mut T) -> Result<usize, Error> {
        Ok(into.write_some_bytes(self))
    }
}

/// Calls [`Bits::read_be_u32()`].  Provided for type-elusion purposes.
pub fn read_be_u32<T: Bits>(mut data: T) -> Result<u32, Error> {
    data.read_be_u32()
}
/// Calls [`Bits::read_be_u64()`].  Provided for type-elusion purposes.
pub fn read_be_u64<T: Bits>(mut data: T) -> Result<u64, Error> {
    data.read_be_u64()
}
/// Calls [`Bits::read_f32()`].  Provided for type-elusion purposes.
pub fn read_f32<T: Bits>(mut data: T) -> Result<f32, Error> {
    data.read_f32()
}
/// Calls [`Bits::read_f64()`].  Provided for type-elusion purposes.
pub fn read_f64<T: Bits>(mut data: T) -> Result<f64, Error> {
    data.read_f64()
}
