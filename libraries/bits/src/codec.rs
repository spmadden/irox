// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Bits encoding and decoding functions

use crate::{Bits, BitsWrapper, Error, MutBits};

/// Splits the input into two equal sized arrays.
pub const fn array_concat_1(a: [u8; 1], b: [u8; 1]) -> [u8; 2] {
    let [a] = a;
    let [b] = b;
    [a, b]
}
/// Splits the input into two equal sized arrays.
pub const fn array_split_1(a: [u8; 2]) -> ([u8; 1], [u8; 1]) {
    let [a, b] = a;
    ([a], [b])
}
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
/// Converts the value into a constant number of bytes
pub trait ToLEBytes<const N: usize> {
    fn to_le_bytes(&self) -> [u8; N];
}
/// Converts to the value from a constant number of bytes
pub trait FromLEBytes<const N: usize> {
    fn from_le_bytes(bytes: [u8; N]) -> Self;
}

macro_rules! impl_codecs {
    ($ty:ty, $len:literal, $len2:literal, $arrsplit:ident, $arrconcat:ident, $writebe:ident, $readbe:ident, $writele:ident, $readle:ident) => {
        impl ToBEBytes<$len> for $ty {
            fn to_be_bytes(&self) -> [u8; $len] {
                <$ty>::to_be_bytes(*self)
            }
        }
        impl FromBEBytes<$len> for $ty {
            fn from_be_bytes(bytes: [u8; $len]) -> $ty {
                <$ty>::from_be_bytes(bytes)
            }
        }

        impl ToLEBytes<$len> for $ty {
            fn to_le_bytes(&self) -> [u8; $len] {
                <$ty>::to_le_bytes(*self)
            }
        }
        impl FromLEBytes<$len> for $ty {
            fn from_le_bytes(bytes: [u8; $len]) -> Self {
                <$ty>::from_le_bytes(bytes)
            }
        }
        impl WriteToBEBits for $ty {
            fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
                bits.$writebe(*self)?;
                Ok($len)
            }
        }
        impl WriteToLEBits for $ty {
            fn write_le_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
                bits.$writele(*self)?;
                Ok($len)
            }
        }
        impl ReadFromBEBits for $ty {
            fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
                inp.$readbe()
            }
        }
        impl ReadFromLEBits for $ty {
            fn read_from_le_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
                inp.$readle()
            }
        }

        impl ToBEBytes<$len2> for [$ty; 2] {
            fn to_be_bytes(&self) -> [u8; $len2] {
                let [a, b] = *self;
                $arrconcat(<$ty>::to_be_bytes(a), <$ty>::to_be_bytes(b))
            }
        }
        impl ToLEBytes<$len2> for [$ty; 2] {
            fn to_le_bytes(&self) -> [u8; $len2] {
                let [a, b] = *self;
                $arrconcat(<$ty>::to_le_bytes(a), <$ty>::to_le_bytes(b))
            }
        }
        impl FromBEBytes<$len2> for [$ty; 2] {
            fn from_be_bytes(bytes: [u8; $len2]) -> Self {
                let (a, b) = $arrsplit(bytes);
                [<$ty>::from_be_bytes(a), <$ty>::from_be_bytes(b)]
            }
        }
        impl FromLEBytes<$len2> for [$ty; 2] {
            fn from_le_bytes(bytes: [u8; $len2]) -> Self {
                let (a, b) = $arrsplit(bytes);
                [<$ty>::from_le_bytes(a), <$ty>::from_le_bytes(b)]
            }
        }
    };
}
impl_codecs!(
    u8,
    1,
    2,
    array_split_1,
    array_concat_1,
    write_u8,
    read_u8,
    write_u8,
    read_u8
);
impl_codecs!(
    i8,
    1,
    2,
    array_split_1,
    array_concat_1,
    write_i8,
    read_i8,
    write_i8,
    read_i8
);
impl_codecs!(
    u16,
    2,
    4,
    array_split_2,
    array_concat_2,
    write_be_u16,
    read_be_u16,
    write_le_u16,
    read_le_u16
);
impl_codecs!(
    i16,
    2,
    4,
    array_split_2,
    array_concat_2,
    write_be_i16,
    read_be_i16,
    write_le_i16,
    read_le_i16
);
impl_codecs!(
    u32,
    4,
    8,
    array_split_4,
    array_concat_4,
    write_be_u32,
    read_be_u32,
    write_le_u32,
    read_le_u32
);
impl_codecs!(
    i32,
    4,
    8,
    array_split_4,
    array_concat_4,
    write_be_i32,
    read_be_i32,
    write_le_i32,
    read_le_i32
);
impl_codecs!(
    f32,
    4,
    8,
    array_split_4,
    array_concat_4,
    write_be_f32,
    read_be_f32,
    write_le_f32,
    read_le_f32
);
impl_codecs!(
    u64,
    8,
    16,
    array_split_8,
    array_concat_8,
    write_be_u64,
    read_be_u64,
    write_le_u64,
    read_le_u64
);
impl_codecs!(
    i64,
    8,
    16,
    array_split_8,
    array_concat_8,
    write_be_i64,
    read_be_i64,
    write_le_i64,
    read_le_i64
);
impl_codecs!(
    f64,
    8,
    16,
    array_split_8,
    array_concat_8,
    write_be_f64,
    read_be_f64,
    write_le_f64,
    read_le_f64
);
impl_codecs!(
    u128,
    16,
    32,
    array_split_16,
    array_concat_16,
    write_be_u128,
    read_le_u128,
    write_le_u128,
    read_le_u128
);
impl_codecs!(
    i128,
    16,
    32,
    array_split_16,
    array_concat_16,
    write_be_i128,
    read_le_i128,
    write_le_i128,
    read_le_i128
);

