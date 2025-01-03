// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::buf::FixedU8Buf;
use crate::IntegerValue;
use irox_bits::{Bits, BitsError, Error, MutBits};

///
/// Encodes up to 9 bytes in the sqlite4 varint format.
pub trait EncodeVarintTo {
    ///
    /// Encodes up to 9 bytes in the sqlite4 varint format.  Returns the number of
    /// bytes actually written.
    fn encode_varint_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError>;
}

///
/// Encodes up to 9 bytes in the sqlite4 varint format.  Returns the number of
/// bytes actually written.
pub fn encode_varint_to<T: MutBits + ?Sized>(
    value: IntegerValue,
    out: &mut T,
) -> Result<usize, BitsError> {
    let v = value.to_be_u64();
    Ok(if v <= 0xF0 {
        // 240
        out.write_u8(v as u8)?;
        1
    } else if v <= 0x8EF {
        // 2287
        let a0 = ((v - 240) / 256 + 241) as u8;
        let a1 = ((v - 240) & 0xFF) as u8;
        out.write_all_bytes(&[a0, a1])?;
        2
    } else if v <= 0x108EF {
        // 67823
        let a0 = 249u8;
        let a1 = ((v - 2288) / 256) as u8;
        let a2 = ((v - 2288) & 0xFF) as u8;
        out.write_all_bytes(&[a0, a1, a2])?;
        3
    } else if v <= 0x00FF_FFFF {
        let v = v as u32 | 0xFA000000;
        out.write_be_u32(v)?;
        4
    } else if v <= 0xFFFF_FFFF {
        let v = v as u32;
        out.write_u8(250)?;
        out.write_be_u32(v)?;
        5
    } else if v <= 0xFF_FFFF_FFFF {
        let [_, _, _, a1, a2, a3, a4, a5] = v.to_be_bytes();
        out.write_all_bytes(&[252, a1, a2, a3, a4, a5])?;
        6
    } else if v <= 0xFFFF_FFFF_FFFF {
        let [_, _, a1, a2, a3, a4, a5, a6] = v.to_be_bytes();
        out.write_all_bytes(&[253, a1, a2, a3, a4, a5, a6])?;
        7
    } else if v <= 0xFF_FFFF_FFFF_FFFF {
        let [_, a1, a2, a3, a4, a5, a6, a7] = v.to_be_bytes();
        out.write_all_bytes(&[254, a1, a2, a3, a4, a5, a6, a7])?;
        8
    } else {
        out.write_u8(255)?;
        out.write_be_u64(v)?;
        9
    })
}

impl<V> EncodeVarintTo for V
where
    V: Into<IntegerValue> + Copy,
{
    fn encode_varint_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        encode_varint_to(Into::<IntegerValue>::into(*self), out)
    }
}

///
/// Decodes up to 9 bytes in the sqlite4 varint format
pub fn decode_varint<T: Bits>(inp: &mut T) -> Result<u64, Error> {
    let mut out = 0;
    let a0 = inp.read_u8()? as u64;
    match a0 {
        241..=248 => {
            let a1 = inp.read_u8()? as u64;
            out = 240 + 258 * (a0 - 241) + a1;
        }
        249 => {
            let a1 = inp.read_u8()? as u64;
            let a2 = inp.read_u8()? as u64;
            out = 2288 + 258 * a1 + a2;
        }
        250 => {
            let a1 = inp.read_u8()?;
            let a2 = inp.read_u8()?;
            let a3 = inp.read_u8()?;
            out = u32::from_be_bytes([0, a1, a2, a3]) as u64;
        }
        251 => {
            out = inp.read_be_u32()? as u64;
        }
        252 => {
            let a1 = (inp.read_u8()? as u64) << 32;
            out = a1 | inp.read_be_u32()? as u64;
        }
        253 => {
            let a1 = (inp.read_be_u16()? as u64) << 32;
            out = a1 | inp.read_be_u32()? as u64;
        }
        254 => {
            let a1 = (inp.read_u8()? as u64) << 40;
            let a2 = (inp.read_be_u16()? as u64) << 32;
            out = a1 | a2 | inp.read_be_u32()? as u64;
        }
        255 => {
            out = inp.read_be_u64()?;
        }
        _ => {}
    }
    Ok(out)
}

///
/// Decodes up to 9 bytes in the sqlite4 varint format
pub trait DecodeVarint {
    fn decode_varint(&mut self) -> Result<u64, Error>;
}

impl<T: Bits> DecodeVarint for T {
    fn decode_varint(&mut self) -> Result<u64, Error> {
        decode_varint(self)
    }
}

///
/// The required length to encode in the group varint format.  Either 1, 2, 3, or 4 bytes.
pub const fn gvarint_length(value: IntegerValue) -> u8 {
    let value = value.to_be_u32();
    match value {
        0x0000_0000..=0x0000_00FF => 1,
        0x0000_0100..=0x0000_FFFF => 2,
        0x0001_0000..=0x00FF_FFFF => 3,
        _ => 4,
    }
}
///
/// The required length to encode in the group varint format.  Either 1, 2, 3, or 4 bytes.
pub trait GroupVarintRequiredLength {
    fn gvarint_length(&self) -> u8;
}
impl<T> GroupVarintRequiredLength for T
where
    T: Into<IntegerValue> + Copy,
{
    fn gvarint_length(&self) -> u8 {
        gvarint_length(Into::<IntegerValue>::into(*self))
    }
}

