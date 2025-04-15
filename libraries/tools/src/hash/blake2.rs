// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::integer_division_remainder_used)]

use crate::buf::ArrayBuf;
use crate::buf::FixedU8Buf;
use crate::hash::HashDigest;
use core::ops::{BitXorAssign, Not};
use irox_bits::WriteToLEBits;

macro_rules! g {
    (
        $a:expr,
        $b:expr,
        $c:expr,
        $d:expr,
        $x:expr,
        $y:expr,
        $r:expr
    ) => {
        *$a = ($a.wrapping_add(*$b).wrapping_add($x));
        *$d ^= *$a;
        *$d = ($d.rotate_right($r[0]));
        *$c = ($c.wrapping_add(*$d));
        *$b ^= *$c;
        *$b = ($b.rotate_right($r[1]));
        *$a = ($a.wrapping_add(*$b).wrapping_add($y));
        *$d ^= *$a;
        *$d = ($d.rotate_right($r[2]));
        *$c = ($c.wrapping_add(*$d));
        *$b ^= *$c;
        *$b = ($b.rotate_right($r[3]));
    };
}
static BLAKE2B_R: &[u32; 4] = &[32, 24, 16, 63];
static BLAKE2S_R: &[u32; 4] = &[16, 12, 8, 7];
static BLAKE2B_IV: &[u64; 8] = &[
    0x6A09E667F3BCC908,
    0xBB67AE8584CAA73B,
    0x3C6EF372FE94F82B,
    0xA54FF53A5F1D36F1,
    0x510E527FADE682D1,
    0x9B05688C2B3E6C1F,
    0x1F83D9ABFB41BD6B,
    0x5BE0CD19137E2179,
];
static BLAKE2S_IV: &[u32; 8] = &[
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];
static SIGMA: &[&[usize; 16]; 12] = &[
    &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    &[14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    &[11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    &[7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    &[9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    &[2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    &[12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    &[13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    &[6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    &[10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
    &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    &[14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
];

macro_rules! impl_blake2 {
    ($name:ident,$prim:ty,$nprim:ty,$nb:literal,$bb:literal,$w:literal, $r:literal, $iv:ident, $RND:ident) => {
        pub struct $name<const NN: usize> {
            h: [$prim; 8],
            written: $nprim,
            buf: ArrayBuf<16, $nb, $prim>,
        }
        impl<const NN: usize> Default for $name<NN> {
            fn default() -> Self {
                Self::new(&[])
            }
        }
        impl<const NN: usize> $name<NN> {
            pub fn new(key: &[u8]) -> Self {
                let mut out = Self {
                    h: *$iv,
                    written: 0,
                    buf: Default::default(),
                };
                let nn = (NN as $prim) & 0xFF;
                let kk = ((key.len() as $prim) & 0xFF) << 8;
                let p = (0x01010000 as $prim) | nn | kk;
                out.h[0].bitxor_assign(p);
                if kk > 0 {
                    let mut key = FixedU8Buf::<$bb>::from_slice(key);
                    out.write(key.as_mut_full());
                }

                out
            }

            fn chomp(&mut self, last: bool) {
                if self.buf.is_empty() && self.written > 0 && !last {
                    return;
                }
                let m = self.buf.take_le_buf();

                let counter = self.written; // + ($nb - (self.written & Self::MASK));
                let mut v: [$prim; 16] = [0; 16];
                (v[0..8]).copy_from_slice(&self.h);
                (v[8..]).copy_from_slice($iv);
                //     self.h[0], self.h[1], self.h[2], self.h[3], self.h[4], self.h[5], self.h[6],
                //     self.h[7], $iv[0], $iv[1], $iv[2], $iv[3], $iv[4], $iv[5], $iv[6], $iv[7],
                // ];
                v[12].bitxor_assign(counter as $prim);
                v[13].bitxor_assign((counter >> $w) as $prim);
                if last {
                    v[14] = v[14].not();
                }
                for i in 0..$r {
                    let s = SIGMA[i % 10];
                    g!(&mut v[0], &mut v[4], &mut v[8], &mut v[12], m[s[0]], m[s[1]], $RND);
                    g!(&mut v[1], &mut v[5], &mut v[9], &mut v[13], m[s[2]], m[s[3]], $RND);
                    g!(&mut v[2], &mut v[6], &mut v[10], &mut v[14], m[s[4]], m[s[5]], $RND);
                    g!(&mut v[3], &mut v[7], &mut v[11], &mut v[15], m[s[6]], m[s[7]], $RND);

                    g!(&mut v[0], &mut v[5], &mut v[10], &mut v[15], m[s[8]], m[s[9]], $RND);
                    g!(&mut v[1], &mut v[6], &mut v[11], &mut v[12], m[s[10]], m[s[11]], $RND);
                    g!(&mut v[2], &mut v[7], &mut v[8], &mut v[13], m[s[12]], m[s[13]], $RND);
                    g!(&mut v[3], &mut v[4], &mut v[9], &mut v[14], m[s[14]], m[s[15]], $RND);
                }
                for i in 0..8 {
                    self.h[i].bitxor_assign(v[i]);
                    self.h[i].bitxor_assign(v[i + 8]);
                }
            }
            pub fn write(&mut self, mut v: &[u8]) {
                let align = self.buf.rem_align();
                if align < 4 && align < v.len() {
                    let (a, b) = v.split_at(self.buf.rem_align());
                    v = b;
                    for val in a {
                        if self.buf.is_full() {
                            self.chomp(false);
                        }
                        let _ = self.buf.write_le_u8(*val);
                        self.written += 1;
                    }
                }

                let mut chunks = v.chunks_exact($nb);
                for c in chunks.by_ref() {
                    if self.buf.is_full() {
                        self.chomp(false);
                    }
                    let _ = self
                        .buf
                        .push_prim(<$prim>::from_le_bytes(c.try_into().unwrap_or_default()));
                    self.written += $nb;
                }
                for val in chunks.remainder() {
                    if self.buf.is_full() {
                        self.chomp(false);
                    }
                    let _ = self.buf.write_le_u8(*val);
                    self.written += 1;
                }
            }
            pub fn hash(mut self, v: &[u8]) -> [u8; NN] {
                self.write(v);
                self.finish()
            }
            pub fn finish(mut self) -> [u8; NN] {
                self.chomp(true);

                // let out: [u8; NN] = unsafe { self.h.align_to::<u8>().1 }.copy_subset();
                let mut out: FixedU8Buf<NN> = FixedU8Buf::default();
                for v in self.h {
                    let _ = v.write_le_to(&mut out);
                }
                // out
                out.take()
            }
        }
        impl<const NN: usize> HashDigest<$bb, NN> for $name<NN> {
            fn finish(self) -> [u8; NN] {
                todo!()
            }
            fn hash(self, _: &[u8]) -> [u8; NN] {
                todo!()
            }
            fn write(&mut self, _: &[u8]) {
                todo!()
            }
        }
    };
}

impl_blake2!(BLAKE2s, u32, u64, 4, 64, 32, 10, BLAKE2S_IV, BLAKE2S_R);
impl_blake2!(BLAKE2b, u64, u128, 8, 128, 64, 12, BLAKE2B_IV, BLAKE2B_R);
pub type BLAKE2s128 = BLAKE2s<16>;
pub type BLAKE2s160 = BLAKE2s<20>;
pub type BLAKE2s224 = BLAKE2s<28>;
pub type BLAKE2s256 = BLAKE2s<32>;
pub type BLAKE2b160 = BLAKE2b<20>;
pub type BLAKE2b224 = BLAKE2b<28>;
pub type BLAKE2b256 = BLAKE2b<32>;
pub type BLAKE2b384 = BLAKE2b<48>;
pub type BLAKE2b512 = BLAKE2b<64>;

#[cfg(test)]
mod tests {
    use crate::hash::{BLAKE2b384, BLAKE2b512, BLAKE2s224, BLAKE2s256};
    use crate::hex;

    #[test]
    pub fn test0() {
        let h = BLAKE2s224::default().hash(b"");
        let exp = hex!("1fa1291e65248b37b3433475b2a0dd63d54a11ecc4e3e034e7bc1ef4");
        assert_eq_hex_slice!(exp, h);
        let h = BLAKE2s256::default().hash(b"");
        let exp = hex!("69217a3079908094e11121d042354a7c1f55b6482ca1a51e1b250dfd1ed0eef9");
        assert_eq_hex_slice!(exp, h);
        let h = BLAKE2b384::default().hash(b"");
        let exp = hex!("b32811423377f52d7862286ee1a72ee540524380fda1724a6f25d7978c6fd3244a6caf0498812673c5e05ef583825100");
        assert_eq_hex_slice!(exp, h);
        let h = BLAKE2b512::default().hash(b"");
        let exp = hex!("786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce");
        assert_eq_hex_slice!(exp, h);
    }
    #[test]
    pub fn test_qbf() {
        let h = BLAKE2b512::default().hash(b"The quick brown fox jumps over the lazy dog");
        let exp = hex!("a8add4bdddfd93e4877d2746e62817b116364a1fa7bc148d95090bc7333b3673f82401cf7aa2e4cb1ecd90296e3f14cb5413f8ed77be73045b13914cdcd6a918");
        assert_eq_hex_slice!(exp, h);
        let h = BLAKE2b512::default().hash(b"The quick brown fox jumps over the lazy dof");
        let exp = hex!("ab6b007747d8068c02e25a6008db8a77c218d94f3b40d2291a7dc8a62090a744c082ea27af01521a102e42f480a31e9844053f456b4b41e8aa78bbe5c12957bb");
        assert_eq_hex_slice!(exp, h);
    }

    #[test]
    pub fn test_abc() {
        let h = BLAKE2s256::default().hash(b"abc");
        let exp = hex!("508C5E8C327C14E2E1A72BA34EEB452F37458B209ED63A294D999B4C86675982");
        assert_eq_hex_slice!(exp, h);
        let h = BLAKE2b512::default().hash(b"abc");
        let exp = hex!("BA80A53F981C4D0D6A2797B69F12F6E94C212F14685AC4B74B12BB6FDBFFA2D17D87C5392AAB792DC252D5DE4533CC9518D38AA8DBF1925AB92386EDD4009923");
        assert_eq_hex_slice!(exp, h);
    }

    #[test]
    pub fn test_long() {
        let exp = hex!("508C5E8C327C14E2E1A72BA34EEB452F37458B209ED63A294D999B4C86675982");
        let mut inp = [0u8; 128];
        let mut v = BLAKE2s256::default();
        for i in 0..100000 {
            v.write(&inp);
            inp[0] = inp[0].wrapping_add(i as u8);
        }
    }
}