macro_rules! impl_large_array {
    ($ty:ty, $len:literal, $len2:literal, $writebe:ident, $writele:ident, $nextbe:ident, $nextle:ident) => {
        impl ToBEBytes<$len2> for [$ty; $len] {
            fn to_be_bytes(&self) -> [u8; $len2] {
                let mut out = [0u8; $len2];
                let mut wr = BitsWrapper::Owned(out.as_mut_slice());
                for v in self {
                    let _ = wr.$writebe(*v);
                }
                out
            }
        }
        impl ToLEBytes<$len2> for [$ty; $len] {
            fn to_le_bytes(&self) -> [u8; $len2] {
                let mut out = [0u8; $len2];
                let mut wr = BitsWrapper::Owned(out.as_mut_slice());
                for v in self {
                    let _ = wr.$writele(*v);
                }
                out
            }
        }
        impl FromBEBytes<$len2> for [$ty; $len] {
            fn from_be_bytes(bytes: [u8; $len2]) -> Self {
                let mut out = [0; $len];
                let mut rd = BitsWrapper::Owned(bytes.as_ref());
                for v in out.iter_mut() {
                    let Ok(Some(a)) = rd.$nextbe() else {
                        break;
                    };
                    *v = a;
                }
                out
            }
        }
        impl FromLEBytes<$len2> for [$ty; $len] {
            fn from_le_bytes(bytes: [u8; $len2]) -> Self {
                let mut out = [0; $len];
                let mut rd = BitsWrapper::Owned(bytes.as_ref());
                for v in out.iter_mut() {
                    let Ok(Some(a)) = rd.$nextle() else {
                        break;
                    };
                    *v = a;
                }
                out
            }
        }
    };
}
impl_large_array!(
    u32,
    3,
    12,
    write_be_u32,
    write_le_u32,
    next_be_u32,
    next_le_u32
); // 96 bit
impl_large_array!(
    u32,
    4,
    16,
    write_be_u32,
    write_le_u32,
    next_be_u32,
    next_le_u32
); // 128 bit
impl_large_array!(
    u32,
    5,
    20,
    write_be_u32,
    write_le_u32,
    next_be_u32,
    next_le_u32
); // 160 bit
impl_large_array!(
    u32,
    6,
    24,
    write_be_u32,
    write_le_u32,
    next_be_u32,
    next_le_u32
); // 192 bit
impl_large_array!(
    u32,
    7,
    28,
    write_be_u32,
    write_le_u32,
    next_be_u32,
    next_le_u32
); // 224 bit
impl_large_array!(
    u32,
    8,
    32,
    write_be_u32,
    write_le_u32,
    next_be_u32,
    next_le_u32
); // 256 bit
impl_large_array!(
    u32,
    9,
    36,
    write_be_u32,
    write_le_u32,
    next_be_u32,
    next_le_u32
); // 288 bit
impl_large_array!(
    u32,
    10,
    40,
    write_be_u32,
    write_le_u32,
    next_be_u32,
    next_le_u32
); // 320 bit
impl_large_array!(
    u32,
    11,
    44,
    write_be_u32,
    write_le_u32,
    next_be_u32,
    next_le_u32
); // 352 bit
impl_large_array!(
    u32,
    12,
    48,
    write_be_u32,
    write_le_u32,
    next_be_u32,
    next_le_u32
); // 384 bit

