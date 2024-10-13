// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Bits encoding and decoding functions

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

impl ToBEBytes<8> for [u32; 2] {
    fn to_be_bytes(&self) -> [u8; 8] {
        let [a, b] = *self;
        array_concat_4(u32::to_be_bytes(a), u32::to_be_bytes(b))
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
