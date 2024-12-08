// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::{Bits, BitsError, BitsErrorKind, BitsWrapper, MutBits};
use core::cmp::Ordering;

pub struct BitStreamEncoder<'a, T: MutBits> {
    delegate: BitsWrapper<'a, T>,
    buf: u32,
    remaining: u8,
}
impl<'a, T: MutBits> Drop for BitStreamEncoder<'a, T> {
    fn drop(&mut self) {
        let [a, b, c, d] = self.buf.to_be_bytes();
        if self.remaining < 8 {
            // write 4
            let _ = self.delegate.write_all_bytes(&[a, b, c, d]);
        } else if self.remaining < 16 {
            // write 3
            let _ = self.delegate.write_all_bytes(&[a, b, c]);
        } else if self.remaining < 24 {
            // write 2
            let _ = self.delegate.write_all_bytes(&[a, b]);
        } else if self.remaining < 32 {
            // write 1
            let _ = self.delegate.write_u8(a);
        }
    }
}
impl<'a, T: MutBits> BitStreamEncoder<'a, T> {
    pub fn new(delegate: BitsWrapper<'a, T>) -> Self {
        Self {
            delegate,
            buf: 0u32,
            remaining: 32u8,
        }
    }
    pub fn write_u8_bits(&mut self, val: u8, num_bits: u8) -> Result<(), BitsError> {
        self.write_u32_bits(val as u32, num_bits)
    }
    pub fn write_u16_bits(&mut self, val: u16, num_bits: u8) -> Result<(), BitsError> {
        self.write_u32_bits(val as u32, num_bits)
    }
    pub fn write_u32_bits(&mut self, val: u32, mut num_bits: u8) -> Result<(), BitsError> {
        if num_bits > 32 {
            return Err(BitsErrorKind::InvalidInput.into());
        }
        while num_bits > 0 {
            match num_bits.cmp(&self.remaining) {
                Ordering::Less => {
                    let shift = self.remaining - num_bits;
                    let mask = (1u32 << num_bits) - 1;
                    self.buf |= (val & mask) << shift;
                    self.remaining -= num_bits;
                    num_bits = 0;
                }
                Ordering::Equal => {
                    let mask = (1u32 << num_bits) - 1;
                    self.buf |= val & mask;
                    num_bits = 0;
                    self.delegate.write_be_u32(self.buf)?;
                    self.remaining = 32;
                    self.buf = 0;
                }
                Ordering::Greater => {
                    let touse = self.remaining;
                    let shift = num_bits - self.remaining;
                    let mask = (1u32 << touse) - 1;
                    self.buf |= (val >> shift) & mask;
                    self.delegate.write_be_u32(self.buf)?;
                    self.remaining = 32;
                    self.buf = 0;
                    num_bits -= touse;
                }
            }
        }
        Ok(())
    }
}

pub struct BitStreamDecoder<'a, T: Bits> {
    delegate: BitsWrapper<'a, T>,
    buf: u32,
    used: u8,
}
impl<'a, T: Bits> BitStreamDecoder<'a, T> {
    pub fn new(delegate: BitsWrapper<'a, T>) -> Self {
        Self {
            delegate,
            buf: 0,
            used: 0,
        }
    }
    pub fn read_u32_bits(&mut self, num_bits: u8) -> Result<u32, BitsError> {
        if num_bits > 32 {
            return Err(BitsErrorKind::InvalidInput.into());
        }
        loop {
            match self.used.cmp(&num_bits) {
                Ordering::Less => {
                    // used < numbits - add more.
                    let v = self.delegate.read_u8()?;
                    self.buf = (self.buf << 8) | v as u32;
                    self.used += 8;
                }
                Ordering::Equal => {
                    let mask = (1u32 << num_bits) - 1;
                    self.used = 0;
                    let b = self.buf & mask;
                    self.buf = 0;
                    return Ok(b);
                }
                Ordering::Greater => {
                    let rem = self.used - num_bits;
                    let mask = (1u32 << num_bits) - 1;
                    let b = (self.buf >> rem) & mask;
                    self.used -= num_bits;
                    return Ok(b);
                }
            }
        }
    }
}
#[cfg(all(test, feature = "std"))]
mod test {
    use crate::{BitStreamDecoder, BitStreamEncoder, BitsError, BitsWrapper};

    #[test]
    pub fn test_dec() -> Result<(), BitsError> {
        let buf = vec![0xAB, 0xCD, 0xAB, 0xCD];
        let mut dec = BitStreamDecoder::new(BitsWrapper::Owned(buf));
        assert_eq!(0xA, dec.read_u32_bits(4)?);
        assert_eq!(0xB, dec.read_u32_bits(4)?);
        assert_eq!(0xC, dec.read_u32_bits(4)?);
        assert_eq!(0xD, dec.read_u32_bits(4)?);
        assert_eq!(0xABCD, dec.read_u32_bits(16)?);
        Ok(())
    }

    #[test]
    pub fn test_dec2() -> Result<(), BitsError> {
        let buf = vec![0x03, 0xC0, 0x81, 0x00, 0x88, 0x10, 0x1A, 0x02];
        let mut dec = BitStreamDecoder::new(BitsWrapper::Owned(buf));
        assert_eq!(7, dec.read_u32_bits(9)?);
        assert_eq!(258, dec.read_u32_bits(9)?);
        assert_eq!(8, dec.read_u32_bits(9)?);
        assert_eq!(8, dec.read_u32_bits(9)?);
        assert_eq!(258, dec.read_u32_bits(9)?);
        assert_eq!(6, dec.read_u32_bits(9)?);
        assert_eq!(257, dec.read_u32_bits(9)?);

        Ok(())
    }

    #[test]
    pub fn test_enc() -> Result<(), BitsError> {
        let mut buf = Vec::<u8>::new();
        {
            let wrap = BitsWrapper::Borrowed(&mut buf);
            let mut enc = BitStreamEncoder::new(wrap);

            enc.write_u16_bits(0xAAAA, 4)?;
            enc.write_u16_bits(0xBBBB, 4)?;
            enc.write_u16_bits(0xCCCC, 4)?;
            enc.write_u16_bits(0xDDDD, 4)?;
            enc.write_u16_bits(0xABCD, 16)?;
        }
        // println!("{:?}", buf);
        assert_eq!(buf, [0xAB, 0xCD, 0xAB, 0xCD]);
        Ok(())
    }

    #[test]
    pub fn test_enc2() -> Result<(), BitsError> {
        let mut buf = Vec::<u8>::new();
        {
            let wrap = BitsWrapper::Borrowed(&mut buf);
            let mut enc = BitStreamEncoder::new(wrap);

            enc.write_u16_bits(7, 9)?;
            enc.write_u16_bits(258, 9)?;
            enc.write_u16_bits(8, 9)?;
            enc.write_u16_bits(8, 9)?;
            enc.write_u16_bits(258, 9)?;
            enc.write_u16_bits(6, 9)?;
            enc.write_u16_bits(257, 9)?;
        }
        assert_eq!(buf, [0x03, 0xC0, 0x81, 0x00, 0x88, 0x10, 0x1A, 0x02]);
        // 0x007  0b000000111
        // 0x102             100000010
        // 0x008                      000001000
        // 0x008                               000001000
        // 0x102                                        100000010
        // 0x006                                                 000000011
        // 0x03 = 0b00000011
        // 0xC0 =           11000000
        // 0x81 =                   10000001
        // 0x00 =                           00000000
        // 0x88 =                                   10001000
        // 0x10 =                                           00010000

        Ok(())
    }
}
