// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::indexing_slicing)]

use crate::Chacha20KeyGenerator;
use core::ops::{Add, AddAssign};
use core::ops::{BitAnd, BitAndAssign, Index, IndexMut, Not};
use irox_bits::{array_split_16, Bits, Error, MutBits};
use irox_tools::buf::{Buffer, FixedU8Buf};
use irox_tools::hex;
use irox_tools::iterators::Zipping;

const _P1305: U136 = U136::from_hex(hex!("FAFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF03"));

const MASK: U136 = U136::from_hex(hex!("FFFFFF0FFCFFFF0FFCFFFF0FFCFFFF0FFF"));
const MINUSP: U136 = U136::from_hex(hex!("05000000000000000000000000000000FC"));
const EMPTY: U136 = U136([0; 17]);

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
struct U136([u32; 17]);

impl U136 {
    const fn from_hex(v: [u8; 17]) -> Self {
        let mut out = EMPTY;
        let mut i = 0;
        while i < 17 {
            out.0[i] = v[i] as u32;
            i += 1;
        }
        out
    }
    fn read_from_bits<T: Bits>(input: &mut T) -> Option<Self> {
        let mut out = Self::default();
        let mut num_read = 0;
        for v in &mut out.0 {
            if num_read == 16 {
                *v = 1;
                break;
            }
            if let Ok(Some(a)) = input.next_u8() {
                num_read += 1;
                *v = a as u32;
            } else {
                *v = 1;
                break;
            }
        }
        (num_read > 0).then_some(out)
    }
}
impl From<u128> for U136 {
    fn from(value: u128) -> Self {
        let mut out = U136::default();
        for (a, b) in out.0.as_mut_slice().iter_mut().zip(value.to_le_bytes()) {
            *a = b as u32;
        }
        out
    }
}
impl From<[u32; 17]> for U136 {
    fn from(h: [u32; 17]) -> Self {
        let mut out = U136::default();
        let mut working = 0u32;
        for (o, a) in out.0.iter_mut().zip(h.iter()) {
            working += *a;
            *o = working & 0xFF;
            working >>= 8;
        }
        out
    }
}
impl From<[u8; 16]> for U136 {
    fn from(h: [u8; 16]) -> Self {
        let mut out = U136::default();
        let mut working = 0u32;
        for (o, a) in out.0.iter_mut().zip(h.iter()) {
            working += *a as u32;
            *o = working & 0xFF;
            working >>= 8;
        }
        out
    }
}
impl Add for U136 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        (&self).add(other)
    }
}

impl Add<U136> for &U136 {
    type Output = U136;

    fn add(self, other: U136) -> Self::Output {
        let mut working = 0u32;
        let mut out = [0u32; 17];
        for (o, [a, b]) in out
            .iter_mut()
            .zip(Zipping::from([self.0.iter(), other.0.iter()]))
        {
            working += a;
            working += b;
            *o = working & 0xFF;
            working >>= 8;
        }

        U136(out)
    }
}

