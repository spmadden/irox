// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::buf::Buffer;
use crate::buf::FixedBuf;
use crate::{cfg_feature_alloc, IntegerValue};
use irox_bits::{Bits, BitsError, Error, MutBits};

macro_rules! round {
    ($val:expr) => {{
        *$val >>= 7;
        let a = ((*$val & 0x7F) | 0x80) as u8;
        a
    }};
}
macro_rules! one_byte_mask {
    () => {
        0x7F
    };
}
macro_rules! two_byte_mask {
    () => {
        0x3FFF
    };
}
macro_rules! three_byte_mask {
    () => {
        0x1F_FFFF
    };
}
macro_rules! four_byte_mask {
    () => {
        0xFFF_FFFF
    };
}
macro_rules! five_byte_mask {
    () => {
        0x7_FFFF_FFFF
    };
}
macro_rules! six_byte_mask {
    () => {
        0x3FF_FFFF_FFFF
    };
}
macro_rules! seven_byte_mask {
    () => {
        0x1_FFFF_FFFF_FFFF
    };
}
macro_rules! eight_byte_mask {
    () => {
        0xFF_FFFF_FFFF_FFFF
    };
}
macro_rules! nine_byte_mask {
    () => {
        0x7FFF_FFFF_FFFF_FFFF
    };
}

///
///
/// ```text
///  7       0
/// |--------|
///  01111111
///
/// ```
pub fn encode_7bits(val: u8) -> [u8; 1] {
    [val & 0x7F]
}
///
///
/// ```text
///  7       0
/// |--------|
///  XXXXXXXX
///
/// 15        7       0
/// |--------|--------|
///  1000000X 0XXXXXXX
///
/// ```
pub fn encode_8bits(val: u8) -> [u8; 2] {
    let upper = (val & 0x80) >> 7;
    [0x80 | upper, val & 0x7F]
}

///
/// 14 bits = `0x3FFF`
/// ```text
/// 15        7       0
/// |--------|--------|
///  00111111 10000000
///
/// ```
pub fn encode_14bits(mut val: u16) -> [u8; 2] {
    let b = (val & 0x7F) as u8;
    let a = round!(&mut val);
    [a, b]
}

///
/// 16 bits => `0xFFFF`
/// ```text
/// 15        7       0
/// |--------|--------|
///  22111111 10000000
///
/// ```
pub fn encode_16bits(mut val: u16) -> [u8; 3] {
    let c = (val & 0x7F) as u8;
    let b = round!(&mut val);
    let a = round!(&mut val);
    [a, b, c]
}
///
/// 21 bits => `0x1F_FFFF`
/// ```text
/// 24       15        7       0
/// |--------|--------|--------|
///  33322222 22111111 10000000
/// |--------|--------|--------|
///  12222222 11111111 00000000
/// ```
pub fn encode_21bits(mut val: u32) -> [u8; 3] {
    let c = (val & 0x7F) as u8;
    let b = round!(&mut val);
    let a = round!(&mut val);
    [a, b, c]
}
///
/// 28 bits => `0xFFF_FFFF`
/// ```text
/// 31       23       15        7       0
/// |--------|--------|--------|--------|
///  44443333 33322222 22111111 10000000
/// |--------|--------|--------|--------|
///  13333333 12222222 11111111 00000000
///
/// ```
pub fn encode_28bits(mut val: u32) -> [u8; 4] {
    let d = (val & 0x7F) as u8;
    let c = round!(&mut val);
    let b = round!(&mut val);
    let a = round!(&mut val);
    [a, b, c, d]
}

///
/// 32 bits => `0xFFFF_FFFF`
/// ```text
/// 31       23       15        7       0
/// |--------|--------|--------|--------|
///  44443333 33322222 22111111 10000000
/// |--------|--------|--------|--------|
///  13333333 12222222 11111111 00000000
///
/// 63       55       47       39       31
/// |--------|--------|--------|--------|
///                    66666655 55555444
/// |--------|--------|--------|--------|
///                             14444444
/// ```
pub fn encode_32bits(mut val: u32) -> [u8; 5] {
    let e = (val & 0x7F) as u8;
    let d = round!(&mut val);
    let c = round!(&mut val);
    let b = round!(&mut val);
    let a = round!(&mut val);
    [a, b, c, d, e]
}

