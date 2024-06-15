// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Bog standard implementation of MD5 / RFC1321.
//!
//! *THIS SHOULD NOT BE USED FOR ANYTHING SECURITY RELATED*

use crate::buf::{Buffer, RoundBuffer};
use crate::u32::{FromU32Array, ToU32Array};
use crate::HashDigest;
use core::ops::{BitAnd, BitOr, BitXor, Not};
use irox_bits::{Bits, Error, MutBits};

pub const BLOCK_SIZE: usize = 64;
pub const OUTPUT_SIZE: usize = 16;

static SHIFT_AMOUNTS: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

static KONSTANTS: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

///
/// Implementation of [RFC 1321](https://datatracker.ietf.org/doc/html/rfc1321) based on the [Wikipedia](https://en.wikipedia.org/wiki/MD5#Algorithm) algorithm
///
///
/// **THIS SHOULD NOT BE USED FOR ANYTHING SECURITY RELATED**
pub struct MD5 {
    written_length: u64,
    a0: u32,
    b0: u32,
    c0: u32,
    d0: u32,
    buf: RoundBuffer<BLOCK_SIZE, u8>,
}

impl Default for MD5 {
    fn default() -> Self {
        Self {
            written_length: 0,
            buf: RoundBuffer::default(),
            a0: 0x67452301,
            b0: 0xefcdab89,
            c0: 0x98badcfe,
            d0: 0x10325476,
        }
    }
}

impl MD5 {
    fn try_chomp(&mut self) {
        if self.buf.len() < BLOCK_SIZE {
            return;
        }

        let words: [u32; 16] = [
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
            self.buf.read_be_u32().unwrap_or_default().swap_bytes(),
        ];

        let mut a = self.a0;
        let mut b = self.b0;
        let mut c = self.c0;
        let mut d = self.d0;
        for i in 0..=63 {
            let mut f: u32 = 0;
            let mut g: u32 = 0;
            match i {
                0..=15 => {
                    f = b.bitand(c).bitor(b.not().bitand(d));
                    g = i;
                }
                16..=31 => {
                    f = d.bitand(b).bitor(d.not().bitand(c));
                    g = i.wrapping_mul(5).wrapping_add(1).bitand(0xF);
                }
                32..=47 => {
                    f = b.bitxor(c).bitxor(d);
                    g = i.wrapping_mul(3).wrapping_add(5).bitand(0xF);
                }
                48..=63 => {
                    f = c.bitxor(b.bitor(d.not()));
                    g = i.wrapping_mul(7).bitand(0xF);
                }
                _ => {
                    // unreachable.
                }
            };
            let i = i as usize;
            let k = KONSTANTS.get(i).copied().unwrap_or_default();
            let m = words.get(g as usize).copied().unwrap_or_default();
            f = f.wrapping_add(a).wrapping_add(k).wrapping_add(m);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(SHIFT_AMOUNTS.get(i).copied().unwrap_or_default()));
        }

        self.a0 = self.a0.wrapping_add(a);
        self.b0 = self.b0.wrapping_add(b);
        self.c0 = self.c0.wrapping_add(c);
        self.d0 = self.d0.wrapping_add(d);
    }

    ///
    /// Finishes the hash and returns the result.
    pub fn finish(mut self) -> u128 {
        let mut modlen64 = (self.written_length & 0x3F) as usize;
        let mut pad: usize = 0;
        if modlen64 >= 56 {
            // append 64 bits/8 bytes;
            pad += BLOCK_SIZE - modlen64;
            modlen64 = 0;
        }
        pad += 56 - modlen64;
        let _ = self.buf.push_back(0x80);
        pad -= 1;
        for _ in 0..pad {
            self.try_chomp();
            let _ = self.buf.push_back(0);
        }
        let [a, b] = (self.written_length << 3).to_u32_array();
        let _ = self.buf.write_be_u32(b.swap_bytes());
        let _ = self.buf.write_be_u32(a.swap_bytes());
        self.try_chomp();
        // assert_eq!(0, self.buf.len(), "Buffer length wasn't zeroed!");
        u128::from_u32_array(&[
            self.a0.swap_bytes(),
            self.b0.swap_bytes(),
            self.c0.swap_bytes(),
            self.d0.swap_bytes(),
        ])
    }

    ///
    /// Appends the bytes to the internal buffer.  NOTE: You must call 'finish' to get the final result.
    pub fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            let _ = self.buf.push_back(*b);
            self.written_length += 1;
            self.try_chomp();
        }
    }

    ///
    /// Hashes the provided bytes.
    pub fn hash(mut self, bytes: &[u8]) -> u128 {
        self.write(bytes);
        self.finish()
    }
}

impl MutBits for MD5 {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.write(&[val]);
        Ok(())
    }
}

impl HashDigest<BLOCK_SIZE, OUTPUT_SIZE> for MD5 {
    fn write(&mut self, bytes: &[u8]) {
        MD5::write(self, bytes)
    }

    fn hash(self, bytes: &[u8]) -> [u8; OUTPUT_SIZE] {
        MD5::hash(self, bytes).to_be_bytes() as [u8; OUTPUT_SIZE]
    }

    fn finish(self) -> [u8; OUTPUT_SIZE] {
        MD5::finish(self).to_be_bytes() as [u8; OUTPUT_SIZE]
    }
}

#[cfg(test)]
mod test {
    use crate::md5::MD5;

    #[test]
    pub fn test_md5() {
        assert_eq_hex!(
            0xd41d8cd98f00b204e9800998ecf8427e,
            MD5::default().hash("".as_bytes())
        );
        assert_eq_hex!(
            0x0cc175b9c0f1b6a831c399e269772661,
            MD5::default().hash("a".as_bytes())
        );
        assert_eq_hex!(
            0x900150983cd24fb0d6963f7d28e17f72,
            MD5::default().hash("abc".as_bytes())
        );
        assert_eq_hex!(
            0xf96b697d7cb7938d525a2f31aaf161d0,
            MD5::default().hash("message digest".as_bytes())
        );
        assert_eq_hex!(
            0xc3fcd3d76192e4007dfb496cca67e13b,
            MD5::default().hash("abcdefghijklmnopqrstuvwxyz".as_bytes())
        );
        assert_eq_hex!(
            0xd174ab98d277d9f5a5611c2c9f419d9f,
            MD5::default()
                .hash("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes())
        );
        assert_eq_hex!(
            0x7f7bfd348709deeaace19e3f535f8c54,
            MD5::default().hash(
                "0123456789012345678901234567890123456789012345678901234567890123".as_bytes()
            )
        );
        assert_eq_hex!(
            0x57edf4a22be3c955ac49da2e2107b67a,
            MD5::default().hash(
                "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                    .as_bytes()
            )
        );
        assert_eq_hex!(
            0x9e107d9d372bb6826bd81d3542a419d6,
            MD5::default().hash("The quick brown fox jumps over the lazy dog".as_bytes())
        );
        assert_eq_hex!(
            0xe4d909c290d0fb1ca068ffaddf22cbd0,
            MD5::default().hash("The quick brown fox jumps over the lazy dog.".as_bytes())
        );
    }
}
