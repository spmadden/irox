// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Basic Bit Buffer interface
//!
use std::io::{Error, ErrorKind, Read, Write};

///
/// Read methods for the primitive types
///
pub trait Bits: Read {
    fn read_u8(&mut self) -> Result<u8, Error>;
    fn read_be_u16(&mut self) -> Result<u16, Error>;
    fn read_be_u32(&mut self) -> Result<u32, Error>;
    fn read_be_u64(&mut self) -> Result<u64, Error>;
    fn read_be_u128(&mut self) -> Result<u128, Error>;

    fn read_f32(&mut self) -> Result<f32, Error>;
    fn read_f64(&mut self) -> Result<f64, Error>;

    fn read_be_i16(&mut self) -> Result<i16, Error>;
    fn read_be_i32(&mut self) -> Result<i32, Error>;
    fn read_be_i64(&mut self) -> Result<i64, Error>;

    fn advance(&mut self, len: usize) -> Result<usize, Error>;
}

impl<T> Bits for T
where
    T: Read,
{
    fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf: [u8; 1] = [0];
        let read = self.read(&mut buf)?;
        if read < 1 {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        Ok(buf[0])
    }

    fn read_be_u16(&mut self) -> Result<u16, Error> {
        let first = self.read_u8()?;
        let sec = self.read_u8()?;
        let ret = u16::from(first) << 8 | u16::from(sec);
        Ok(ret)
    }

    fn read_be_u32(&mut self) -> Result<u32, Error> {
        let first = self.read_be_u16()?;
        let sec = self.read_be_u16()?;
        let ret = u32::from(first) << 16 | u32::from(sec);
        Ok(ret)
    }

    fn read_be_u64(&mut self) -> Result<u64, Error> {
        let first = self.read_be_u32()?;
        let sec = self.read_be_u32()?;
        let ret = u64::from(first) << 32 | u64::from(sec);
        Ok(ret)
    }

    fn read_be_u128(&mut self) -> Result<u128, Error> {
        let first = self.read_be_u64()?;
        let sec = self.read_be_u64()?;
        let ret = u128::from(first) << 32 | u128::from(sec);
        Ok(ret)
    }

    fn read_f32(&mut self) -> Result<f32, Error> {
        Ok(f32::from_bits(self.read_be_u32()?))
    }

    fn read_f64(&mut self) -> Result<f64, Error> {
        Ok(f64::from_bits(self.read_be_u64()?))
    }

    fn read_be_i16(&mut self) -> Result<i16, Error> {
        Ok(self.read_be_u16()? as i16)
    }

    fn read_be_i32(&mut self) -> Result<i32, Error> {
        Ok(self.read_be_u32()? as i32)
    }

    fn read_be_i64(&mut self) -> Result<i64, Error> {
        Ok(self.read_be_u64()? as i64)
    }

    fn advance(&mut self, len: usize) -> Result<usize, Error> {
        for _ in 0..len {
            self.read_u8()?;
        }
        Ok(len)
    }
}

pub trait MutBits: Write {
    fn write_u8(&mut self, val: u8) -> Result<(), Error>;
    fn write_be_u16(&mut self, val: u16) -> Result<(), Error>;
    fn write_be_u32(&mut self, val: u32) -> Result<(), Error>;
    fn write_be_u64(&mut self, val: u64) -> Result<(), Error>;
    fn write_be_u128(&mut self, val: u128) -> Result<(), Error>;

    fn write_f32(&mut self, val: f32) -> Result<(), Error>;
    fn write_f64(&mut self, val: f64) -> Result<(), Error>;

    fn write_be_i16(&mut self, val: i16) -> Result<(), Error>;
    fn write_be_i32(&mut self, val: i32) -> Result<(), Error>;
    fn write_be_i64(&mut self, val: i64) -> Result<(), Error>;
    fn write_be_i128(&mut self, val: i128) -> Result<(), Error>;
}

impl<T> MutBits for T
where
    T: Write,
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
