// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Basic Bit Buffer interface
//!

// use std::io::{Error, ErrorKind, Read, Write};

extern crate alloc;
use alloc::fmt::format;
use alloc::{vec, vec::Vec};
use core::fmt::Arguments;

#[cfg(feature = "std")]
pub type Error = std::io::Error;
#[cfg(feature = "std")]
pub type ErrorKind = std::io::ErrorKind;

#[cfg(not(feature = "std"))]
pub use error::*;
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

    /// Reads a single [`u32`] in big-endian order, 4 bytes, MSB first.
    fn read_be_u32(&mut self) -> Result<u32, Error> {
        let Some(ret) = self.next_be_u32()? else {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        };
        Ok(ret)
    }

    /// Optionally reads a single [`u32`] in big-endian order, 4 bytes, MSB first.
    fn next_be_u32(&mut self) -> Result<Option<u32>, Error> {
        let Some(a) = self.next_u8()? else {
            return Ok(None);
        };
        let mut out: u32 = ((a as u32) << 8) | maybe_next_u8!(self, a as u32) as u32;
        next_and_shift!(self, u32, out);
        next_and_shift!(self, u32, out);
        next_and_shift!(self, u32, out);

        Ok(Some(out))
    }

    /// Reads a single [`u64`] in big-endian order, 8 bytes, MSB first.
    fn read_be_u64(&mut self) -> Result<u64, Error> {
        let Some(ret) = self.next_be_u64()? else {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        };
        Ok(ret)
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
        next_and_shift!(self, u64, out);

        Ok(Some(out))
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

    /// Optionally reads a single [`i16`] in big-endian order, 2 bytes, MSB first.
    fn next_be_i16(&mut self) -> Result<Option<i16>, Error> {
        Ok(self.next_be_u16()?.map(|v| v as i16))
    }

    /// Reads a single [`i32`] in big-endian order, 4 bytes, MSB first.
    fn read_be_i32(&mut self) -> Result<i32, Error> {
        Ok(self.read_be_u32()? as i32)
    }

    /// Optionally reads a single [`i32`] in big-endian order, 4 bytes, MSB first.
    fn next_be_i32(&mut self) -> Result<Option<i32>, Error> {
        Ok(self.next_be_u32()?.map(|v| v as i32))
    }

    /// Reads a single [`i64`] in big-endian order, 8 bytes, MSB first.
    fn read_be_i64(&mut self) -> Result<i64, Error> {
        Ok(self.read_be_u64()? as i64)
    }

    /// Optionally reads a single [`i64`] in big-endian order, 8 bytes, MSB first.
    fn next_be_i64(&mut self) -> Result<Option<i64>, Error> {
        Ok(self.next_be_u64()?.map(|v| v as i64))
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
        for _i in 0..size {
            buf.push(self.read_u8()?);
        }
        Ok(buf)
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

#[cfg(feature = "std")]
impl<T> Bits for T
where
    T: std::io::Read,
{
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        let mut byte: u8 = 0;
        let read = self.read(core::slice::from_mut(&mut byte))?;
        if read < 1 {
            return Ok(None);
        }
        Ok(Some(byte))
    }

    fn next_be_u16(&mut self) -> Result<Option<u16>, Error> {
        let mut buf: [u8; 2] = [0; 2];
        absorb_eof!(self, buf);
        let [a, b] = buf;
        let out: u16 = (a as u16) << 8 | (b as u16);
        Ok(Some(out))
    }

    fn next_be_u32(&mut self) -> Result<Option<u32>, Error> {
        let mut buf: [u8; 4] = [0; 4];
        absorb_eof!(self, buf);
        let out = u32::from_be_bytes(buf);
        Ok(Some(out))
    }

    fn next_be_u64(&mut self) -> Result<Option<u64>, Error> {
        let mut buf: [u8; 8] = [0; 8];
        absorb_eof!(self, buf);
        let out = u64::from_be_bytes(buf);
        Ok(Some(out))
    }

    fn next_be_u128(&mut self) -> Result<Option<u128>, Error> {
        let mut buf: [u8; 16] = [0; 16];
        absorb_eof!(self, buf);
        let out = u128::from_be_bytes(buf);
        Ok(Some(out))
    }

    fn read_exact_vec(&mut self, size: usize) -> Result<Vec<u8>, Error> {
        let mut buf: Vec<u8> = vec![0; size];
        self.read_exact(buf.as_mut_slice())?;
        Ok(buf)
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
    /// Writes a single [`u32`] in big-endian order, 4 bytes, MSB first.
    fn write_be_u32(&mut self, val: u32) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`u64`] in big-endian order, 8 bytes, MSB first.
    fn write_be_u64(&mut self, val: u64) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`u128`] in big-endian order, 16 bytes, MSB first.
    fn write_be_u128(&mut self, val: u128) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
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
    /// Writes a single [`i32`] in big-endian order, 4 bytes, MSB first.
    fn write_be_i32(&mut self, val: i32) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`i64`] in big-endian order, 8 bytes, MSB first.
    fn write_be_i64(&mut self, val: i64) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`i128`] in big-endian order, 16 bytes, MSB first.
    fn write_be_i128(&mut self, val: i128) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
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
}