impl_large_array!(
    u64,
    3,
    24,
    write_be_u64,
    write_le_u64,
    next_be_u64,
    next_le_u64
); // 192 bit
impl_large_array!(
    u64,
    4,
    32,
    write_be_u64,
    write_le_u64,
    next_be_u64,
    next_le_u64
); // 256 bit
impl_large_array!(
    u64,
    5,
    40,
    write_be_u64,
    write_le_u64,
    next_be_u64,
    next_le_u64
); // 320 bit
impl_large_array!(
    u64,
    6,
    48,
    write_be_u64,
    write_le_u64,
    next_be_u64,
    next_le_u64
); // 384 bit
impl_large_array!(
    u64,
    7,
    56,
    write_be_u64,
    write_le_u64,
    next_be_u64,
    next_le_u64
); // 448 bit
impl_large_array!(
    u64,
    8,
    64,
    write_be_u64,
    write_le_u64,
    next_be_u64,
    next_le_u64
); // 512 bit
impl_large_array!(
    u64,
    9,
    72,
    write_be_u64,
    write_le_u64,
    next_be_u64,
    next_le_u64
); // 576 bit
impl_large_array!(
    u64,
    10,
    80,
    write_be_u64,
    write_le_u64,
    next_be_u64,
    next_le_u64
); // 640 bit
impl_large_array!(
    u64,
    11,
    88,
    write_be_u64,
    write_le_u64,
    next_be_u64,
    next_le_u64
); // 704 bit
impl_large_array!(
    u64,
    12,
    96,
    write_be_u64,
    write_le_u64,
    next_be_u64,
    next_le_u64
); // 768 bit

impl_large_array!(
    u128,
    3,
    48,
    write_be_u128,
    write_le_u128,
    next_be_u128,
    next_le_u128
); // 384 bit
impl_large_array!(
    u128,
    4,
    64,
    write_be_u128,
    write_le_u128,
    next_be_u128,
    next_le_u128
); // 512 bit
impl_large_array!(
    u128,
    5,
    80,
    write_be_u128,
    write_le_u128,
    next_be_u128,
    next_le_u128
); // 640 bit
impl_large_array!(
    u128,
    6,
    96,
    write_be_u128,
    write_le_u128,
    next_be_u128,
    next_le_u128
); // 768 bit
impl_large_array!(
    u128,
    7,
    112,
    write_be_u128,
    write_le_u128,
    next_be_u128,
    next_le_u128
); // 896 bit
impl_large_array!(
    u128,
    8,
    128,
    write_be_u128,
    write_le_u128,
    next_be_u128,
    next_le_u128
); // 1024 bit
impl_large_array!(
    u128,
    9,
    144,
    write_be_u128,
    write_le_u128,
    next_be_u128,
    next_le_u128
); // 1152 bit
impl_large_array!(
    u128,
    10,
    160,
    write_be_u128,
    write_le_u128,
    next_be_u128,
    next_le_u128
); // 1280 bit
impl_large_array!(
    u128,
    11,
    176,
    write_be_u128,
    write_le_u128,
    next_be_u128,
    next_le_u128
); // 1408 bit
impl_large_array!(
    u128,
    12,
    192,
    write_be_u128,
    write_le_u128,
    next_be_u128,
    next_le_u128
); // 1536 bit

/// Writes 'self' to the provided [`MutBits`] impl in big endian order.
pub trait WriteToBEBits {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error>;
}
/// Writes 'self' to the provided [`MutBits`] impl in little endian order.
pub trait WriteToLEBits {
    fn write_le_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error>;
}

pub trait ReadFromBEBits: Sized {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error>;
}
pub trait ReadFromLEBits: Sized {
    fn read_from_le_bits<T: Bits>(inp: &mut T) -> Result<Self, Error>;
}
impl<const N: usize> ReadFromBEBits for [u8; N] {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        let mut out = [0; N];
        inp.read_all_into(&mut out.as_mut_slice())?;
        Ok(out)
    }
}
impl<const N: usize> ReadFromLEBits for [u8; N] {
    fn read_from_le_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
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
impl WriteToLEBits for &str {
    fn write_le_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let len = self.len() as u32;
        bits.write_le_u32(len)?;
        bits.write_all_bytes(self.as_bytes())?;

        Ok((len + 4) as usize)
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
    impl ReadFromLEBits for alloc::string::String {
        fn read_from_le_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
            let len = inp.read_le_u32()?;
            inp.read_str_sized_lossy(len as usize)
        }
    }
    impl WriteToBEBits for alloc::sync::Arc<alloc::string::String> {
        fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
            bits.write_str_u32_blob(self.as_str())
        }
    }
}
