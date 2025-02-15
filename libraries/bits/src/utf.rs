// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! UTF-8 Encoding & Decoding
//!
//! Using info from: [here](https://simonsapin.github.io/wtf-8/#generalized-utf8)
//!

#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::indexing_slicing)]

use crate::{Bits, BitsErrorKind, Error, MutBits};

const MAX_ONE_BYTE: u8 = 0x80;

const TAG_TWO_BYTE: u8 = 0b110_00000;
const TAG_TWO_BYTE_MASK: u8 = 0b111_00000;
const MAX_TWO_BYTE: u32 = 0x800;

const TAG_THREE_BYTE: u8 = 0b1110_0000;
const TAG_THREE_BYTE_MASK: u8 = 0b1111_0000;
const MAX_THREE_BYTE: u32 = 0x10000;

const TAG_FOUR_BYTE: u8 = 0b11110_000;
const TAG_FOUR_BYTE_MASK: u8 = 0b11111_000;

const TAG_CONTINUE: u8 = 0b1000_0000;

/// Returns the number of required bytes to store the specified character in UTF-8.
pub fn required_utf8_bytes(val: char) -> usize {
    let val = val as u32;
    if val < MAX_ONE_BYTE as u32 {
        1
    } else if val < MAX_TWO_BYTE {
        2
    } else if val < MAX_THREE_BYTE {
        3
    } else {
        4
    }
}
/// Encodes the character into the provided buffer using UTF-8.  A subslice
/// of the provided buffer is returned providing the amount of buffer actually
/// used.
pub fn encode_be_utf8_char(val: char, buf: &mut [u8; 4]) -> Result<&[u8], Error> {
    let len = required_utf8_bytes(val);
    let val = val as u32;
    match (len, &mut buf[..]) {
        (1, [a, ..]) => {
            *a = val as u8;
        }
        (2, [a, b, ..]) => {
            *a = ((val >> 6) & 0x1F) as u8 | TAG_TWO_BYTE;
            *b = (val & 0x3F) as u8 | TAG_CONTINUE;
        }
        (3, [a, b, c, ..]) => {
            *a = ((val >> 12) & 0x0F) as u8 | TAG_THREE_BYTE;
            *b = ((val >> 6) & 0x3F) as u8 | TAG_CONTINUE;
            *c = (val & 0x3F) as u8 | TAG_CONTINUE;
        }
        (4, [a, b, c, d]) => {
            *a = ((val >> 18) & 0x07) as u8 | TAG_FOUR_BYTE;
            *b = ((val >> 12) & 0x3F) as u8 | TAG_CONTINUE;
            *c = ((val >> 6) & 0x3F) as u8 | TAG_CONTINUE;
            *d = (val & 0x3F) as u8 | TAG_CONTINUE;
        }
        _ => return Err(BitsErrorKind::FormatError.into()),
    }
    Ok(&buf[..len])
}

/// Writes 1, 2, 3, or 4 bytes representing the unicode UTF-8 format character to the
/// specified output.  Upon success, returns the number of bytes written.
pub fn write_be_utf8_char<T: MutBits + ?Sized>(val: char, out: &mut T) -> Result<usize, Error> {
    let mut buf = [0u8; 4];
    let val = encode_be_utf8_char(val, &mut buf)?;
    out.write_all_bytes(val)?;
    Ok(val.len())
}

/// Reads 1, 2, 3, or 4 bytes representing the unicode UTF-8 formatted character from the
/// specified input.  Returns the character read and the number of bytes consumed.
pub fn read_be_utf8_char<T: Bits + ?Sized>(src: &mut T) -> Result<(char, usize), Error> {
    let a = src.read_u8()?;
    if a < MAX_ONE_BYTE {
        return Ok((a as char, 1));
    }
    let (val, len) = if (a & TAG_TWO_BYTE_MASK) == TAG_TWO_BYTE {
        let b = (src.read_u8()? & 0x3F) as u32;
        let a = ((a & 0x1F) as u32) << 6;
        (a | b, 2)
    } else if (a & TAG_THREE_BYTE_MASK) == TAG_THREE_BYTE {
        let b = ((src.read_u8()? & 0x3F) as u32) << 6;
        let c = (src.read_u8()? & 0x3F) as u32;
        let a = ((a & 0xF) as u32) << 12;
        (a | b | c, 3)
    } else if (a & TAG_FOUR_BYTE_MASK) == TAG_FOUR_BYTE {
        let b = ((src.read_u8()? & 0x3F) as u32) << 12;
        let c = ((src.read_u8()? & 0x3F) as u32) << 6;
        let d = (src.read_u8()? & 0x3F) as u32;
        let a = ((a & 0x7) as u32) << 24;
        (a | b | c | d, 4)
    } else {
        return Err(BitsErrorKind::FormatError.into());
    };
    let Some(val) = char::from_u32(val) else {
        return Err(BitsErrorKind::InvalidInput.into());
    };
    Ok((val, len))
}