#[cfg(feature = "std")]
impl<T> MutBits for T
where
    T: std::io::Write,
{
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.write_all(&[val])
    }

    fn write_be_u16(&mut self, val: u16) -> Result<(), Error> {
        self.write_all(&val.to_be_bytes())
    }

    fn write_be_u32(&mut self, val: u32) -> Result<(), Error> {
        self.write_all(&val.to_be_bytes())
    }

    fn write_be_u64(&mut self, val: u64) -> Result<(), Error> {
        self.write_all(&val.to_be_bytes())
    }

    fn write_be_u128(&mut self, val: u128) -> Result<(), Error> {
        self.write_all(&val.to_be_bytes())
    }

    fn write_f32(&mut self, val: f32) -> Result<(), Error> {
        self.write_all(&val.to_be_bytes())
    }

    fn write_f64(&mut self, val: f64) -> Result<(), Error> {
        self.write_all(&val.to_be_bytes())
    }

    fn write_be_i16(&mut self, val: i16) -> Result<(), Error> {
        self.write_all(&val.to_be_bytes())
    }

    fn write_be_i32(&mut self, val: i32) -> Result<(), Error> {
        self.write_all(&val.to_be_bytes())
    }

    fn write_be_i64(&mut self, val: i64) -> Result<(), Error> {
        self.write_all(&val.to_be_bytes())
    }

    fn write_be_i128(&mut self, val: i128) -> Result<(), Error> {
        self.write_all(&val.to_be_bytes())
    }
}
#[cfg(not(feature = "std"))]
pub use nostdimpls::*;
#[cfg(not(feature = "std"))]
mod nostdimpls {
    use crate::bits::{Error, ErrorKind};
    use alloc::vec::Vec;

    impl crate::bits::Bits for &[u8] {
        fn next_u8(&mut self) -> Result<Option<u8>, crate::bits::Error> {
            let Some((first, rest)) = self.split_first() else {
                return Ok(None);
            };
            *self = rest;
            Ok(Some(*first))
        }
    }
    impl crate::bits::MutBits for &mut [u8] {
        fn write_u8(&mut self, val: u8) -> Result<(), Error> {
            let Some((a, b)) = core::mem::take(self).split_first_mut() else {
                return Err(ErrorKind::UnexpectedEof.into());
            };
            *a = val;
            *self = b;
            Ok(())
        }
    }
    impl crate::bits::MutBits for &mut Vec<u8> {
        fn write_u8(&mut self, val: u8) -> Result<(), Error> {
            self.push(val);
            Ok(())
        }
    }
    impl crate::bits::MutBits for Vec<u8> {
        fn write_u8(&mut self, val: u8) -> Result<(), Error> {
            self.push(val);
            Ok(())
        }
    }
}

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
