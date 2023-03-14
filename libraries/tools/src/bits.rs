//!
//! Basic Bit Buffer interface
//!
use std::io::{Error, ErrorKind, Read};

///
/// Read methods for the primitive types
///
pub trait Bits {
    fn read_u8(&mut self) -> Result<u8, Error>;
    fn read_be_u16(&mut self) -> Result<u16, Error>;
    fn read_be_u32(&mut self) -> Result<u32, Error>;
    fn read_be_u64(&mut self) -> Result<u64, Error>;
    fn read_be_u128(&mut self) -> Result<u128, Error>;

    fn read_f32(&mut self) -> Result<f32, Error>;
    fn read_f64(&mut self) -> Result<f64, Error>;

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
        let ret = (first as u16) << 8 | (sec as u16);
        Ok(ret)
    }

    fn read_be_u32(&mut self) -> Result<u32, Error> {
        let first = self.read_be_u16()?;
        let sec = self.read_be_u16()?;
        let ret = (first as u32) << 16 | (sec as u32);
        Ok(ret)
    }

    fn read_f32(&mut self) -> Result<f32, Error> {
        Ok(f32::from_bits(self.read_be_u32()?))
    }

    fn read_be_u64(&mut self) -> Result<u64, Error> {
        let first = self.read_be_u32()?;
        let sec = self.read_be_u32()?;
        let ret = (first as u64) << 32 | (sec as u64);
        Ok(ret)
    }

    fn read_f64(&mut self) -> Result<f64, Error> {
        Ok(f64::from_bits(self.read_be_u64()?))
    }

    fn read_be_u128(&mut self) -> Result<u128, Error> {
        let first = self.read_be_u64()?;
        let sec = self.read_be_u64()?;
        let ret = (first as u128) << 32 | (sec as u128);
        Ok(ret)
    }

    fn advance(&mut self, len: usize) -> Result<usize, Error> {
        for _ in 0..len {
            self.read_u8()?;
        }
        Ok(len)
    }
}
