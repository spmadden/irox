// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Hexdump & Hex manipulation

use std::io::{Error, Write};

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
    fn hexdump(&self);

    /// Hexdump to the specified writer.
    fn hexdump_to<T: Write>(&self, out: &mut T) -> Result<(), std::io::Error>;
}

impl<S: AsRef<[u8]>> HexDump for S {
    fn hexdump(&self) {
        let _ = self.hexdump_to(&mut std::io::stdout());
    }

    fn hexdump_to<T: Write>(&self, out: &mut T) -> Result<(), Error> {
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

#[cfg(test)]
mod tests {
    use crate::hex::HexDump;

    #[test]
    pub fn test() -> Result<(), std::io::Error> {
        let mut buf: Vec<u8> = Vec::new();
        for v in u8::MIN..=u8::MAX {
            buf.push(v);
        }

        buf.hexdump();

        Ok(())
    }
}