///
/// 35 bits => `0x7_FFFF_FFFF`
/// ```text
/// 31       23       15        7       0
/// |--------|--------|--------|--------|
///  44443333 33322222 22111111 10000000
/// |--------|--------|--------|--------|
///  13333333 12222222 11111111 00000000
///
/// 63       55       47       39       31
/// |--------|--------|--------|--------|
///  98888888 77777776 66666655 55555444
/// |--------|--------|--------|--------|
///  17777777 16666666 15555555 14444444
/// ```
pub fn encode_35bits(mut val: u64) -> [u8; 5] {
    let e = (val & 0x7F) as u8;
    let d = round!(&mut val);
    let c = round!(&mut val);
    let b = round!(&mut val);
    let a = round!(&mut val);
    [a, b, c, d, e]
}

///
/// 42 bits => `0x3FF_FFFF_FFFF`
/// ```text
/// 31       23       15        7       0
/// |--------|--------|--------|--------|
///  44443333 33322222 22111111 10000000
/// |--------|--------|--------|--------|
///  13333333 12222222 11111111 00000000
///
/// 63       55       47       39       31
/// |--------|--------|--------|--------|
///  98888888 77777776 66666655 55555444
/// |--------|--------|--------|--------|
///  17777777 16666666 15555555 14444444
/// ```
pub fn encode_42bits(mut val: u64) -> [u8; 6] {
    let f = (val & 0x7F) as u8;
    let e = round!(&mut val);
    let d = round!(&mut val);
    let c = round!(&mut val);
    let b = round!(&mut val);
    let a = round!(&mut val);
    [a, b, c, d, e, f]
}

///
/// 49 bits => `0x1_FFFF_FFFF_FFFF`
/// ```text
/// 31       23       15        7       0
/// |--------|--------|--------|--------|
///  44443333 33322222 22111111 10000000
/// |--------|--------|--------|--------|
///  13333333 12222222 11111111 00000000
///
/// 63       55       47       39       32
/// |--------|--------|--------|--------|
///  98888888 77777776 66666655 55555444
/// |--------|--------|--------|--------|
///  17777777 16666666 15555555 14444444
/// ```
pub fn encode_49bits(mut val: u64) -> [u8; 7] {
    let g = (val & 0x7F) as u8;
    let f = round!(&mut val);
    let e = round!(&mut val);
    let d = round!(&mut val);
    let c = round!(&mut val);
    let b = round!(&mut val);
    let a = round!(&mut val);
    [a, b, c, d, e, f, g]
}
///
/// 56 bits => `0xFF_FFFF_FFFF_FFFF`
/// ```text
/// 32       23       15        7       0
/// |--------|--------|--------|--------|
///  44443333 33322222 22111111 10000000
/// |--------|--------|--------|--------|
///  13333333 12222222 11111111 00000000
///
/// 63       55       47       39       31
/// |--------|--------|--------|--------|
///  98888888 77777776 66666655 55555444
/// |--------|--------|--------|--------|
///  17777777 16666666 15555555 14444444
/// ```
pub fn encode_56bits(mut val: u64) -> [u8; 8] {
    let h = (val & 0x7F) as u8;
    let g = round!(&mut val);
    let f = round!(&mut val);
    let e = round!(&mut val);
    let d = round!(&mut val);
    let c = round!(&mut val);
    let b = round!(&mut val);
    let a = round!(&mut val);
    [a, b, c, d, e, f, g, h]
}
///
/// 63 bits => `0x7FFF_FFFF_FFFF_FFFF`
/// ```text
/// 32       23       15        7       0
/// |--------|--------|--------|--------|
///  44443333 33322222 22111111 10000000
/// |--------|--------|--------|--------|
///  13333333 12222222 11111111 00000000
///
/// 63       55       47       39       31
/// |--------|--------|--------|--------|
///  98888888 77777776 66666655 55555444
/// |--------|--------|--------|--------|
///  17777777 16666666 15555555 14444444
/// ```
pub fn encode_63bits(mut val: u64) -> [u8; 9] {
    let i = (val & 0x7F) as u8;
    let h = round!(&mut val);
    let g = round!(&mut val);
    let f = round!(&mut val);
    let e = round!(&mut val);
    let d = round!(&mut val);
    let c = round!(&mut val);
    let b = round!(&mut val);
    let a = round!(&mut val);
    [a, b, c, d, e, f, g, h, i]
}

