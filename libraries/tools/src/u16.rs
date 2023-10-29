// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

///
/// Converts the specified primitive to a big-endian [`[u16;T]`]
pub trait ToU16Array<const T: usize> {
    ///
    /// Creates an big-endian array of [`u16`]'s from this specified primitive type.
    fn to_u16_array(&self) -> [u16; T];
}

impl ToU16Array<2> for u32 {
    fn to_u16_array(&self) -> [u16; 2] {
        let a = (self >> 16) as u16;
        let b = *self as u16;
        [a, b]
    }
}

impl ToU16Array<4> for u64 {
    fn to_u16_array(&self) -> [u16; 4] {
        let a = (self >> 48) as u16;
        let b = (self >> 32) as u16;
        let c = (self >> 16) as u16;
        let d = *self as u16;
        [a, b, c, d]
    }
}

impl ToU16Array<8> for u128 {

    fn to_u16_array(&self) -> [u16; 8] {
        let a = (self >> 112) as u16;
        let b = (self >> 96) as u16;
        let c = (self >> 80) as u16;
        let d = (self >> 64) as u16;
        let e = (self >> 48) as u16;
        let f = (self >> 32) as u16;
        let g = (self >> 16) as u16;
        let h = *self as u16;
        [a, b, c, d, e, f, g, h]
    }
}

///
/// Creates a Self from a constant u16 array.
pub trait FromU16Array<const L: usize> {
    ///
    /// Creates a primitive type from an big-endian array of [`u16`]'s
    fn from_u16_array(arr: &[u16; L]) -> Self;
}

impl FromU16Array<8> for u128 {
    fn from_u16_array(arr: &[u16; 8]) -> Self {
        let [a, b, c, d, e, f, g, h] = *arr;

        let a: u128 = (a as u128) << 112;
        let b: u128 = (b as u128) << 96;
        let c: u128 = (c as u128) << 80;
        let d: u128 = (d as u128) << 64;
        let e: u128 = (e as u128) << 48;
        let f: u128 = (f as u128) << 32;
        let g: u128 = (g as u128) << 16;
        let h: u128 = h as u128;

        a | b | c | d | e | f | g | h
    }
}

impl FromU16Array<4> for u64 {
    fn from_u16_array(arr: &[u16; 4]) -> Self {
        let [a, b, c, d] = *arr;

        let a: u64 = (a as u64) << 48;
        let b: u64 = (b as u64) << 32;
        let c: u64 = (c as u64) << 16;
        let d: u64 = d as u64;

        a | b | c | d
    }
}

impl FromU16Array<2> for u32 {
    fn from_u16_array(arr: &[u16; 2]) -> Self {
        let [a, b] = *arr;

        let a: u32 = (a as u32) << 16;
        let b: u32 = b as u32;

        a | b
    }
}