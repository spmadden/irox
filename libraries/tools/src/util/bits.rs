// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Basic Bit Buffer interface
//!

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::fmt::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::fmt::Arguments;

#[cfg(not(feature = "std"))]
pub use error::*;

#[cfg(feature = "std")]
pub type Error = std::io::Error;
#[cfg(feature = "std")]
pub type ErrorKind = std::io::ErrorKind;

#[cfg(not(feature = "std"))]
mod error {
    pub type Error = BitsError;
    pub type ErrorKind = BitsErrorKind;

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct BitsError {
        kind: BitsErrorKind,
        msg: &'static str,
    }
    impl BitsError {
        pub fn new(kind: BitsErrorKind, msg: &'static str) -> Self {
            BitsError { kind, msg }
        }
    }
    impl From<BitsErrorKind> for BitsError {
        fn from(kind: BitsErrorKind) -> Self {
            BitsError {
                kind,
                msg: match kind {
                    BitsErrorKind::InvalidData => "Invalid Data",
                    BitsErrorKind::UnexpectedEof => "Unexpected EOF",
                    BitsErrorKind::FormatError => "Unspecified Formatting Error",
                    BitsErrorKind::OutOfMemory => "Out of Memory",
                },
            }
        }
    }

    impl From<BitsError> for core::fmt::Error {
        fn from(_kind: BitsError) -> Self {
            core::fmt::Error
        }
    }

    impl From<core::fmt::Error> for BitsError {
        fn from(_value: core::fmt::Error) -> Self {
            BitsErrorKind::FormatError.into()
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum BitsErrorKind {
        InvalidData,
        UnexpectedEof,
        FormatError,
        OutOfMemory,
    }
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

    /// Reads a single [`u16`] in big-endian order, 2 bytes, MSB first.
    fn read_be_u16(&mut self) -> Result<u16, Error> {
        let Some(ret) = self.next_be_u16()? else {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        };
        Ok(ret)
    }
    
    /// Reads a single [`u16`] in little-endian order, 2 bytes, LSB first.
    fn read_le_u16(&mut self)-> Result<u16, Error> {
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
    fn read_u8_blob(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.read_u8()?;
        self.read_exact_vec(size as usize)
    }

    /// Reads a sized blob, a series of bytes preceded by a [`u16`] declaring the size.
    fn read_u16_blob(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.read_be_u16()?;
        self.read_exact_vec(size as usize)
    }

    /// Reads a sized blob, a series of bytes preceded by a [`u32`] declaring the size.
    fn read_u32_blob(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.read_be_u32()?;
        self.read_exact_vec(size as usize)
    }

    /// Reads a sized blob, a series of bytes preceded by a [`u64`] declaring the size.
    fn read_u64_blob(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.read_be_u64()?;
        self.read_exact_vec(size as usize)
    }

    /// Reads the specified amount of bytes into a [`Vec<u8>`] and returns it
    fn read_exact_vec(&mut self, size: usize) -> Result<Vec<u8>, Error> {
        let mut buf: Vec<u8> = vec![0; size];
        self.read_exact_into(size, &mut buf)?;
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
    fn read_all_str_lossy(&mut self) -> Result<String, Error> {
        Ok(String::from_utf8_lossy(&self.read_all_vec()?).to_string())
    }
    
    /// Reads the specified amount of bytes into a UTF-8 String, dropping all other bytes.
    fn read_str_sized_lossy(&mut self, len: usize) -> Result<String, Error> {
        Ok(String::from_utf8_lossy(&self.read_exact_vec(len)?).to_string())
    }

    /// Reads to the end of the stream and returns the data as a [`Vec<u8>`]
    fn read_all_vec(&mut self) -> Result<Vec<u8>, Error> {
        let mut out: Vec<u8> = vec![];
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
    fn read_until(&mut self, search: &[u8]) -> Result<Vec<u8>, Error> {
        let mut ringbuf: VecDeque<u8> = VecDeque::with_capacity(search.len());

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
    fn consume_until(&mut self, search: &[u8]) -> Result<(), Error> {
        let mut ringbuf: VecDeque<u8> = VecDeque::with_capacity(search.len());
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

macro_rules! impl_bits_pop {
    ($($ty:tt)+) => {
        impl Bits for $($ty)+ {
            fn next_u8(&mut self) -> Result<Option<u8>, Error> {
                Ok(self.pop().map(|v| v as u8))
            }

            fn read_some_into<T: MutBits>(&mut self, into: &mut T) -> Result<usize, Error> {
                Ok(into.write_some_bytes(self.as_ref()))
            }
        }
    };
}

impl_bits_pop!(String);
impl_bits_pop!(&mut String);
impl_bits_pop!(Vec<u8>);
impl_bits_pop!(&mut Vec<u8>);
macro_rules! impl_bits_vecdeque {
    ($($ty:tt)+) => {
        impl Bits for $($ty)+ {
            fn next_u8(&mut self) -> Result<Option<u8>, Error> {
                Ok(self.pop_front())
            }

            fn read_some_into<T: MutBits>(&mut self, into: &mut T) -> Result<usize, Error> {
                let mut wrote = 0;
                while let Some(val) = self.pop_front() {
                    let Ok(()) = into.write_u8(val) else {
                        return Ok(wrote);
                    };
                    wrote += 1;
                }
                Ok(wrote)
            }
        }
    };
}
impl_bits_vecdeque!(VecDeque<u8>);
impl_bits_vecdeque!(&mut VecDeque<u8>);

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

#[cfg(feature = "std")]
impl Bits for std::fs::File {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        use std::io::Read;
        let mut buf: [u8; 1] = [0];
        let read = self.read(&mut buf)?;
        if read == 1 {
            return Ok(Some(buf[0]));
        }
        Ok(None)
    }
}

///
/// Write methods for the primitive types
pub trait MutBits {
    /// Writes a single [`u8`]
    fn write_u8(&mut self, val: u8) -> Result<(), Error>;
    /// Writes a single [`u16`] in big-endian order, 2 bytes, MSB first.
    fn write_be_u16(&mut self, val: u16) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`u16`] in little-endian order, 2 bytes, LSB first.
    fn write_le_u16(&mut self, val: u16) -> Result<(), Error> {
        self.write_all_bytes(&val.swap_bytes().to_be_bytes())
    }
    /// Writes a single [`u32`] in big-endian order, 4 bytes, MSB first.
    fn write_be_u32(&mut self, val: u32) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`u32`] in little-endian order, 4 bytes, LSB first.
    fn write_le_u32(&mut self, val: u32) -> Result<(), Error> {
        self.write_all_bytes(&val.swap_bytes().to_be_bytes())
    }
    /// Writes a single [`u64`] in big-endian order, 8 bytes, MSB first.
    fn write_be_u64(&mut self, val: u64) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`u64`] in little-endian order, 8 bytes, LSB first.
    fn write_le_u64(&mut self, val: u64) -> Result<(), Error> {
        self.write_all_bytes(&val.swap_bytes().to_be_bytes())
    }
    /// Writes a single [`u128`] in big-endian order, 16 bytes, MSB first.
    fn write_be_u128(&mut self, val: u128) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`u128`] in little-endian order, 16 bytes, LSB first.
    fn write_le_u128(&mut self, val: u128) -> Result<(), Error> {
        self.write_all_bytes(&val.swap_bytes().to_be_bytes())
    }

    /// Writes a single [`f32`] in standard IEEE754 format, 4 bytes
    fn write_f32(&mut self, val: f32) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`u16`] in standard IEEE754 format, 8 bytes
    fn write_f64(&mut self, val: f64) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }

