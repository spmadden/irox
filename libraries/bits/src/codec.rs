// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Bits encoding and decoding functions

use crate::{Bits, BitsWrapper, Error, MutBits};

/// Splits the input into two equal sized arrays.
pub const fn array_concat_2(a: [u8; 2], b: [u8; 2]) -> [u8; 4] {
    let [c, d] = b;
    let [a, b] = a;
    [a, b, c, d]
}
/// Splits the input into two equal sized arrays.
pub const fn array_split_2(a: [u8; 4]) -> ([u8; 2], [u8; 2]) {
    let [a, b, c, d] = a;
    ([a, b], [c, d])
}
/// Splits the input into two equal sized arrays.
pub const fn array_concat_4(a: [u8; 4], b: [u8; 4]) -> [u8; 8] {
    let [e, f, g, h] = b;
    let [a, b, c, d] = a;
    [a, b, c, d, e, f, g, h]
}
/// Splits the input into two equal sized arrays.
pub const fn array_split_4(a: [u8; 8]) -> ([u8; 4], [u8; 4]) {
    let [a, b, c, d, e, f, g, h] = a;
    ([a, b, c, d], [e, f, g, h])
}
/// Splits the input into two equal sized arrays.
pub const fn array_concat_8(a: [u8; 8], b: [u8; 8]) -> [u8; 16] {
    let [i, j, k, l, m, n, o, p] = b;
    let [a, b, c, d, e, f, g, h] = a;
    [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]
}
/// Splits the input into two equal sized arrays.
pub const fn array_split_8(a: [u8; 16]) -> ([u8; 8], [u8; 8]) {
    let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = a;
    ([a, b, c, d, e, f, g, h], [i, j, k, l, m, n, o, p])
}
/// Splits the input into two equal sized arrays.
pub const fn array_concat_16(a: [u8; 16], b: [u8; 16]) -> [u8; 32] {
    let [q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af] = b;
    let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = a;
    [
        a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac,
        ad, ae, af,
    ]
}
/// Splits the input into two equal sized arrays.
pub const fn array_split_16(a: [u8; 32]) -> ([u8; 16], [u8; 16]) {
    let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af] =
        a;
    (
        [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p],
        [q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af],
    )
}
/// Converts the value into a constant number of bytes
pub trait ToBEBytes<const N: usize> {
    fn to_be_bytes(&self) -> [u8; N];
}
/// Converts to the value from a constant number of bytes
pub trait FromBEBytes<const N: usize> {
    fn from_be_bytes(bytes: [u8; N]) -> Self;
}

impl ToBEBytes<1> for u8 {
    fn to_be_bytes(&self) -> [u8; 1] {
        [*self]
    }
}
impl FromBEBytes<1> for u8 {
    fn from_be_bytes(bytes: [u8; 1]) -> u8 {
        bytes[0]
    }
}

impl ToBEBytes<2> for u16 {
    fn to_be_bytes(&self) -> [u8; 2] {
        u16::to_be_bytes(*self)
    }
}
impl FromBEBytes<2> for u16 {
    fn from_be_bytes(bytes: [u8; 2]) -> u16 {
        u16::from_be_bytes(bytes)
    }
}
impl ToBEBytes<4> for u32 {
    fn to_be_bytes(&self) -> [u8; 4] {
        u32::to_be_bytes(*self)
    }
}

// impl ToBEBytes<8> for [u32; 2] {
//     fn to_be_bytes(&self) -> [u8; 8] {
//         let [a, b] = *self;
//         array_concat_4(u32::to_be_bytes(a), u32::to_be_bytes(b))
//     }
// }
// impl ToBEBytes<16> for [u32; 4] {
//     fn to_be_bytes(&self) -> [u8; 16] {
//         let [a, b, c, d] = *self;
//         array_concat_8(
//             array_concat_4(u32::to_be_bytes(a), u32::to_be_bytes(b)),
//             array_concat_4(u32::to_be_bytes(c), u32::to_be_bytes(d)),
//         )
//     }
// }
impl<const N: usize, const Y: usize> ToBEBytes<Y> for [u32; N] {
    fn to_be_bytes(&self) -> [u8; Y] {
        let mut out = [0u8; Y];
        let mut wr = BitsWrapper::Owned(out.as_mut_slice());
        for v in self {
            let _ = wr.write_be_u32(*v);
        }
        out
    }
}

