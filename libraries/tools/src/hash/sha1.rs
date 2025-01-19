// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Bog standard implementation of SHA1 / RFC3174.
//!
//! *THIS SHOULD NOT BE USED FOR ANYTHING SECURITY RELATED*

#![allow(clippy::indexing_slicing)]

use crate::buf::{Buffer, FixedBuf, RoundBuffer};
use crate::hash::HashDigest;
use core::ops::{BitAnd, BitOr, BitXor, Not};
use irox_bits::{Bits, Error, MutBits};

pub const BLOCK_SIZE: usize = 64;
pub const OUTPUT_SIZE: usize = 20;

///
/// Implementation of [RFC 3174](https://datatracker.ietf.org/doc/html/rfc3174) based on the [Wikipedia](https://en.wikipedia.org/wiki/SHA-1#Examples_and_pseudocode) algorithm
///
///
/// **THIS SHOULD NOT BE USED FOR ANYTHING SECURITY RELATED**
pub struct SHA1 {
    written_length: u64,
    buf: RoundBuffer<BLOCK_SIZE, u8>,

    h0: u32,
    h1: u32,
    h2: u32,
    h3: u32,
    h4: u32,
}

impl Default for SHA1 {
    fn default() -> Self {
        Self {
            h0: 0x67452301,
            h1: 0xEFCDAB89,
            h2: 0x98BADCFE,
            h3: 0x10325476,
            h4: 0xC3D2E1F0,
            written_length: 0,
            buf: RoundBuffer::default(),
        }
    }
}

impl SHA1 {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    fn try_chomp(&mut self) {
        if self.buf.len() < BLOCK_SIZE {
            return;
        }
        let mut words: FixedBuf<80, u32> = FixedBuf::default();
        for i in 0..16 {
            words[i] = self.buf.read_be_u32().unwrap_or_default();
        }
        for i in 16..=79 {
            let w1 = words.get(i - 3).copied().unwrap_or_default();
            let w2 = words.get(i - 8).copied().unwrap_or_default();
            let w3 = words.get(i - 14).copied().unwrap_or_default();
            let w4 = words.get(i - 16).copied().unwrap_or_default();
            words[i] = w1.bitxor(w2).bitxor(w3).bitxor(w4).rotate_left(1);
        }

        let mut a = self.h0;
        let mut b = self.h1;
        let mut c = self.h2;
        let mut d = self.h3;
        let mut e = self.h4;

        for i in 0..=79 {
            let mut f: u32 = 0;
            let mut k: u32 = 0;
            match i {
                0..=19 => {
                    f = b.bitand(c).bitxor(b.not().bitand(d));
                    k = 0x5A827999;
                }
                20..=39 => {
                    f = b.bitxor(c).bitxor(d);
                    k = 0x6ED9EBA1;
                }
                40..=59 => {
                    f = b.bitand(c).bitor(b.bitand(d)).bitor(c.bitand(d));
                    k = 0x8F1BBCDC;
                }
                60..=79 => {
                    f = b.bitxor(c).bitxor(d);
                    k = 0xCA62C1D6;
                }
                _ => {
                    // unreachable
                }
            }
            let w = words.get(i as usize).copied().unwrap_or_default();
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        self.h0 = self.h0.wrapping_add(a);
        self.h1 = self.h1.wrapping_add(b);
        self.h2 = self.h2.wrapping_add(c);
        self.h3 = self.h3.wrapping_add(d);
        self.h4 = self.h4.wrapping_add(e);
    }
    ///
    /// Finishes the hash and returns the result.
    pub fn finish(mut self) -> [u8; OUTPUT_SIZE] {
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
        let _ = self.buf.write_be_u64(self.written_length << 3);
        self.try_chomp();
        let mut out: [u8; OUTPUT_SIZE] = [0; OUTPUT_SIZE];
        let mut v = out.as_mut_slice();
        let _ = v.write_be_u32(self.h0);
        let _ = v.write_be_u32(self.h1);
        let _ = v.write_be_u32(self.h2);
        let _ = v.write_be_u32(self.h3);
        let _ = v.write_be_u32(self.h4);
        out
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
    pub fn hash(mut self, bytes: &[u8]) -> [u8; OUTPUT_SIZE] {
        self.write(bytes);
        self.finish()
    }
}

impl MutBits for SHA1 {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.write(&[val]);
        Ok(())
    }
}

impl HashDigest<BLOCK_SIZE, OUTPUT_SIZE> for SHA1 {
    fn write(&mut self, bytes: &[u8]) {
        SHA1::write(self, bytes)
    }

    fn hash(self, bytes: &[u8]) -> [u8; OUTPUT_SIZE] {
        SHA1::hash(self, bytes)
    }

    fn finish(self) -> [u8; OUTPUT_SIZE] {
        SHA1::finish(self)
    }
}

#[cfg(test)]
mod test {
    use crate::hash::sha1::SHA1;
    use crate::hex::from_hex_into;
    use irox_bits::Error;

    #[test]
    pub fn test_sha1() -> Result<(), Error> {
        let tests = [
            ("DA39A3EE 5E6B4B0D 3255BFEF 95601890 AFD80709", ""),
            ("86F7E437 FAA5A7FC E15D1DDC B9EAEAEA 377667B8", "a"),
            ("A9993E36 4706816A BA3E2571 7850C26C 9CD0D89D", "abc"),
            (
                "C12252CE DA8BE899 4D5FA029 0A47231C 1D16AAE3",
                "message digest",
            ),
            (
                "32D10C7B 8CF96570 CA04CE37 F2A19D84 240D3A89",
                "abcdefghijklmnopqrstuvwxyz",
            ),
            (
                "761C457B F73B14D2 7E9E9265 C46F4B4D DA11F940",
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            ),
            (
                "CF0800F7 644ACE3C B4C3FA33 388D3BA0 EA3C8B6E",
                "0123456789012345678901234567890123456789012345678901234567890123",
            ),
            (
                "50ABF570 6A150990 A08B2C5E A40FA0E5 85554732",
                "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
            ),
            (
                "84983E44 1C3BD26E BAAE4AA1 F95129E5 E54670F1",
                "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            ),
            (
                "2FD4E1C6 7A2D28FC ED849EE1 BB76E739 1B93EB12",
                "The quick brown fox jumps over the lazy dog",
            ),
            (
                "DE9F2C7F D25E1B3A FAD3E85A 0BD17D9B 100DB4B3",
                "The quick brown fox jumps over the lazy cog",
            ),
            (
                "408D9438 4216F890 FF7A0C35 28E8BED1 E0B01621",
                "The quick brown fox jumps over the lazy dog.",
            ),
        ];
        for (hash, st) in tests {
            let mut bbuf: [u8; 1024] = [0; 1024];
            let mut buf = bbuf.as_mut_slice();
            let wrote = from_hex_into(hash, &mut buf)?;
            let hash = SHA1::default().hash(st.as_bytes());
            assert_eq!(&bbuf[0..wrote], &hash);
        }

        Ok(())
    }
}