impl AddAssign for U136 {
    fn add_assign(&mut self, other: Self) {
        let mut working = 0u32;
        for (out, a) in self.0.iter_mut().zip(other.0.iter()) {
            working += *out;
            working += *a;
            *out = working & 0xFF;
            working >>= 8;
        }
    }
}
impl BitAnd for U136 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        let mut out = [0u32; 17];
        for (o, [a, b]) in out
            .iter_mut()
            .zip(Zipping::from([self.0.iter(), other.0.iter()]))
        {
            *o = a & b;
        }
        U136(out)
    }
}
impl BitAndAssign for U136 {
    fn bitand_assign(&mut self, other: Self) {
        for (out, a) in self.0.iter_mut().zip(other.0.iter()) {
            *out &= *a;
        }
    }
}
impl Index<usize> for U136 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for U136 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub struct Poly1305 {
    r: U136,
    s: [u8; 16],
    h: U136,
    c: U136,
    u: u32,
    buf: FixedU8Buf<16>,
}
#[allow(clippy::needless_range_loop)]
impl Poly1305 {
    pub fn new(key: &[u8; 32]) -> Self {
        let (r, s) = array_split_16(*key);
        let mut r = U136::from(r);
        r.bitand_assign(MASK);

        let h = EMPTY;
        let c: U136 = EMPTY;
        let u: u32 = 0;
        Self {
            r,
            s,
            h,
            c,
            u,
            buf: Default::default(),
        }
    }
    fn try_process_block(&mut self) {
        if !self.buf.is_full() {
            return;
        }
        if let Some(c) = U136::read_from_bits(&mut self.buf.as_ref_used()) {
            self.c = c;
            self.process_block();
        }
        self.buf.clear();
    }
    fn process_block(&mut self) {
        self.h.add_assign(self.c);
        let mut x = EMPTY;
        for i in 0..17 {
            x[i] = 0;
            for j in 0..17 {
                let mult = if j <= i {
                    self.r[i - j]
                } else {
                    self.r[i + 17 - j] * 320
                };
                x[i] += mult * self.h[j];
            }
        }
        self.h = x;
        self.u = 0;
        for j in 0..16 {
            self.u += self.h[j];
            self.h[j] = self.u & 0xFF;
            self.u >>= 8;
        }
        self.u += self.h[16];
        self.h[16] = self.u & 0x3;
        self.u = 5 * (self.u >> 2);
        for j in 0..16 {
            self.u += self.h[j];
            self.h[j] = self.u & 0xFF;
            self.u >>= 8;
        }
        self.u += self.h[16];
        self.h[16] = self.u;
    }
    pub fn finish(mut self) -> [u8; 16] {
        self.try_process_block();
        if let Some(c) = U136::read_from_bits(&mut self.buf.as_ref_used()) {
            self.c = c;
            self.process_block();
        }
        let g = self.h;
        self.h.add_assign(MINUSP);
        let q = (self.h[16] >> 7).not().wrapping_add(1);
        for j in 0..17 {
            self.h[j] ^= q & (g[j] ^ self.h[j]);
        }
        self.c = self.s.into();
        self.c[16] = 0;
        self.h.add_assign(self.c);
        let mut out = [0u8; 16];
        for j in 0..16 {
            out[j] = self.h[j] as u8;
        }
        out
    }
    pub fn hash<T: Bits>(msg: &mut T, key: &[u8; 32]) -> [u8; 16] {
        let mut poly = Poly1305::new(key);
        while let Ok(Some(v)) = msg.next_u8() {
            let _ = poly.write_u8(v);
        }
        poly.finish()
    }

    pub fn key_gen(key: &[u8; 32], nonce: &[u8; 12]) -> [u8; 32] {
        let mut block = Chacha20KeyGenerator::generate(*key, 0, *nonce);
        let out = block.pop_n_front::<32>();
        out.unwrap_or_default()
    }
}
impl MutBits for Poly1305 {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        let _ = self.buf.push(val);
        self.try_process_block();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Poly1305;
    use irox_tools::{assert_eq_hex_slice, hex};