impl FromBEBytes<4> for u32 {
    fn from_be_bytes(bytes: [u8; 4]) -> u32 {
        u32::from_be_bytes(bytes)
    }
}
impl FromBEBytes<8> for [u32; 2] {
    fn from_be_bytes(bytes: [u8; 8]) -> [u32; 2] {
        let (a, b) = array_split_4(bytes);
        [u32::from_be_bytes(a), u32::from_be_bytes(b)]
    }
}
impl FromBEBytes<16> for [u32; 4] {
    fn from_be_bytes(bytes: [u8; 16]) -> [u32; 4] {
        let (a, b) = array_split_8(bytes);
        let (c, d) = array_split_4(b);
        let (a, b) = array_split_4(a);
        [
            u32::from_be_bytes(a),
            u32::from_be_bytes(b),
            u32::from_be_bytes(c),
            u32::from_be_bytes(d),
        ]
    }
}
impl ToBEBytes<4> for f32 {
    fn to_be_bytes(&self) -> [u8; 4] {
        f32::to_be_bytes(*self)
    }
}
impl FromBEBytes<4> for f32 {
    fn from_be_bytes(bytes: [u8; 4]) -> f32 {
        f32::from_be_bytes(bytes)
    }
}
impl ToBEBytes<8> for [f32; 2] {
    fn to_be_bytes(&self) -> [u8; 8] {
        let [a, b] = *self;
        array_concat_4(f32::to_be_bytes(a), f32::to_be_bytes(b))
    }
}
impl FromBEBytes<8> for [f32; 2] {
    fn from_be_bytes(bytes: [u8; 8]) -> [f32; 2] {
        let (a, b) = array_split_4(bytes);
        [f32::from_be_bytes(a), f32::from_be_bytes(b)]
    }
}

impl ToBEBytes<8> for u64 {
    fn to_be_bytes(&self) -> [u8; 8] {
        u64::to_be_bytes(*self)
    }
}
impl FromBEBytes<8> for u64 {
    fn from_be_bytes(bytes: [u8; 8]) -> u64 {
        u64::from_be_bytes(bytes)
    }
}

impl ToBEBytes<8> for f64 {
    fn to_be_bytes(&self) -> [u8; 8] {
        f64::to_be_bytes(*self)
    }
}
impl FromBEBytes<8> for f64 {
    fn from_be_bytes(bytes: [u8; 8]) -> f64 {
        f64::from_be_bytes(bytes)
    }
}
impl ToBEBytes<16> for [f64; 2] {
    fn to_be_bytes(&self) -> [u8; 16] {
        let [a, b] = *self;
        array_concat_8(a.to_be_bytes(), b.to_be_bytes())
    }
}
impl FromBEBytes<16> for [f64; 2] {
    fn from_be_bytes(bytes: [u8; 16]) -> [f64; 2] {
        let (a, b) = array_split_8(bytes);
        [f64::from_be_bytes(a), f64::from_be_bytes(b)]
    }
}

impl ToBEBytes<16> for [u64; 2] {
    fn to_be_bytes(&self) -> [u8; 16] {
        let [a, b] = *self;
        array_concat_8(u64::to_be_bytes(a), u64::to_be_bytes(b))
    }
}
impl FromBEBytes<16> for [u64; 2] {
    fn from_be_bytes(bytes: [u8; 16]) -> [u64; 2] {
        let (a, b) = array_split_8(bytes);
        [u64::from_be_bytes(a), u64::from_be_bytes(b)]
    }
}

impl ToBEBytes<16> for u128 {
    fn to_be_bytes(&self) -> [u8; 16] {
        u128::to_be_bytes(*self)
    }
}
impl FromBEBytes<16> for u128 {
    fn from_be_bytes(bytes: [u8; 16]) -> u128 {
        u128::from_be_bytes(bytes)
    }
}
impl ToBEBytes<32> for [u128; 2] {
    fn to_be_bytes(&self) -> [u8; 32] {
        let [a, b] = *self;
        array_concat_16(u128::to_be_bytes(a), u128::to_be_bytes(b))
    }
}
impl FromBEBytes<32> for [u128; 2] {
    fn from_be_bytes(bytes: [u8; 32]) -> [u128; 2] {
        let (a, b) = array_split_16(bytes);
        [u128::from_be_bytes(a), u128::from_be_bytes(b)]
    }
}

/// Writes 'self' to the provided [`MutBits`] impl in big endian order.
pub trait WriteToBEBits {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error>;
}

impl WriteToBEBits for u8 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_u8(*self)?;
        Ok(1)
    }
}
impl WriteToBEBits for u16 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_be_u16(*self)?;
        Ok(2)
    }
}

impl WriteToBEBits for u32 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_be_u32(*self)?;
        Ok(4)
    }
}
impl WriteToBEBits for u64 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_be_u64(*self)?;
        Ok(8)
    }
}
impl WriteToBEBits for u128 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_be_u128(*self)?;
        Ok(16)
    }
}
impl WriteToBEBits for f32 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_be_f32(*self)?;
        Ok(4)
    }
}
impl WriteToBEBits for f64 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_be_f64(*self)?;
        Ok(8)
    }
}
impl WriteToBEBits for i8 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_i8(*self)?;
        Ok(1)
    }
}
impl WriteToBEBits for i16 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_be_i16(*self)?;
        Ok(2)
    }
}
impl WriteToBEBits for i32 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_be_i32(*self)?;
        Ok(4)
    }
}
impl WriteToBEBits for i64 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_be_i64(*self)?;
        Ok(8)
    }
}
impl WriteToBEBits for i128 {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_be_i128(*self)?;
        Ok(16)
    }
}

