// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Hexdump & Hex manipulation

crate::cfg_feature_alloc! {
    extern crate alloc;
}
use crate::buf::{FixedU8Buf, StrBuf};
use crate::cfg_feature_alloc;
use core::fmt::Write;
use irox_bits::{BitsError, BitsErrorKind, Error, ErrorKind, FormatBits, MutBits};

/// 0-9, A-F
pub static HEX_UPPER_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];
/// 0-9, a-f
pub static HEX_LOWER_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

///
/// Dumps the contents of this data structure in a pretty 16 slot wide format, like the output of
/// `hexdump -C`
pub trait HexDump {
    crate::cfg_feature_std! {
        /// Hexdump this data structure to stdout
        fn hexdump(&self);
    }

    /// Hexdump to the specified writer.
    fn hexdump_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<(), Error>;
}

impl<S: AsRef<[u8]>> HexDump for S {
    crate::cfg_feature_std! {
        fn hexdump(&self) {
            let _ = self.hexdump_to(&mut irox_bits::BitsWrapper::Borrowed(&mut std::io::stdout().lock()));
        }
    }

    fn hexdump_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<(), Error> {
        let mut idx = 0;
        let chunks = self.as_ref().chunks(16);
        let mut out: FormatBits<T> = out.into();
        for chunk in chunks {
            write!(out, "{idx:08X}  ")?;
            for v in chunk {
                write!(out, "{v:02X} ")?;
            }
            for _i in 0..(16 - chunk.len()) {
                write!(out, "   ")?;
            }
            write!(out, " |")?;
            for v in chunk {
                match *v {
                    0..=0x1F | 0x7F..=0xA0 | 0xFF => {
                        // nonprintables
                        write!(out, ".")?;
                    }
                    p => {
                        // printables
                        write!(out, "{}", p as char)?;
                    }
                }
            }
            for _i in 0..(16 - chunk.len()) {
                write!(out, " ")?;
            }
            writeln!(out, "|")?;
            idx += 16;
        }
        Ok(())
    }
}
cfg_feature_alloc! {
    /// Prints the values in the slice as a static rust-type array
    pub fn to_hex_array(value: &[u8]) -> alloc::string::String {
        let mut out = alloc::vec::Vec::new();
        for v in value {
            out.push(format!("0x{:02X}", v));
        }
        let joined = out.join(",");

        format!("[{joined}]")
    }
}

pub const fn hex_char_to_nibble(ch: char) -> Result<u8, Error> {
    Ok(match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 0xA,
        'b' | 'B' => 0xB,
        'c' | 'C' => 0xC,
        'd' | 'D' => 0xD,
        'e' | 'E' => 0xE,
        'f' | 'F' => 0xF,
        _ => return ErrorKind::InvalidData.err("Invalid hex character"),
    })
}
/// Static equivalent of `format!("{:X}", val);`
pub const fn nibble_to_hex_char(val: u8) -> Result<char, Error> {
    Ok(match val {
        0x0 => '0',
        0x1 => '1',
        0x2 => '2',
        0x3 => '3',
        0x4 => '4',
        0x5 => '5',
        0x6 => '6',
        0x7 => '7',
        0x8 => '8',
        0x9 => '9',
        0xA => 'A',
        0xB => 'B',
        0xC => 'C',
        0xD => 'D',
        0xE => 'E',
        0xF => 'F',
        _ => return ErrorKind::InvalidData.err("Invalid hex character"),
    })
}

crate::cfg_feature_alloc! {
    ///
    /// Parses the provided string, a series of hex characters [a-fA-F0-9] and converts them to the
    /// associated byte format.
    pub fn from_hex_str(hex: &str) -> Result<alloc::boxed::Box<[u8]>, Error> {
        let len = hex.len();
        let mut out = alloc::vec::Vec::with_capacity(len * 2);

        let mut val = 0u8;
        let mut idx = 0;
        for ch in hex.chars() {
            if ch == ' ' {
                continue;
            }
            let ch = hex_char_to_nibble(ch)?;
            if idx & 0x1 == 0 {
                val |= (ch << 4) & 0xF0;
            } else {
                val |= ch & 0xF;
                out.push(val);
                val = 0;
            }
            idx += 1;
        }

        Ok(out.into_boxed_slice())
    }
}

///
/// Parses the provided string, a series of hex characters [a-fA-F0-9] and converts them to the
/// associated byte format.  Returns the number of bytes written.
pub fn from_hex_into<T: MutBits>(hex: &str, out: &mut T) -> Result<usize, Error> {
    let mut val = 0u8;
    let mut idx = 0;
    let mut wrote = 0;
    for ch in hex.chars() {
        if ch == ' ' {
            continue;
        }
        let ch = hex_char_to_nibble(ch)?;
        if idx & 0x1 == 0 {
            val |= (ch << 4) & 0xF0;
        } else {
            val |= ch & 0xF;
            out.write_u8(val)?;
            wrote += 1;
            val = 0;
        }
        idx += 1;
    }

    Ok(wrote)
}

///
/// Attempts to fill a static array buffer with the hex data within the provided string,
/// returns the buffer if the string and buffer are perfectly matched.
pub fn try_from_hex_str<const N: usize>(str: &str) -> Result<[u8; N], BitsError> {
    let mut buf = FixedU8Buf::<N>::new();
    if from_hex_into(str, &mut buf)? != N {
        return Err(BitsErrorKind::UnexpectedEof.into());
    }
    Ok(buf.take())
}