    #[test]
    pub fn test_block1() {
        let key = hex!("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
        let msg = b"Cryptographic Forum Research Group";
        let tag = hex!("a8061dc1305136c6c22b8baf0c0127a9");
        let mut msg = msg.as_slice();
        let calc = Poly1305::hash(&mut msg, &key);
        assert_eq_hex_slice!(tag, calc);
    }

    const IETF: &[u8] = b"\
Any submission to the IETF intended by the Contributor \
for publication as all or part of an IETF Internet-Draft \
or RFC and any statement made within the context of an IETF \
activity is considered an \"IETF Contribution\". Such statements \
include oral statements in IETF sessions, as well as written and \
electronic communications made at any time or place, which are addressed to";

    #[test]
    pub fn tv01() {
        let key = [0u8; 32];
        let msg = [0u8; 64];
        let tag = [0u8; 16];
        let calc = Poly1305::hash(&mut msg.as_slice(), &key);
        assert_eq_hex_slice!(tag, calc);
    }
    #[test]
    pub fn tv02() {
        let key = hex!("0000000000000000000000000000000036e5f6b5c5e06070f0efca96227a863e");
        let mut msg = IETF;
        let tag = hex!("36e5f6b5c5e06070f0efca96227a863e");
        let calc = Poly1305::hash(&mut msg, &key);
        assert_eq_hex_slice!(tag, calc);
    }
    #[test]
    pub fn tv03() {
        let key = hex!("36e5f6b5c5e06070f0efca96227a863e00000000000000000000000000000000");
        let mut msg = IETF;
        let tag = hex!("f3477e7cd95417af89a6b8794c310cf0");
        let calc = Poly1305::hash(&mut msg, &key);
        assert_eq_hex_slice!(tag, calc);
    }
    const JABBER: &[u8] = b"'Twas brillig, and the slithy toves\nDid gyre and gimble in the wabe:\nAll mimsy were the borogoves,\nAnd the mome raths outgrabe.";
    #[test]
    pub fn tv04() {
        let key = hex!("1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0");
        let mut msg = JABBER;
        let tag = hex!("4541669a7eaaee61e708dc7cbcc5eb62");
        let calc = Poly1305::hash(&mut msg, &key);
        assert_eq_hex_slice!(tag, calc);
    }
    #[test]
    pub fn tv05() {
        let key = hex!("0200000000000000000000000000000000000000000000000000000000000000");
        let mut msg = hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").as_slice();
        let tag = hex!("03000000000000000000000000000000").as_slice();
        let calc = Poly1305::hash(&mut msg, &key);
        assert_eq_hex_slice!(tag, calc);
    }
    #[test]
    pub fn tv06() {
        let key = hex!("02000000000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
        let mut msg = hex!("02000000000000000000000000000000").as_slice();
        let tag = hex!("03000000000000000000000000000000").as_slice();
        let calc = Poly1305::hash(&mut msg, &key);
        assert_eq_hex_slice!(tag, calc);
    }
    #[test]
    pub fn tv07() {
        let key = hex!("0100000000000000000000000000000000000000000000000000000000000000");
        let mut msg = hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF11000000000000000000000000000000").as_slice();
        let tag = hex!("05000000000000000000000000000000").as_slice();
        let calc = Poly1305::hash(&mut msg, &key);
        assert_eq_hex_slice!(tag, calc);
    }
    #[test]
    pub fn tv08() {
        let key = hex!("0100000000000000000000000000000000000000000000000000000000000000");
        let mut msg = hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFBFEFEFEFEFEFEFEFEFEFEFEFEFEFEFE01010101010101010101010101010101").as_slice();
        let tag = hex!("00000000000000000000000000000000").as_slice();
        let calc = Poly1305::hash(&mut msg, &key);
        assert_eq_hex_slice!(tag, calc);
    }
    #[test]
    pub fn tv09() {
        let key = hex!("0200000000000000000000000000000000000000000000000000000000000000");
        let mut msg = hex!("FDFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").as_slice();
        let tag = hex!("FAFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").as_slice();
        let calc = Poly1305::hash(&mut msg, &key);
        assert_eq_hex_slice!(tag, calc);
    }
    #[test]
    pub fn tv10() {
        let key = hex!("0100000000000000040000000000000000000000000000000000000000000000");
        let mut msg = hex!("E33594D7505E43B900000000000000003394D7505E4379CD01000000000000000000000000000000000000000000000001000000000000000000000000000000").as_slice();
        let tag = hex!("14000000000000005500000000000000").as_slice();
        let calc = Poly1305::hash(&mut msg, &key);
        assert_eq_hex_slice!(tag, calc);
    }
    #[test]
    pub fn tv11() {
        let key = hex!("0100000000000000040000000000000000000000000000000000000000000000");
        let mut msg = hex!("E33594D7505E43B900000000000000003394D7505E4379CD010000000000000000000000000000000000000000000000").as_slice();
        let tag = hex!("13000000000000000000000000000000").as_slice();
        let calc = Poly1305::hash(&mut msg, &key);
        assert_eq_hex_slice!(tag, calc);
    }
    #[test]
    pub fn test_key() {
        let key = hex!("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f");
        let nonce = hex!("000000000001020304050607");
        let exp = hex!("8ad5a08b905f81cc815040274ab29471a833b637e3fd0da508dbb8e2fdd1a646");
        let calc = Poly1305::key_gen(&key, &nonce);
        assert_eq_hex_slice!(exp, calc);
    }
    #[test]
    pub fn test_kv1() {
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let exp = hex!("76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7");
        let calc = Poly1305::key_gen(&key, &nonce);
        assert_eq_hex_slice!(exp, calc);
    }
    #[test]
    pub fn test_kv2() {
        let key = hex!("0000000000000000000000000000000000000000000000000000000000000001");
        let nonce = hex!("000000000000000000000002");
        let exp = hex!("ecfa254f845f647473d3cb140da9e87606cb33066c447b87bc2666dde3fbb739");
        let calc = Poly1305::key_gen(&key, &nonce);
        assert_eq_hex_slice!(exp, calc);
    }
    #[test]
    pub fn test_kv3() {
        let key = hex!("1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0");
        let nonce = hex!("000000000000000000000002");
        let exp = hex!("965e3bc6f9ec7ed9560808f4d229f94b137ff275ca9b3fcbdd59deaad23310ae");
        let calc = Poly1305::key_gen(&key, &nonce);
        assert_eq_hex_slice!(exp, calc);
    }
}
