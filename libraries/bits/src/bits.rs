// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Basic Bit Buffer interface
//!

use crate::error::{Error, ErrorKind};
use crate::mutbits::MutBits;
use crate::BitsErrorKind;

cfg_feature_alloc! {
    extern crate alloc;
    use alloc::string::{String, ToString as _};
    use alloc::vec::Vec;
    use alloc::vec;
}

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

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ByteOrder {
    LittleEndian,
    #[default]
    BigEndian,
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
    fn read_be_f32(&mut self) -> Result<f32, Error> {
        Ok(f32::from_bits(self.read_be_u32()?))
    }

    /// Reads a single [`f32`], 4 bytes.  Reversed IEEE754 encoding
    fn read_le_f32(&mut self) -> Result<f32, Error> {
        Ok(f32::from_bits(self.read_le_u32()?))
    }

    /// Reads a single [`f32`], 4 bytes.  Specified byte ordering.
    fn read_f32(&mut self, order: ByteOrder) -> Result<f32, Error> {
        Ok(f32::from_bits(self.read_u32(order)?))
    }

    /// Optionally reads a single [`f32`], 4 bytes.  Standard IEEE754 encoding
    fn next_be_f32(&mut self) -> Result<Option<f32>, Error> {
        Ok(self.next_be_u32()?.map(f32::from_bits))
    }

    /// Optionally reads a single [`f32`], 4 bytes.  Reversed IEEE754 encoding
    fn next_le_f32(&mut self) -> Result<Option<f32>, Error> {
        Ok(self.next_le_u32()?.map(f32::from_bits))
    }

    /// Reads a single [`f64`], 8 bytes.  Standard IEEE754 encoding
    fn read_be_f64(&mut self) -> Result<f64, Error> {
        Ok(f64::from_bits(self.read_be_u64()?))
    }

    /// Reads a single [`f64`], 8 bytes.  Reversed IEEE754 encoding
    fn read_le_f64(&mut self) -> Result<f64, Error> {
        Ok(f64::from_bits(self.read_le_u64()?))
    }

    /// Optionally reads a single [`f64`], 8 bytes.  Standard IEEE754 encoding
    fn next_be_f64(&mut self) -> Result<Option<f64>, Error> {
        Ok(self.next_be_u64()?.map(f64::from_bits))
    }

    /// Optionally reads a single [`f64`], 8 bytes.  Reversed IEEE754 encoding
    fn next_le_f64(&mut self) -> Result<Option<f64>, Error> {
        Ok(self.next_le_u64()?.map(f64::from_bits))
    }

    /// Reads a single [`f64`], 8 bytes.  Specified byte ordering.
    fn read_f64(&mut self, order: ByteOrder) -> Result<f64, Error> {
        Ok(f64::from_bits(self.read_u64(order)?))
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

    /// Reads a single [`i128`] in little-endian order, 8 bytes, LSB first.
    fn read_le_i128(&mut self) -> Result<i128, Error> {
        Ok(self.read_be_i128()?.swap_bytes())
    }

    /// Optionally reads a single [`i64`] in little-endian order, 8 bytes, LSB first.
    fn next_le_i128(&mut self) -> Result<Option<i128>, Error> {
        Ok(self.next_be_u128()?.map(|v| v.swap_bytes() as i128))
    }

    /// Reads a single [`u128`] in little-endian order, 8 bytes, LSB first.
    fn read_le_u128(&mut self) -> Result<u128, Error> {
        Ok(self.read_be_u128()?.swap_bytes())
    }

    /// Optionally reads a single [`i64`] in little-endian order, 8 bytes, LSB first.
    fn next_le_u128(&mut self) -> Result<Option<u128>, Error> {
        Ok(self.next_be_u128()?.map(u128::swap_bytes))
    }

    /// Advances the stream by at most 'len' bytes.  The actual amount of bytes advanced may be
    /// less, and is returned in [`Ok(usize)`]
    fn advance(&mut self, len: usize) -> Result<usize, Error> {
        for _ in 0..len {
            self.read_u8()?;
        }
        Ok(len)
    }

    fn read_u8_blob_into<T: MutBits>(&mut self, into: &mut T) -> Result<(), Error> {
        let size = self.read_u8()? as usize;
        self.read_exact_into(size, into)
    }
    fn read_u16_blob_into<T: MutBits>(&mut self, into: &mut T) -> Result<(), Error> {
        let size = self.read_be_u16()? as usize;
        self.read_exact_into(size, into)
    }
    fn read_u32_blob_into<T: MutBits>(&mut self, into: &mut T) -> Result<(), Error> {
        let size = self.read_be_u32()? as usize;
        self.read_exact_into(size, into)
    }
    fn read_u64_blob_into<T: MutBits>(&mut self, into: &mut T) -> Result<(), Error> {
        let size = self.read_be_u64()? as usize;
        self.read_exact_into(size, into)
    }

    /// reads from the stream until a null (0x0) is encountered into the provided output, does NOT include null.
    fn read_str_nul_terminated_into<T: MutBits>(&mut self, into: &mut T) -> Result<(), Error> {
        while let Some(ch) = self.next_u8()? {
            if ch == 0 {
                break;
            }
            into.write_u8(ch)?;
        }
        Ok(())
    }

    cfg_feature_alloc! {
        /// Reads a sized blob, a series of bytes preceded by a [`u8`] declaring the size.
        fn read_u8_blob(&mut self) -> Result<Vec<u8>, Error> {
            let size = self.read_u8()?;
            self.read_exact_vec(size as usize)
        }

        /// Reads a sized blob, a series of bytes preceded by a [`u16`] declaring the size.
        fn read_be_u16_blob(&mut self) -> Result<Vec<u8>, Error> {
            let size = self.read_be_u16()?;
            self.read_exact_vec(size as usize)
        }

        /// Reads a sized blob, a series of bytes preceded by a [`u16`] declaring the size.
        fn read_le_u16_blob(&mut self) -> Result<Vec<u8>, Error> {
            let size = self.read_le_u16()?;
            self.read_exact_vec(size as usize)
        }
        /// Reads a sized blob, a series of bytes preceded by a [`u32`] declaring the size.
        fn read_be_u32_blob(&mut self) -> Result<Vec<u8>, Error> {
            let size = self.read_be_u32()?;
            self.read_exact_vec(size as usize)
        }
        /// Reads a sized blob, a series of bytes preceded by a [`u32`] declaring the size.
        fn read_le_u32_blob(&mut self) -> Result<Vec<u8>, Error> {
            let size = self.read_le_u32()?;
            self.read_exact_vec(size as usize)
        }

        /// Reads a sized blob, a series of bytes preceded by a [`u64`] declaring the size.
        fn read_be_u64_blob(&mut self) -> Result<Vec<u8>, Error> {
            let size = self.read_be_u64()?;
            self.read_exact_vec(size as usize)
        }
        /// Reads a sized blob, a series of bytes preceded by a [`u64`] declaring the size.
        fn read_le_u64_blob(&mut self) -> Result<Vec<u8>, Error> {
            let size = self.read_le_u64()?;
            self.read_exact_vec(size as usize)
        }

        /// Reads the specified amount of bytes into a [`Vec<u8>`] and returns it
        fn read_exact_vec(&mut self, size: usize) -> Result<alloc::vec::Vec<u8>, Error> {
            let mut buf: alloc::vec::Vec<u8> = alloc::vec::Vec::with_capacity(size);
            self.read_exact_into(size, &mut buf)?;
            Ok(buf)
        }

        /// Reads the entire stream into a UTF-8 String, dropping all other bytes.
        fn read_all_str_lossy(&mut self) -> Result<alloc::string::String, Error> {
            Ok(String::from_utf8_lossy(&self.read_all_vec()?).to_string())
        }

        /// Reads the specified amount of bytes into a UTF-8 String, dropping all other bytes.
        fn read_str_sized_lossy(&mut self, len: usize) -> Result<String, Error> {
            Ok(String::from_utf8_lossy(&self.read_exact_vec(len)?).to_string())
        }

        /// Reads a string from the stream, terminated by a null byte.  Does NOT include the null byte.
        fn read_str_null_terminated(&mut self) -> Result<String, Error> {
            let mut out = String::new();
            while let Some(ch) = self.next_u8()? {
                if ch == 0 {
                    break;
                }
                out.push(ch as char);
            }
            Ok(out)
        }

        /// Reads to the end of the stream and returns the data as a [`Vec<u8>`]
        fn read_all_vec(&mut self) -> Result<alloc::vec::Vec<u8>, Error> {
            let mut out: alloc::vec::Vec<u8> = vec![];
            self.read_all_into(&mut out)?;
            Ok(out)
        }

        ///
        /// Reads from the input stream until:
        /// 1. The byte stream represented by 'search' has been found or
        /// 2. The input stream returns 0 bytes read (or errors out)
        /// It returns all bytes read in the interim
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
        /// Reads until the next `\n` character, ignoring any `\r` characters along
        /// the way.
        fn read_line_vec(&mut self) -> Result<Option<alloc::vec::Vec<u8>>, Error> {
            let mut out = Vec::new();
            while let Some(val) = self.next_u8()? {
                if val == b'\r' {
                    continue;
                }
                else if val == b'\n' {
                    return Ok(Some(out));
                }
                out.push(val);
            }
            if out.is_empty() {
                return Ok(None)
            }
            Ok(Some(out))
        }

        ///
        /// Reads until the next `\n` character, then calls [`String::from_utf8_lossy`].
        fn read_line_str_lossy(&mut self) -> Result<Option<alloc::string::String>, Error> {
            let Some(data) = self.read_line_vec()? else {
                return Ok(None);
            };
            Ok(Some(String::from_utf8_lossy(&data).to_string()))
        }

        ///
        /// Reads until the next `\n` character, then calls [`String::from_utf8`]
        fn read_line_str(&mut self) -> Result<Option<alloc::string::String>, Error> {
            let Some(data) = self.read_line_vec()? else {
                return Ok(None);
            };
            Ok(Some(String::from_utf8(data)?))
        }

        ///
        /// Consumes data from the input stream until:
        /// 1. The byte stream represented by 'search' has been found or
        /// 2. The input reader returns 0 bytes read (or errors out)
        ///
        /// Note: The input stream position is left JUST AFTER the found search string.
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
        fn read_str_u32_blob(&mut self) -> Result<String, Error> {
            let len = self.read_be_u32()?;
            self.read_str_sized_lossy(len as usize)
        }
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

    /// Reads to the end of the stream, and writes it into the specified target.
    fn read_all_into<T: MutBits>(&mut self, into: &mut T) -> Result<(), Error> {
        while let Some(val) = self.next_u8()? {
            into.write_u8(val)?;
        }
        Ok(())
    }

    /// Fills the provided buffer
    fn read_filling<T: MutBits>(&mut self, into: &mut T) -> Result<(), Error> {
        while let Some(val) = self.next_u8()? {
            if let Err(_e) = into.write_u8(val) {
                break;
            }
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

    /// Reads a single [`u16`] in the specified order order, 2 bytes.
    fn read_u16(&mut self, order: ByteOrder) -> Result<u16, Error> {
        match order {
            ByteOrder::LittleEndian => self.read_le_u16(),
            ByteOrder::BigEndian => self.read_be_u16(),
        }
    }
    /// Reads a single [`u32`] in the specified order order, 4 bytes.
    fn read_u32(&mut self, order: ByteOrder) -> Result<u32, Error> {
        match order {
            ByteOrder::LittleEndian => self.read_le_u32(),
            ByteOrder::BigEndian => self.read_be_u32(),
        }
    }
    /// Reads a single [`u64`] in the specified order order, 8 bytes.
    fn read_u64(&mut self, order: ByteOrder) -> Result<u64, Error> {
        match order {
            ByteOrder::LittleEndian => self.read_le_u64(),
            ByteOrder::BigEndian => self.read_be_u64(),
        }
    }
    /// Reads a single [`u128`] in the specified order order, 16 bytes.
    fn read_u128(&mut self, order: ByteOrder) -> Result<u128, Error> {
        match order {
            ByteOrder::LittleEndian => self.read_le_u128(),
            ByteOrder::BigEndian => self.read_be_u128(),
        }
    }
    /// Reads a single [`i16`] in the specified order order, 2 bytes.
    fn read_i16(&mut self, order: ByteOrder) -> Result<i16, Error> {
        match order {
            ByteOrder::LittleEndian => self.read_le_i16(),
            ByteOrder::BigEndian => self.read_be_i16(),
        }
    }
    /// Reads a single [`i32`] in the specified order order, 4 bytes.
    fn read_i32(&mut self, order: ByteOrder) -> Result<i32, Error> {
        match order {
            ByteOrder::LittleEndian => self.read_le_i32(),
            ByteOrder::BigEndian => self.read_be_i32(),
        }
    }
    /// Reads a single [`i64`] in the specified order order, 4 bytes.
    fn read_i64(&mut self, order: ByteOrder) -> Result<i64, Error> {
        match order {
            ByteOrder::LittleEndian => self.read_le_i64(),
            ByteOrder::BigEndian => self.read_be_i64(),
        }
    }
    /// Reads a single [`i128`] in the specified order order, 16 bytes.
    fn read_i128(&mut self, order: ByteOrder) -> Result<i128, Error> {
        match order {
            ByteOrder::LittleEndian => self.read_le_i128(),
            ByteOrder::BigEndian => self.read_be_i128(),
        }
    }

    /// Some implementations may be able to return the size of the remaining data
    /// in the buffer.
    fn remaining(&self) -> Option<usize> {
        None
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

    fn remaining(&self) -> Option<usize> {
        Some(self.len())
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

    fn remaining(&self) -> Option<usize> {
        Some(self.len())
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
    data.read_be_f32()
}
/// Calls [`Bits::read_f64()`].  Provided for type-elusion purposes.
pub fn read_f64<T: Bits>(mut data: T) -> Result<f64, Error> {
    data.read_be_f64()
}

///
/// This struct wraps a provided borrowed static array in a MutBits impl.  Operates
/// like a slice, walking through the array filling it up.
pub struct MutBitsArray<'a, const N: usize> {
    arr: &'a mut [u8; N],
    pos: usize,
}
impl<'a, const N: usize> MutBitsArray<'a, N> {
    /// Wraps the provided array, providing a [`MutBits`] impl
    pub fn new(arr: &'a mut [u8; N]) -> Self {
        Self { arr, pos: 0 }
    }
    /// Returns the current length of the written data.
    pub fn len(&self) -> usize {
        self.pos
    }
    /// Has anything been written?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Resets the writing position to zero. Does NOT clear the data.
    pub fn reset(&mut self) {
        self.pos = 0;
    }
    /// Resets the writing position to zero and clears the data back to zeros.
    /// Same as calling `fill(0)`
    pub fn zero(&mut self) {
        self.fill(0)
    }
    /// Fills the array with the value, resets the writing position back to zero.
    pub fn fill(&mut self, val: u8) {
        self.arr.fill(val);
        self.pos = 0;
    }
    /// Get an Reader from this array, starts at the beginning and runs until the
    /// high water 'pos' mark returning a view into JUST the data written.
    pub fn reader(&'a self) -> BitsArray<'a, N> {
        BitsArray::new_limited(self.arr, self.pos)
    }
}
impl<'a, const N: usize> From<&'a mut [u8; N]> for MutBitsArray<'a, N> {
    fn from(arr: &'a mut [u8; N]) -> Self {
        MutBitsArray { arr, pos: 0 }
    }
}
impl<const N: usize> MutBits for MutBitsArray<'_, N> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        if let Some(v) = self.arr.get_mut(self.pos) {
            *v = val;
            self.pos += 1;
            return Ok(());
        }
        Err(BitsErrorKind::UnexpectedEof.into())
    }
}
///
/// This struct wraps a provided borrowed static array in a MutBits impl.  Operates
/// like a slice, walking through the array filling it up.
pub struct BitsArray<'a, const N: usize> {
    arr: &'a [u8; N],
    pos: usize,
    max_len: usize,
}
impl<'a, const N: usize> BitsArray<'a, N> {
    /// A new view into the backed array, limited only by the length of the backed array
    pub fn new(arr: &'a [u8; N]) -> Self {
        Self {
            max_len: arr.len(),
            pos: 0,
            arr,
        }
    }
    /// A new view into the backed array, limited by the provided maximum length.
    pub fn new_limited(arr: &'a [u8; N], max_len: usize) -> Self {
        Self {
            arr,
            max_len,
            pos: 0,
        }
    }
    /// Resets the reading position back to the start of the array.
    pub fn reset(&mut self) {
        self.pos = 0;
    }
}
impl<'a, const N: usize> From<&'a [u8; N]> for BitsArray<'a, N> {
    fn from(value: &'a [u8; N]) -> Self {
        Self::new(value)
    }
}
impl<const N: usize> Bits for BitsArray<'_, N> {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        if self.pos >= self.max_len {
            return Ok(None);
        }
        let v = self.arr.get(self.pos).copied();
        self.pos += 1;
        Ok(v)
    }

    fn remaining(&self) -> Option<usize> {
        Some(self.max_len.saturating_sub(self.pos))
    }
}