crate::cfg_feature_alloc! {
    ///
    /// Prints the value to a uppercase hex string
    pub fn to_hex_str_upper(val: &[u8]) -> alloc::string::String {
        let len = val.len() * 2;
        let mut out = alloc::string::String::with_capacity(len);

        for v in val {
            let _ = write!(&mut out, "{v:02X}");
        }

        out
    }
}

crate::cfg_feature_alloc! {
    ///
    /// Prints the value to a lowercase hex string
    pub fn to_hex_str_lower(val: &[u8]) -> alloc::string::String {
        let len = val.len() * 2;
        let mut out = alloc::string::String::with_capacity(len);

        for v in val {
            let _ = write!(&mut out, "{v:02x}");
        }

        out
    }
}

///
/// Prints the value to a lowercase hex string and stores it in the provided
/// [`StrBuf`].  The size of the StrBuf must be `>= 2x val.len()`
pub fn to_hex_strbuf_lower<const N: usize>(val: &[u8], buf: &mut StrBuf<N>) -> Result<(), Error> {
    let len = val.len() * 2;
    if N < len {
        return Err(ErrorKind::UnexpectedEof.into());
    }
    for v in val {
        write!(buf, "{v:02x}")?;
    }

    Ok(())
}

///
/// Prints the value to a uppercase hex string and stores it in the provided
/// [`StrBuf`].  The size of the StrBuf must be `>= 2x val.len()`
pub fn to_hex_strbuf_upper<const N: usize>(val: &[u8], buf: &mut StrBuf<N>) -> Result<(), Error> {
    let len = val.len() * 2;
    if N < len {
        return Err(ErrorKind::UnexpectedEof.into());
    }
    for v in val {
        write!(buf, "{v:02X}")?;
    }

    Ok(())
}

#[doc(hidden)]
#[allow(clippy::indexing_slicing)]
pub const fn hex_len(vals: &[&[u8]]) -> Option<usize> {
    let mut out = 0;
    let mut idx = 0;
    while idx < vals.len() {
        let val = vals[idx];
        let len = val.len();

        out += len;
        idx += 1;
    }
    if out & 0x01 == 0x01 {
        None
    } else {
        Some(out / 2)
    }
}
#[doc(hidden)]
#[allow(clippy::indexing_slicing)]
pub const fn raw_hex<const L: usize>(vals: &[&[u8]]) -> Result<[u8; L], char> {
    let mut out = [0u8; L];
    let mut outidx = 0;
    let mut idx = 0;
    while idx < vals.len() {
        let val = vals[idx];
        let mut inneridx = 0;
        while inneridx < val.len() {
            let a = val[inneridx] as char;
            let Ok(a) = hex_char_to_nibble(a) else {
                return Err(a);
            };
            inneridx += 1;
            let b = val[inneridx] as char;
            let Ok(b) = hex_char_to_nibble(b) else {
                return Err(b);
            };
            inneridx += 1;
            out[outidx] = (a << 4) | b;
            outidx += 1;
        }
        idx += 1;
    }
    Ok(out)
}

#[allow(unused_macros)]
#[macro_export]
///
/// Const compile-time evaluation of the provided string literals
/// ```
/// let raw_hex = irox_tools::hex!("C0ffee" "BeEf");
//  assert_eq_hex_slice!(&[0xc0, 0xff, 0xee, 0xbe, 0xef] as &[u8], &raw_hex);
/// ```
macro_rules! hex {
    ($($input:literal)+) => {{
        const VALS: &[& 'static [u8]] = &[$($input.as_bytes(),)*];
        const LEN: usize = match $crate::hex::hex_len(VALS) {
            Some(v) => v,
            None => panic!("Hex string is an odd length")
        };
        const RTN: [u8;LEN] = match $crate::hex::raw_hex::<LEN>(VALS) {
            Ok(v) => v,
            Err(_) => panic!("Hex string contains invalid character")
        };
        RTN
    }};
}

cfg_feature_alloc! {
    use alloc::string::String;
    #[derive(Debug)]
    pub enum HexStr<const N: usize> {
        Str(String),
        Hex([u8; N]),
    }
    impl<const N: usize> Default for HexStr<N> {
        fn default() -> Self {
            HexStr::Hex([0; N])
        }
    }
    impl<const N: usize> HexStr<N> {
        pub fn as_u8(&mut self) -> Result<&[u8; N], BitsError> {
            Ok(match self {
                HexStr::Str(s) => {
                    let v = try_from_hex_str(s)?;
                    *self = HexStr::Hex(v);
                    self.as_u8()?
                }
                HexStr::Hex(a) => a,
            })
        }
        pub fn as_str(&mut self) -> Result<&str, BitsError> {
            Ok(match self {
                HexStr::Str(s) => {
                    s.as_str()
                }
                HexStr::Hex(a) => {
                    let s= to_hex_str_upper(a);
                    *self = HexStr::Str(s);
                    self.as_str()?
                }
            })
        }
    }

}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    extern crate alloc;
    use crate::hex::HexDump;
    use alloc::vec::Vec;

    #[test]
    pub fn test() -> Result<(), irox_bits::Error> {
        let mut buf: Vec<u8> = Vec::new();
        for v in u8::MIN..=u8::MAX {
            buf.push(v);
        }

        buf.hexdump();

        Ok(())
    }

    #[test]
    pub fn const_hex_test() -> Result<(), irox_bits::Error> {
        let raw_hex = hex!("");
        assert_eq_hex_slice!(&[] as &[u8], &raw_hex);
        let raw_hex = hex!("00");
        assert_eq_hex_slice!(&[0x0u8], &raw_hex);
        raw_hex.hexdump();
        Ok(())
    }
}
