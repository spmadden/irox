// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Hexdump & Hex manipulation

extern crate alloc;
use crate::bits::{Error, ErrorKind, MutBits};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

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
    /// Hexdump this data structure to stdout
    #[cfg(feature = "std")]
    fn hexdump(&self);

    /// Hexdump to the specified writer.
    fn hexdump_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<(), Error>;
}

impl<S: AsRef<[u8]>> HexDump for S {
    #[cfg(feature = "std")]
    fn hexdump(&self) {
        let _ = self.hexdump_to(&mut crate::bits::BitsWrapper(&mut std::io::stdout().lock()));
    }

    fn hexdump_to<T: MutBits + ?Sized>(&self, out: &mut T) -> Result<(), Error> {
        let mut idx = 0;
        let val = self.as_ref();
        loop {
            write!(out, "{idx:08X}  ")?;
            let mut buf = Vec::new();
            for sidx in 0..16 {
                let Some(v) = val.get(idx + sidx) else {
                    break;
                };
                buf.push(*v);
            }
            for v in &buf {
                write!(out, "{v:02X} ")?;
            }
            for _i in 0..(16 - buf.len()) {
                write!(out, "   ")?;
            }
            write!(out, " |")?;
            for v in &buf {
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
            for _i in 0..(16 - buf.len()) {
                write!(out, " ")?;
            }
            writeln!(out, "|")?;
            idx += 16;
            if buf.len() != 16 {
                break;
            }
        }
        Ok(())
    }
}

pub fn hex_char_to_nibble(ch: char) -> Result<u8, Error> {
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
        _ => return Err(ErrorKind::InvalidData.into()),
    })
}

///
/// Parses the provided string, a series of hex characters [a-fA-F0-9] and converts them to the
/// associated byte format.
pub fn from_hex_str(hex: &str) -> Result<Box<[u8]>, Error> {
    let len = hex.len();
    let mut out: Vec<u8> = Vec::with_capacity(len * 2);

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

///
/// Prints the value to a uppercase hex string
pub fn to_hex_str_upper(val: &[u8]) -> String {
    let len = val.len() * 2;
    let mut out = String::with_capacity(len);

    for v in val {
        let _ = write!(&mut out, "{v:02X}");
    }

    out
}

///
/// Prints the value to a uppercase hex string
pub fn to_hex_str_lower(val: &[u8]) -> String {
    let len = val.len() * 2;
    let mut out = String::with_capacity(len);

    for v in val {
        let _ = write!(&mut out, "{v:02x}");
    }

    out
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use crate::hex::HexDump;
    use alloc::vec::Vec;

    #[test]
    pub fn test() -> Result<(), crate::bits::Error> {
        let mut buf: Vec<u8> = Vec::new();
        for v in u8::MIN..=u8::MAX {
            buf.push(v);
        }

        buf.hexdump();

        Ok(())
    }
}
