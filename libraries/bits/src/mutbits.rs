// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

cfg_feature_alloc! {
    extern crate alloc;
}

use crate::{Error, ErrorKind};
use core::ops::{Deref, DerefMut};

///
/// Write methods for the primitive types
pub trait MutBits {
    /// Writes a single [`u8`]
    fn write_u8(&mut self, val: u8) -> Result<(), Error>;

    /// Writes a single [`i8`]
    fn write_i8(&mut self, val: i8) -> Result<(), Error> {
        self.write_u8(val as u8)
    }

    /// Writes 1u8 if true, 0u8 if false
    fn write_bool(&mut self, val: bool) -> Result<(), Error> {
        self.write_u8(val as u8)
    }

    /// Encodes the character as UTF-8, and writes anywhere from 1-4 bytes.  The number of bytes
    /// written is returned.
    fn write_be_utf8_char(&mut self, val: char) -> Result<usize, Error> {
        crate::utf::write_be_utf8_char(val, self)
    }

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
    fn write_be_f32(&mut self, val: f32) -> Result<(), Error> {
        self.write_all_bytes(&val.to_be_bytes())
    }
    /// Writes a single [`u16`] in standard IEEE754 format, 8 bytes
    fn write_be_f64(&mut self, val: f64) -> Result<(), Error> {
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
    fn write_be_u16_blob(&mut self, val: &[u8]) -> Result<(), Error> {
        if val.len() > u16::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "value is too long to fit into a u16",
            ));
        }
        self.write_be_u16(val.len() as u16)?;
        self.write_all_bytes(val)
    }
    /// Writes a sized blob, a series of bytes preceded by a [`u16`] declaring the size
    fn write_le_u16_blob(&mut self, val: &[u8]) -> Result<(), Error> {
        if val.len() > u16::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "value is too long to fit into a u16",
            ));
        }
        self.write_le_u16(val.len() as u16)?;
        self.write_all_bytes(val)
    }
    /// Writes a sized blob, a series of bytes preceded by a [`u32`] declaring the size
    fn write_be_u32_blob(&mut self, val: &[u8]) -> Result<(), Error> {
        if val.len() > u32::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "value is too long to fit into a u32",
            ));
        }
        self.write_be_u32(val.len() as u32)?;
        self.write_all_bytes(val)
    }
    /// Writes a sized blob, a series of bytes preceded by a [`u32`] declaring the size
    fn write_le_u32_blob(&mut self, val: &[u8]) -> Result<(), Error> {
        if val.len() > u32::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "value is too long to fit into a u32",
            ));
        }
        self.write_le_u32(val.len() as u32)?;
        self.write_all_bytes(val)
    }
    /// Writes a sized blob, a series of bytes preceded by a [`u64`] declaring the size
    fn write_be_u64_blob(&mut self, val: &[u8]) -> Result<(), Error> {
        if val.len() > u64::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "value is too long to fit into a u64",
            ));
        }
        self.write_be_u64(val.len() as u64)?;
        self.write_all_bytes(val)
    }
    /// Writes a sized blob, a series of bytes preceded by a [`u64`] declaring the size
    fn write_le_u64_blob(&mut self, val: &[u8]) -> Result<(), Error> {
        if val.len() > u64::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "value is too long to fit into a u64",
            ));
        }
        self.write_le_u64(val.len() as u64)?;
        self.write_all_bytes(val)
    }

    /// Writes all the bytes in order
    fn write_all_bytes(&mut self, val: &[u8]) -> Result<(), Error> {
        for v in val {
            self.write_u8(*v)?;
        }
        Ok(())
    }

    cfg_feature_alloc! {
        /// Allows the use of [`core::format_args`] macro
        fn write_fmt_impl(&mut self, args: core::fmt::Arguments<'_>) -> Result<(), Error> {
            extern crate alloc;
            self.write_all_bytes(alloc::fmt::format(args).as_bytes())
        }
    }

    ///
    /// Writes some subset of bytes until either the input is exhausted, or the
    /// underlying buffer fills up or errors.  Returns the total number of bytes
    /// written
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

    ///
    /// Writes a specific sized string from the stream, a string prefixed by a
    /// 4-byte big-endian length.
    fn write_str_u32_blob(&mut self, val: &str) -> Result<usize, Error> {
        let len = val.len() as u32;
        self.write_be_u32(len)?;
        self.write_all_bytes(val.as_bytes())?;

        Ok((len + 4) as usize)
    }

    ///
    /// Writes the exact amount of bytes from the input buffer to the output stream
    /// Returns an error if the buffer doesn't have enough values or if the
    /// output stream errors.
    fn write_exact(&mut self, val: &[u8], len: usize) -> Result<(), Error> {
        for idx in 0..len {
            let Some(val) = val.get(idx) else {
                return Err(ErrorKind::UnexpectedEof.into());
            };
            self.write_u8(*val)?;
        }
        Ok(())
    }
}

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

/// Wraps a borrowed [`MutBits`], providing an implementation of [`core::fmt::Write`] to permit the
/// use of the [`core::write!`] macro with a target of `&mut`[`MutBits`]
pub struct FormatBits<'a, T: MutBits + ?Sized>(pub &'a mut T);

impl<'a, T: MutBits + ?Sized> FormatBits<'a, T> {
    pub fn wrap(val: &'a mut T) -> Self {
        FormatBits(val)
    }
}
impl<'a, T: MutBits + ?Sized> From<&'a mut T> for FormatBits<'a, T> {
    fn from(val: &'a mut T) -> Self {
        FormatBits(val)
    }
}
impl<'a, T: MutBits + ?Sized> Deref for FormatBits<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a, T: MutBits + ?Sized> DerefMut for FormatBits<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'a, T: MutBits + ?Sized> core::fmt::Write for FormatBits<'a, T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            self.write_char(ch)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_be_utf8_char(c)?;
        Ok(())
    }
}