    /// Writes a single [`i16`] in big-endian order, 2 bytes, MSB first.
    fn write_be_i16(&mut self, val: i16) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`i16`] in little-endian order, 2 bytes, LSB first.
    fn write_le_i16(&mut self, val: i16) -> Result<(), Error> {
        self.write_all_bytes(&val.swap_bytes().to_be_bytes())
    }
    /// Writes a single [`i32`] in big-endian order, 4 bytes, MSB first.
    fn write_be_i32(&mut self, val: i32) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`i32`] in little-endian order, 4 bytes, LSB first.
    fn write_le_i32(&mut self, val: i32) -> Result<(), Error> {
        self.write_all_bytes(&val.swap_bytes().to_be_bytes())
    }
    /// Writes a single [`i64`] in big-endian order, 8 bytes, MSB first.
    fn write_be_i64(&mut self, val: i64) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`i64`] in little-endian order, 8 bytes, LSB first.
    fn write_le_i64(&mut self, val: i64) -> Result<(), Error> {
        self.write_all_bytes(&val.swap_bytes().to_be_bytes())
    }
    /// Writes a single [`i128`] in big-endian order, 16 bytes, MSB first.
    fn write_be_i128(&mut self, val: i128) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`i128`] in little-endian order, 16 bytes, LSB first.
    fn write_le_i128(&mut self, val: i128) -> Result<(), Error> {
        self.write_all_bytes(&val.swap_bytes().to_be_bytes())
    }

    /// Writes a sized blob, a series of bytes preceded by a [`u8`] declaring the size
    fn write_u8_blob(&mut self, val: &[u8]) -> Result<(), Error> {
        if val.len() > u8::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "value is too long to fit into a u8",
            ));
        }
        self.write_u8(val.len() as u8)?;
        self.write_all_bytes(val)
    }
    /// Writes a sized blob, a series of bytes preceded by a [`u16`] declaring the size
    fn write_u16_blob(&mut self, val: &[u8]) -> Result<(), Error> {
        if val.len() > u16::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "value is too long to fit into a u16",
            ));
        }
        self.write_be_u16(val.len() as u16)?;
        self.write_all_bytes(val)
    }
    /// Writes a sized blob, a series of bytes preceded by a [`u32`] declaring the size
    fn write_u32_blob(&mut self, val: &[u8]) -> Result<(), Error> {
        if val.len() > u32::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "value is too long to fit into a u32",
            ));
        }
        self.write_be_u32(val.len() as u32)?;
        self.write_all_bytes(val)
    }
    /// Writes a sized blob, a series of bytes preceded by a [`u64`] declaring the size
    fn write_u64_blob(&mut self, val: &[u8]) -> Result<(), Error> {
        if val.len() > u64::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "value is too long to fit into a u64",
            ));
        }
        self.write_be_u64(val.len() as u64)?;
        self.write_all_bytes(val)
    }

    /// Writes all the bytes in order
    fn write_all_bytes(&mut self, val: &[u8]) -> Result<(), Error> {
        for val in val {
            self.write_u8(*val)?;
        }
        Ok(())
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result<(), Error> {
        self.write_all_bytes(format(args).as_bytes())
    }

    fn write_some_bytes(&mut self, val: &[u8]) -> usize {
        let mut wrote = 0;
        for val in val {
            if self.write_u8(*val).is_err() {
                return wrote;
            }
            wrote += 1;
        }
        wrote
    }
}

