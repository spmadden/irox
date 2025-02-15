// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use irox_bits::{BitStreamDecoder, BitStreamEncoder, Bits, BitsWrapper, Error, MutBits};
const CLEAR_CODE: u16 = 256;
const END_OF_INFO: u16 = 257;

pub struct LZWEncoder<'a, T: MutBits> {
    strtable: BTreeMap<Box<[u8]>, u16>,
    delegate: BitStreamEncoder<'a, T>,
    next_val: u16,
    current_buf: Vec<u8>,
    current_bitlen: u8,
}
impl<'a, T: MutBits> LZWEncoder<'a, T> {
    pub fn new(delegate: BitsWrapper<'a, T>) -> Self {
        let mut strtable = BTreeMap::new();
        for i in 0..=255u8 {
            let v: Box<[u8]> = Box::from([i]);
            strtable.insert(v, i as u16);
        }
        let next_val = 258;
        Self {
            strtable,
            delegate: BitStreamEncoder::new(delegate),
            next_val,
            current_buf: Vec::new(),
            current_bitlen: 9,
        }
    }
    fn reset_state(&mut self) {
        self.strtable.clear();
        for i in 0..=255u8 {
            let v: Box<[u8]> = Box::from([i]);
            self.strtable.insert(v, i as u16);
        }
        self.next_val = 258;
        self.current_bitlen = 9;
    }
    fn code_from_buf(&mut self, buf: Box<[u8]>) -> u16 {
        *self.strtable.entry(buf).or_insert_with(|| {
            let v = self.next_val;
            self.next_val += 1;
            if self.next_val == 512 {
                self.current_bitlen = 10;
            } else if self.next_val == 1024 {
                self.current_bitlen = 11;
            } else if self.next_val == 2048 {
                self.current_bitlen = 12;
            }
            v
        })
    }
    #[allow(dead_code)]
    fn getstrcode(&self, val: &[u8]) -> Option<u16> {
        let bx = Box::<[u8]>::from(val);
        self.strtable.get(&bx).cloned()
    }
}
impl<T: MutBits> Drop for LZWEncoder<'_, T> {
    fn drop(&mut self) {
        if !self.current_buf.is_empty() {
            let code = {
                let buf = core::mem::take(&mut self.current_buf);
                let bx = buf.into_boxed_slice();
                self.code_from_buf(bx)
            };
            let _ = self.delegate.write_u16_bits(code, self.current_bitlen);
        }
        let _ = self
            .delegate
            .write_u16_bits(END_OF_INFO, self.current_bitlen);
    }
}

impl<T: MutBits> MutBits for LZWEncoder<'_, T> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        // check if in table:
        let mut buf = self.current_buf.clone();
        buf.push(val);
        let buf = buf.into_boxed_slice();
        if self.strtable.contains_key(&buf) {
            self.current_buf.push(val);
        } else {
            let bx = core::mem::take(&mut self.current_buf);
            let code = self.code_from_buf(bx.into_boxed_slice());
            self.delegate.write_u16_bits(code, self.current_bitlen)?;
            if code == 4094 {
                self.delegate
                    .write_u16_bits(CLEAR_CODE, self.current_bitlen)?;
                self.reset_state();
            }
            let _ = self.code_from_buf(buf);
            self.current_buf.clear();
            self.current_buf.push(val);
        }

        Ok(())
    }
}

pub struct LZWDecoder<'a, T: Bits> {
    _delegate: BitStreamDecoder<'a, T>,
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use crate::lzw::LZWEncoder;
    use alloc::vec::Vec;
    use irox_bits::{BitsWrapper, Error, MutBits};

    #[test]
    pub fn test() -> Result<(), Error> {
        let mut buf = Vec::<u8>::new();
        {
            let out = BitsWrapper::Borrowed(&mut buf);
            let mut lzw = LZWEncoder::new(out);
            let input: &[u8] = &[7, 7, 7, 8, 8, 7, 7, 6, 6];

            for i in input {
                lzw.write_u8(*i)?;
            }
            assert_eq!(lzw.strtable.len(), 262);
            assert_eq!(lzw.next_val, 264);
            assert_eq!(lzw.getstrcode(&[7, 7]), Some(258));
            assert_eq!(lzw.getstrcode(&[7, 7, 8]), Some(259));
            assert_eq!(lzw.getstrcode(&[8, 8]), Some(260));
            assert_eq!(lzw.getstrcode(&[8, 7]), Some(261));
            assert_eq!(lzw.getstrcode(&[7, 7, 6]), Some(262));
            assert_eq!(lzw.getstrcode(&[6, 6]), Some(263));
        }
        println!("{buf:#?}");
        assert_eq!(buf, [0x03, 0xC0, 0x81, 0x00, 0x88, 0x10, 0x18, 0x0D, 0x01]);

        Ok(())
    }
}