pub trait ReadFromBEBits: Sized {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error>;
}
impl ReadFromBEBits for u8 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_u8()
    }
}
impl ReadFromBEBits for u16 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_be_u16()
    }
}
impl ReadFromBEBits for u32 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_be_u32()
    }
}
impl ReadFromBEBits for u64 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_be_u64()
    }
}
impl ReadFromBEBits for u128 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_be_u128()
    }
}
impl ReadFromBEBits for f32 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_be_f32()
    }
}
impl ReadFromBEBits for f64 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_be_f64()
    }
}
impl ReadFromBEBits for i8 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_i8()
    }
}
impl ReadFromBEBits for i16 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_be_i16()
    }
}
impl ReadFromBEBits for i32 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_be_i32()
    }
}
impl ReadFromBEBits for i64 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_be_i64()
    }
}
impl ReadFromBEBits for i128 {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        inp.read_be_i128()
    }
}
impl<const N: usize> ReadFromBEBits for [u8; N] {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        let mut out = [0; N];
        inp.read_all_into(&mut out.as_mut_slice())?;
        Ok(out)
    }
}

/// compile time u64x2 to u128 conversion
pub const fn u64_to_u128(val: [u64; 2]) -> u128 {
    let [a, b] = val;
    u128::from_be_bytes(array_concat_8(u64::to_be_bytes(a), u64::to_be_bytes(b)))
}
/// compile time u32x2 to u64 conversion
pub const fn u32_to_u64(val: [u32; 2]) -> u64 {
    let [a, b] = val;
    u64::from_be_bytes(array_concat_4(u32::to_be_bytes(a), u32::to_be_bytes(b)))
}
/// compile time u32x4 to u128 conversion
pub const fn u32_to_u128(val: [u32; 4]) -> u128 {
    let [a, b, c, d] = val;
    u64_to_u128([u32_to_u64([a, b]), u32_to_u64([c, d])])
}
/// compile time u16x2 to u32 conversion
pub const fn u16_to_u32(val: [u16; 2]) -> u32 {
    let [a, b] = val;
    u32::from_be_bytes(array_concat_2(u16::to_be_bytes(a), u16::to_be_bytes(b)))
}
/// compile time u16 to u64 conversion
pub const fn u16_to_u64(val: [u16; 4]) -> u64 {
    let [a, b, c, d] = val;
    u32_to_u64([u16_to_u32([a, b]), u16_to_u32([c, d])])
}
/// compile time u16 to u128 conversion
pub const fn u16_to_u128(val: [u16; 8]) -> u128 {
    let [a, b, c, d, e, f, g, h] = val;
    u64_to_u128([u16_to_u64([a, b, c, d]), u16_to_u64([e, f, g, h])])
}

/// compile time u128 to u64 conversion
pub const fn u128_to_u64(val: u128) -> [u64; 2] {
    let a = val as u64;
    let b = (val >> 64) as u64;
    [b, a]
}
/// compile time u64 to u32 conversion
pub const fn u64_to_u32(val: u64) -> [u32; 2] {
    let a = val as u32;
    let b = (val >> 32) as u32;
    [b, a]
}
/// compile time u32 to u16 conversion
pub const fn u32_to_u16(val: u32) -> [u16; 2] {
    let a = val as u16;
    let b = (val >> 16) as u16;
    [b, a]
}
/// compile time u128 to u32 conversion
pub const fn u128_to_u32(val: u128) -> [u32; 4] {
    let [a, b] = u128_to_u64(val);
    let [c, d] = u64_to_u32(a);
    let [e, f] = u64_to_u32(b);
    [c, d, e, f]
}
/// compile time u64 to u16 conversion
pub const fn u64_to_u16(val: u64) -> [u16; 4] {
    let [a, b] = u64_to_u32(val);
    let [c, d] = u32_to_u16(a);
    let [e, f] = u32_to_u16(b);
    [c, d, e, f]
}
/// compile-time u128 to u16 conversion
pub const fn u128_to_u16(val: u128) -> [u16; 8] {
    let [a, b, c, d] = u128_to_u32(val);
    let [e, f] = u32_to_u16(a);
    let [g, h] = u32_to_u16(b);
    let [i, j] = u32_to_u16(c);
    let [k, l] = u32_to_u16(d);
    [e, f, g, h, i, j, k, l]
}

impl WriteToBEBits for &str {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_str_u32_blob(self)
    }
}

cfg_feature_alloc! {
    extern crate alloc;
    impl WriteToBEBits for alloc::string::String {
        fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
            bits.write_str_u32_blob(self.as_str())
        }
    }
    impl ReadFromBEBits for alloc::string::String {
        fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
            inp.read_str_u32_blob()
        }
    }
}
