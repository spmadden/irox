// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::buf::Buffer;
use crate::buf::FixedBuf;
use crate::IntegerValue;
use alloc::boxed::Box;

macro_rules! round {
    ($val:ident) => {{
        let val = $val >> 7;
        let a = ((val & 0x7F) | 0x80) as u8;
        (a, val)
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
pub fn encode_14bits(val: u16) -> [u8; 2] {
    let b = (val & 0x7F) as u8;
    let (a, _) = round!(val);
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
pub fn encode_16bits(val: u16) -> [u8; 3] {
    let c = (val & 0x7F) as u8;
    let (b, val) = round!(val);
    let (a, _) = round!(val);
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
pub fn encode_21bits(val: u32) -> [u8; 3] {
    let c = (val & 0x7F) as u8;
    let (b, val) = round!(val);
    let (a, _) = round!(val);
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
pub fn encode_28bits(val: u32) -> [u8; 4] {
    let d = (val & 0x7F) as u8;
    let (c, val) = round!(val);
    let (b, val) = round!(val);
    let (a, _) = round!(val);
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
pub fn encode_32bits(val: u32) -> [u8; 5] {
    let e = (val & 0x7F) as u8;
    let (d, val) = round!(val);
    let (c, val) = round!(val);
    let (b, val) = round!(val);
    let (a, _) = round!(val);
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
pub fn encode_35bits(val: u64) -> [u8; 5] {
    let e = (val & 0x7F) as u8;
    let (d, val) = round!(val);
    let (c, val) = round!(val);
    let (b, val) = round!(val);
    let (a, _) = round!(val);
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
pub fn encode_42bits(val: u64) -> [u8; 6] {
    let f = (val & 0x7F) as u8;
    let (e, val) = round!(val);
    let (d, val) = round!(val);
    let (c, val) = round!(val);
    let (b, val) = round!(val);
    let (a, _) = round!(val);
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
pub fn encode_49bits(val: u64) -> [u8; 7] {
    let g = (val & 0x7F) as u8;
    let (f, val) = round!(val);
    let (e, val) = round!(val);
    let (d, val) = round!(val);
    let (c, val) = round!(val);
    let (b, val) = round!(val);
    let (a, _) = round!(val);
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
pub fn encode_56bits(val: u64) -> [u8; 8] {
    let h = (val & 0x7F) as u8;
    let (g, val) = round!(val);
    let (f, val) = round!(val);
    let (e, val) = round!(val);
    let (d, val) = round!(val);
    let (c, val) = round!(val);
    let (b, val) = round!(val);
    let (a, _) = round!(val);
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
pub fn encode_63bits(val: u64) -> [u8; 9] {
    let i = (val & 0x7F) as u8;
    let (h, val) = round!(val);
    let (g, val) = round!(val);
    let (f, val) = round!(val);
    let (e, val) = round!(val);
    let (d, val) = round!(val);
    let (c, val) = round!(val);
    let (b, val) = round!(val);
    let (a, _) = round!(val);
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
pub fn encode_64bits(val: u64) -> [u8; 10] {
    let j = (val & 0x7F) as u8;
    let (i, val) = round!(val);
    let (h, val) = round!(val);
    let (g, val) = round!(val);
    let (f, val) = round!(val);
    let (e, val) = round!(val);
    let (d, val) = round!(val);
    let (c, val) = round!(val);
    let (b, val) = round!(val);
    let (a, _) = round!(val);
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
pub fn encode_u128bits(val: u128) -> Box<[u8]> {
    let mut out = FixedBuf::<19, u8>::new();
    let j = (val & 0x7F) as u8;
    let _ = out.push(j);
    loop {
        let (i, val) = round!(val);
        let _ = out.push(i);
        if val == 0 {
            break;
        }
    }
    out.into_boxed_slice()
}

pub fn encode_integer(val: IntegerValue) -> Box<[u8]> {
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
pub trait EncodeVByte {
    fn encode_vbyte(&self) -> Box<[u8]>;
}
macro_rules! impl_encode {
    ($typ:ty) => {
        impl crate::codec::vbyte::EncodeVByte for $typ {
            fn encode_vbyte(&self) -> Box<[u8]> {
                crate::codec::vbyte::encode_integer(self.into())
            }
        }
        // impl EncodeVByte for [$typ] {
        //     fn encode_vbyte(&self) -> Box<[u8]> {
        //         crate::vbyte::encode(self.into())
        //     }
        // }
        impl EncodeVByte for &$typ {
            fn encode_vbyte(&self) -> Box<[u8]> {
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

#[cfg(test)]
mod tests {
    use crate::codec::vbyte::EncodeVByte;

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
}