#[cfg(feature = "std")]
impl MutBits for std::fs::File {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        use std::io::Write;
        self.write_all(&[val])
    }
}

macro_rules! impl_push {
    ($cast:ty, $($ty:tt)+) => {
        impl MutBits for $($ty)+ {
            fn write_u8(&mut self, val: u8) -> Result<(), Error> {
                self.push(val as $cast);
                Ok(())
            }
        }
    };
}

impl_push!(char, &mut String);
impl_push!(char, String);
impl_push!(u8, Vec<u8>);
impl_push!(u8, &mut Vec<u8>);
impl MutBits for &mut [u8] {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        let Some((a, b)) = core::mem::take(self).split_first_mut() else {
            return Err(ErrorKind::UnexpectedEof.into());
        };
        *a = val;
        *self = b;
        Ok(())
    }
}

macro_rules! impl_mutbits_vecdeque {
    ($($ty:tt)+) => {
        impl MutBits for $($ty)+ {
            fn write_u8(&mut self, val: u8) -> Result<(), Error> {
                self.push_back(val);
                Ok(())
            }
        }
    };
}

impl_mutbits_vecdeque!(VecDeque<u8>);
impl_mutbits_vecdeque!(&mut VecDeque<u8>);

#[cfg(feature = "std")]
pub mod stdwrappers {
    use crate::bits::{Bits, Error, MutBits};
    pub struct BitsWrapper<'a, T>(pub &'a mut T);

    impl<'a, T> Bits for BitsWrapper<'a, T>
    where
        T: std::io::Read,
    {
        fn next_u8(&mut self) -> Result<Option<u8>, Error> {
            let mut byte: u8 = 0;
            let read = self.0.read(core::slice::from_mut(&mut byte))?;
            if read < 1 {
                return Ok(None);
            }
            Ok(Some(byte))
        }
    }

    impl<'a, T> MutBits for BitsWrapper<'a, T>
    where
        T: std::io::Write,
    {
        fn write_u8(&mut self, val: u8) -> Result<(), Error> {
            self.0.write_all(&[val])
        }
    }
}
#[cfg(feature = "std")]
pub use stdwrappers::*;

pub fn read_be_u32<T: Bits>(mut data: T) -> Result<u32, Error> {
    data.read_be_u32()
}

pub fn read_be_u64<T: Bits>(mut data: T) -> Result<u64, Error> {
    data.read_be_u64()
}

pub fn read_f32<T: Bits>(mut data: T) -> Result<f32, Error> {
    data.read_f32()
}

pub fn read_f64<T: Bits>(mut data: T) -> Result<f64, Error> {
    data.read_f64()
}
