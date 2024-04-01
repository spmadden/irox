// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

///
/// Converts the specified primitive to a big-endian [`[u32;T]`]
pub trait ToU32Array<const T: usize> {
    ///
    /// Creates an big-endian array of [`u32`]'s from this specified primitive type.
    fn to_u32_array(&self) -> [u32; T];
}

impl ToU32Array<2> for u64 {
    fn to_u32_array(&self) -> [u32; 2] {
        let a = (self >> 32) as u32;
        let b = *self as u32;
        [a, b]
    }
}

impl ToU32Array<4> for u128 {
    fn to_u32_array(&self) -> [u32; 4] {
        let a = (self >> 96) as u32;
        let b = (self >> 64) as u32;
        let c = (self >> 32) as u32;
        let d = *self as u32;
        [a, b, c, d]
    }
}

///
/// Creates a Self from a constant u32 array.
pub trait FromU32Array<const L: usize> {
    ///
    /// Creates a primitive type from an big-endian array of [`u32`]'s
    fn from_u32_array(arr: &[u32; L]) -> Self;
}

impl FromU32Array<4> for u128 {
    fn from_u32_array(arr: &[u32; 4]) -> Self {
        let [a, b, c, d] = *arr;

        let a: u128 = (a as u128) << 96;
        let b: u128 = (b as u128) << 64;
        let c: u128 = (c as u128) << 32;
        let d: u128 = d as u128;

        a | b | c | d
    }
}

impl FromU32Array<2> for u64 {
    fn from_u32_array(arr: &[u32; 2]) -> Self {
        let [a, b] = *arr;

        let a: u64 = (a as u64) << 32;
        let b: u64 = b as u64;

        a | b
    }
}