///
/// Writes only the used number of bytes in the integer to the output stream.
pub trait EncodeUsedBytesTo {
    fn encode_used_bytes_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError>;
}
impl EncodeUsedBytesTo for u32 {
    fn encode_used_bytes_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        let used = self.gvarint_length() as usize;
        if used == 1 {
            out.write_u8(*self as u8)?;
        } else if used == 2 {
            out.write_be_u16(*self as u16)?;
        } else if used == 3 {
            let [_, a, b, c] = self.to_be_bytes();
            out.write_all_bytes(&[a, b, c])?;
        } else {
            out.write_be_u32(*self)?;
        }
        Ok(used)
    }
}

pub trait DecodeUsedBytesFrom: Sized {
    fn decode_used_bytes<T: Bits + ?Sized>(inp: &mut T, len: u8) -> Result<Self, Error>;
}
impl DecodeUsedBytesFrom for u32 {
    fn decode_used_bytes<T: Bits + ?Sized>(inp: &mut T, len: u8) -> Result<Self, Error> {
        let mut out = 0u32;
        for _ in 0..len {
            out <<= 8;
            out |= inp.read_u8()? as u32;
        }
        Ok(out)
    }
}

///
/// The 'Group Varint' format, which moves all the control bits to header bytes
pub trait EncodeGroupVarintTo {
    fn encode_group_varint_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError>;
}
impl EncodeGroupVarintTo for [u32; 4] {
    fn encode_group_varint_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<usize, BitsError> {
        let mut buf = FixedU8Buf::<16>::new();
        let [a, b, c, d] = *self;
        let a = a.encode_used_bytes_to(&mut buf)? as u8;
        let b = b.encode_used_bytes_to(&mut buf)? as u8;
        let c = c.encode_used_bytes_to(&mut buf)? as u8;
        let d = d.encode_used_bytes_to(&mut buf)? as u8;
        let hdr = ((a - 1) & 0x03) << 6;
        let hdr = hdr | ((b - 1) & 0x03) << 4;
        let hdr = hdr | ((c - 1) & 0x03) << 2;
        let hdr = hdr | ((d - 1) & 0x03);
        out.write_u8(hdr)?;
        out.write_all_bytes(buf.as_ref_used())?;
        Ok(buf.len() + 1)
    }
}

pub trait DecodeGroupVarintFrom: Sized {
    fn decode_group_varint_from<T: Bits>(inp: &mut T) -> Result<Option<[Self; 4]>, Error>;
}
impl DecodeGroupVarintFrom for u32 {
    fn decode_group_varint_from<T: Bits>(inp: &mut T) -> Result<Option<[Self; 4]>, Error> {
        let Some(ctrl) = inp.next_u8()? else {
            return Ok(None);
        };
        let dl = (ctrl & 0x3) + 1;
        let cl = ((ctrl >> 2) & 0x3) + 1;
        let bl = ((ctrl >> 4) & 0x3) + 1;
        let al = ((ctrl >> 6) & 0x3) + 1;

        Ok(Some([
            u32::decode_used_bytes(inp, al)?,
            u32::decode_used_bytes(inp, bl)?,
            u32::decode_used_bytes(inp, cl)?,
            u32::decode_used_bytes(inp, dl)?,
        ]))
    }
}

#[cfg(test)]
mod test {
    use crate::buf::{Buffer, FixedU8Buf, RoundU8Buffer};
    use crate::codec::{DecodeGroupVarintFrom, EncodeGroupVarintTo};
    use irox_bits::Error;

    #[test]
    pub fn test_group_encoding() -> Result<(), Error> {
        let mut buf = FixedU8Buf::<16>::new();
        let used = [0xAAAAu32, 0xBBBBBB, 0xCC, 0xDDDDDDDD].encode_group_varint_to(&mut buf)?;
        assert_eq_hex_slice!(
            &[0x63, 0xAA, 0xAA, 0xBB, 0xBB, 0xBB, 0xCC, 0xDD, 0xDD, 0xDD, 0xDD],
            buf.as_ref()
        );
        assert_eq!(11, used);

        Ok(())
    }

    #[test]
    pub fn test_group_decoding() -> Result<(), Error> {
        let mut buf = RoundU8Buffer::from([
            0x63, 0xAA, 0xAA, 0xBB, 0xBB, 0xBB, 0xCC, 0xDD, 0xDD, 0xDD, 0xDD,
        ]);

        let res = u32::decode_group_varint_from(&mut buf)?;
        assert!(res.is_some());
        if let Some(res) = res {
            assert_eq_hex_slice!(&[0xAAAA, 0xBBBBBB, 0xCC, 0xDDDDDDDD], res.as_ref());
        }

        assert_eq!(0, buf.len());

        Ok(())
    }
}