///
/// 64 bits => 0xFFFF_FFFF_FFFF_FFFF
/// ```text
/// 32       23       15        7       0
/// |--------|--------|--------|--------|
///  44443333 33322222 22111111 10000000
/// |--------|--------|--------|--------|
///  13333333 12222222 11111111 00000000
///
/// 63       55       47       39       31
/// |--------|--------|--------|--------|
///  98888888 77777776 66666655 55555444
/// |--------|--------|--------|--------|
///  17777777 16666666 15555555 14444444
/// ```
pub fn encode_64bits(mut val: u64) -> [u8; 10] {
    let j = (val & 0x7F) as u8;
    let i = round!(&mut val);
    let h = round!(&mut val);
    let g = round!(&mut val);
    let f = round!(&mut val);
    let e = round!(&mut val);
    let d = round!(&mut val);
    let c = round!(&mut val);
    let b = round!(&mut val);
    let a = round!(&mut val);
    [a, b, c, d, e, f, g, h, i, j]
}

///
/// 70 bits => 0x3F_FFFF_FFFF_FFFF_FFFF
/// ```text
///      8        7        6        5        4        3        2        1
/// 64       56       48       40       32       24       16        8       0
/// |--------|--------|--------|--------|--------|--------|--------|--------|
///  98888888 77777776 66666655 55555444 44443333 33322222 22111111 10000000
/// |--------|--------|--------|--------|--------|--------|--------|--------|
///  17777777 16666666 15555555 14444444 13333333 12222222 11111111 00000000
///
///     16       15       14       13       12        11       10       9
/// 128     120      112      104       96       88       80       72       64
/// |--------|--------|--------|--------|--------|--------|--------|--------|
///  IIHHHHHH HGGGGGGG FFFFFFFE EEEEEEDD DDDDDCCC CCCCBBBB BBBAAAAA AA999999
/// |--------|--------|--------|--------|--------|--------|--------|--------|
///  1FFFFFFF 1EEEEEEE 1DDDDDDD 1CCCCCCC 1BBBBBBB 1AAAAAAA 19999999 18888888
///
///      19       18       17
/// 152     144      136      128
/// |--------|--------|--------|
///  100000II 1HHHHHHH 1GGGGGGG
/// |--------|--------|--------|
///
/// ```
pub fn encode_u128bits(mut val: u128) -> FixedBuf<19, u8> {
    let mut out = FixedBuf::<19, u8>::new();
    loop {
        if val == 0 && !out.is_empty() {
            break;
        }
        let mut i = (val & 0x7F) as u8;
        val >>= 7;
        if val != 0 {
            i |= 0x80;
        }
        let _ = out.push(i);
    }
    out
}

pub fn encode_vbyte_to<T: MutBits + ?Sized>(val: u128, out: &mut T) -> Result<usize, BitsError> {
    encode_u128bits(val).write_to(out)
}

macro_rules! zigzag_impl {
    ($id:ident,$un:ident,$sig:ty,$usig:ty,$len:literal) => {
        pub fn $id(n: $sig) -> $usig {
            ((n << 1) ^ (n >> ($len - 1))) as $usig
        }
        pub fn $un(n: $usig) -> $sig {
            let v = (n & 0x01) as $sig;
            let v = (n >> 1) as $sig ^ -v;
            v as $sig
        }
        impl ZigZag for $sig {
            type Output = $usig;
            fn zigzag(self) -> $usig {
                $id(self)
            }
        }
        impl ZagZig for $usig {
            type Output = $sig;
            fn zagzig(self) -> Self::Output {
                $un(self)
            }
        }
    };
}
zigzag_impl!(zigzag_i8, zagzig_u8, i8, u8, 8);
zigzag_impl!(zigzag_i16, zagzig_u16, i16, u16, 16);
zigzag_impl!(zigzag_i32, zagzig_u32, i32, u32, 32);
zigzag_impl!(zigzag_i64, zagzig_u64, i64, u64, 64);
zigzag_impl!(zigzag_i128, zagzig_u128, i128, u128, 128);

pub trait ZigZag {
    type Output;
    fn zigzag(self) -> Self::Output;
}
pub trait ZagZig {
    type Output;
    fn zagzig(self) -> Self::Output;
}
pub fn encode_le_integer_to<T: MutBits + ?Sized>(
    val: IntegerValue,
    out: &mut T,
) -> Result<usize, BitsError> {
    encode_integer_to(val.to_le(), out)
}
pub fn encode_integer_to<T: MutBits + ?Sized>(
    val: IntegerValue,
    out: &mut T,
) -> Result<usize, BitsError> {
    match val {
        IntegerValue::U8(v) => {
            if v <= one_byte_mask!() {
                out.write_all_bytes(&encode_7bits(v))?;
                Ok(1)
            } else {
                out.write_all_bytes(&encode_8bits(v))?;
                Ok(2)
            }
        }
        IntegerValue::U16(v) => {
            if v <= one_byte_mask!() {
                out.write_all_bytes(&encode_7bits(v as u8))?;
                Ok(1)
            } else if v <= two_byte_mask!() {
                out.write_all_bytes(&encode_14bits(v))?;
                Ok(2)
            } else {
                out.write_all_bytes(&encode_16bits(v))?;
                Ok(3)
            }
        }
        IntegerValue::U32(v) => {
            if v <= one_byte_mask!() {
                out.write_all_bytes(&encode_7bits(v as u8))?;
                Ok(1)
            } else if v <= two_byte_mask!() {
                out.write_all_bytes(&encode_14bits(v as u16))?;
                Ok(2)
            } else if v <= three_byte_mask!() {
                out.write_all_bytes(&encode_21bits(v))?;
                Ok(3)
            } else if v <= four_byte_mask!() {
                out.write_all_bytes(&encode_28bits(v))?;
                Ok(4)
            } else {
                out.write_all_bytes(&encode_32bits(v))?;
                Ok(5)
            }
        }
        IntegerValue::U64(v) => {
            if v <= one_byte_mask!() {
                out.write_all_bytes(&encode_7bits(v as u8))?;
                Ok(1)
            } else if v <= two_byte_mask!() {
                out.write_all_bytes(&encode_14bits(v as u16))?;
                Ok(2)
            } else if v <= three_byte_mask!() {
                out.write_all_bytes(&encode_21bits(v as u32))?;
                Ok(3)
            } else if v <= four_byte_mask!() {
                out.write_all_bytes(&encode_28bits(v as u32))?;
                Ok(4)
            } else if v <= five_byte_mask!() {
                out.write_all_bytes(&encode_35bits(v))?;
                Ok(5)
            } else if v <= six_byte_mask!() {
                out.write_all_bytes(&encode_42bits(v))?;
                Ok(6)
            } else if v <= seven_byte_mask!() {
                out.write_all_bytes(&encode_49bits(v))?;
                Ok(7)
            } else if v <= eight_byte_mask!() {
                out.write_all_bytes(&encode_56bits(v))?;
                Ok(8)
            } else if v <= nine_byte_mask!() {
                out.write_all_bytes(&encode_63bits(v))?;
                Ok(9)
            } else {
                out.write_all_bytes(&encode_64bits(v))?;
                Ok(10)
            }
        }
        IntegerValue::U128(v) => encode_u128bits(v).write_to(out),
        IntegerValue::I8(v) => zigzag_i8(v).encode_vbyte_to(out),
        IntegerValue::I16(v) => zigzag_i16(v).encode_vbyte_to(out),
        IntegerValue::I32(v) => zigzag_i32(v).encode_vbyte_to(out),
        IntegerValue::I64(v) => zigzag_i64(v).encode_vbyte_to(out),
        IntegerValue::I128(v) => zigzag_i128(v).encode_vbyte_to(out),
    }
}
pub trait EncodeVByteTo {
    fn encode_vbyte_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError>;
}
impl EncodeVByteTo for u128 {
    fn encode_vbyte_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        encode_integer_to(IntegerValue::U128(*self), out)
    }
}
impl EncodeVByteTo for i128 {
    fn encode_vbyte_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        encode_integer_to(IntegerValue::I128(*self), out)
    }
}
impl EncodeVByteTo for u64 {
    fn encode_vbyte_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        encode_integer_to(IntegerValue::U64(*self), out)
    }
}
impl EncodeVByteTo for i64 {
    fn encode_vbyte_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        encode_integer_to(IntegerValue::I64(*self), out)
    }
}
impl EncodeVByteTo for u32 {
    fn encode_vbyte_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        encode_integer_to(IntegerValue::U32(*self), out)
    }
}
impl EncodeVByteTo for i32 {
    fn encode_vbyte_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        encode_integer_to(IntegerValue::I32(*self), out)
    }
}
impl EncodeVByteTo for u16 {
    fn encode_vbyte_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        encode_integer_to(IntegerValue::U16(*self), out)
    }
}
impl EncodeVByteTo for i16 {
    fn encode_vbyte_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        encode_integer_to(IntegerValue::I16(*self), out)
    }
}
impl EncodeVByteTo for u8 {
    fn encode_vbyte_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        encode_integer_to(IntegerValue::U8(*self), out)
    }
}
impl EncodeVByteTo for i8 {
    fn encode_vbyte_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        encode_integer_to(IntegerValue::I8(*self), out)
    }
}
cfg_feature_alloc! {
    pub fn encode_integer(val: IntegerValue) -> alloc::boxed::Box<[u8]> {
        use alloc::boxed::Box;
        match val {
            IntegerValue::U8(v) => {
                if v <= one_byte_mask!() {
                    Box::new(encode_7bits(v))
                } else {
                    Box::new(encode_8bits(v))
                }
            }
            IntegerValue::U16(v) => {
                if v <= one_byte_mask!() {
                    Box::new(encode_7bits(v as u8))
                } else if v <= two_byte_mask!() {
                    Box::new(encode_14bits(v))
                } else {
                    Box::new(encode_16bits(v))
                }
            }
            IntegerValue::U32(v) => {
                if v <= one_byte_mask!() {
                    Box::new(encode_7bits(v as u8))
                } else if v <= two_byte_mask!() {
                    Box::new(encode_14bits(v as u16))
                } else if v <= three_byte_mask!() {
                    Box::new(encode_21bits(v))
                } else if v <= four_byte_mask!() {
                    Box::new(encode_28bits(v))
                } else {
                    Box::new(encode_32bits(v))
                }
            }
            IntegerValue::U64(v) => {
                if v <= one_byte_mask!() {
                    Box::new(encode_7bits(v as u8))
                } else if v <= two_byte_mask!() {
                    Box::new(encode_14bits(v as u16))
                } else if v <= three_byte_mask!() {
                    Box::new(encode_21bits(v as u32))
                } else if v <= four_byte_mask!() {
                    Box::new(encode_28bits(v as u32))
                } else if v <= five_byte_mask!() {
                    Box::new(encode_35bits(v))
                } else if v <= six_byte_mask!() {
                    Box::new(encode_42bits(v))
                } else if v <= seven_byte_mask!() {
                    Box::new(encode_49bits(v))
                } else if v <= eight_byte_mask!() {
                    Box::new(encode_56bits(v))
                } else if v <= nine_byte_mask!() {
                    Box::new(encode_63bits(v))
                } else {
                    Box::new(encode_64bits(v))
                }
            }
            // IntegerValue::U128(_) => {}
            // IntegerValue::I8(_) => {}
            // IntegerValue::I16(_) => {}
            // IntegerValue::I32(_) => {}
            // IntegerValue::I64(_) => {}
            // IntegerValue::I128(_) => {}
            _ => {
                todo!()
            }
        }
    }
}

pub const fn resultant_length(value: IntegerValue) -> u8 {
    let v = value.to_be_u64();
    match v {
        0x0000_0000_0000_0000..=0x0000_0000_0000_007F => 1,
        0x0000_0000_0000_0080..=0x0000_0000_0000_3FFF => 2,
        0x0000_0000_0000_4000..=0x0000_0000_001F_FFFF => 3,
        0x0000_0000_0020_0000..=0x0000_0000_0FFF_FFFF => 4,
        0x0000_0000_1000_0000..=0x0000_0007_FFFF_FFFF => 5,
        0x0000_0008_0000_0000..=0x0000_03FF_FFFF_FFFF => 6,
        0x0000_0400_0000_0000..=0x0001_FFFF_FFFF_FFFF => 7,
        0x0002_0000_0000_0000..=0x00FF_FFFF_FFFF_FFFF => 8,
        0x0100_0000_0000_0000..=0x7FFF_FFFF_FFFF_FFFF => 9,
        _ => 10,
    }
}
cfg_feature_alloc! {
    extern crate alloc;
    pub trait EncodeVByte {
        fn encode_vbyte(&self) -> alloc::boxed::Box<[u8]>;
    }
}
pub trait EncodeVByteLength {
    fn vbyte_length(&self) -> u8;
}
impl<T> EncodeVByteLength for T
where
    T: Into<IntegerValue> + Copy,
{
    fn vbyte_length(&self) -> u8 {
        resultant_length(Into::<IntegerValue>::into(*self))
    }
}
cfg_feature_alloc! {
    macro_rules! impl_encode {
        ($typ:ty) => {
            impl crate::codec::vbyte::EncodeVByte for $typ {
                fn encode_vbyte(&self) -> alloc::boxed::Box<[u8]> {
                    crate::codec::vbyte::encode_integer(self.into())
                }
            }
            // impl EncodeVByte for [$typ] {
            //     fn encode_vbyte(&self) -> Box<[u8]> {
            //         crate::vbyte::encode(self.into())
            //     }
            // }
            impl EncodeVByte for &$typ {
                fn encode_vbyte(&self) -> alloc::boxed::Box<[u8]> {
                    let v: IntegerValue = (*self).into();
                    crate::codec::vbyte::encode_integer(v)
                }
            }
            // impl EncodeVByte for &mut $typ {
            //     fn encode_vbyte(&self) -> Box<[u8]> {
            //         let v : IntegerValue = (*self).into();
            //         crate::codec::vbyte::encode_integer(v)
            //     }
            // }
        };
    }

    impl_encode!(u8);
    impl_encode!(i8);
    impl_encode!(u16);
    impl_encode!(i16);
    impl_encode!(u32);
    impl_encode!(i32);
    impl_encode!(u64);
    impl_encode!(i64);
}

pub trait DecodeVByte {
    fn decode_vbyte(&mut self) -> Result<u128, Error>;
}

pub fn decode_vbyte<T: Bits>(inp: &mut T) -> Result<u128, Error> {
    let mut out: u128 = 0;
    while let Some(val) = inp.next_u8()? {
        let v = (val & 0x7F) as u128;
        out = (out << 7) | v;
        if val & 0x80 == 0 {
            break;
        }
    }
    Ok(out)
}

impl<T: Bits> DecodeVByte for T {
    fn decode_vbyte(&mut self) -> Result<u128, Error> {
        decode_vbyte(self)
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use crate::codec::vbyte::{DecodeVByte, EncodeVByte};
    use crate::codec::EncodeVByteLength;
    use irox_bits::Error;

    #[test]
    pub fn test_encode() {
        assert_eq!(0x00u8.encode_vbyte().as_ref(), &[0x00]);
        assert_eq!(0x7Fu8.encode_vbyte().as_ref(), &[0x7F]);
        assert_eq!(0x80u8.encode_vbyte().as_ref(), &[0x81, 0x00]);
        assert_eq!(0x2000u16.encode_vbyte().as_ref(), &[0xC0, 0x00]);
        assert_eq!(0x3FFFu16.encode_vbyte().as_ref(), &[0xFF, 0x7F]);
        assert_eq!(0x4000u16.encode_vbyte().as_ref(), &[0x81, 0x80, 0x00]);
        assert_eq!(0x1F_FFFFu32.encode_vbyte().as_ref(), &[0xFF, 0xFF, 0x7F]);
        assert_eq!(
            0x20_0000u32.encode_vbyte().as_ref(),
            &[0x81, 0x80, 0x80, 0x00]
        );
        assert_eq!(
            0x800_0000u32.encode_vbyte().as_ref(),
            &[0xC0, 0x80, 0x80, 0x00]
        );
        assert_eq!(
            0xFFF_FFFFu32.encode_vbyte().as_ref(),
            &[0xFF, 0xFF, 0xFF, 0x7F]
        );
    }

    #[test]
    pub fn test_decode() -> Result<(), Error> {
        assert_eq_hex!(0x0, [0x0].as_ref().decode_vbyte()?);
        assert_eq_hex!(0x7F, [0x7F].as_ref().decode_vbyte()?);
        assert_eq_hex!(0x80, [0x81, 0x00].as_ref().decode_vbyte()?);
        assert_eq_hex!(0x2000, [0xC0, 0x00].as_ref().decode_vbyte()?);
        assert_eq_hex!(0x3FFF, [0xFF, 0x7F].as_ref().decode_vbyte()?);
        assert_eq_hex!(0x4000, [0x81, 0x80, 0x00].as_ref().decode_vbyte()?);
        assert_eq_hex!(0x1F_FFFF, [0xFF, 0xFF, 0x7F].as_ref().decode_vbyte()?);
        assert_eq_hex!(
            0x800_0000,
            [0xC0, 0x80, 0x80, 0x00].as_ref().decode_vbyte()?
        );
        assert_eq_hex!(
            0xFFF_FFFF,
            [0xFF, 0xFF, 0xFF, 0x7F].as_ref().decode_vbyte()?
        );

        Ok(())
    }

    #[test]
    pub fn test_vbyte_length() -> Result<(), Error> {
        assert_eq!(2, 0xCC.vbyte_length());
        assert_eq!(3, 0xAAAA.vbyte_length());
        Ok(())
    }
}
