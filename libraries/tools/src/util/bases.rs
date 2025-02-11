// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::buf::{Buffer, StrBuf};

/// Base32 Alphabet chosen to maximize letter disparity
pub static ZBASE32: &[u8; 32] = &[
    b'Y', b'B', b'N', b'D', b'R', b'F', b'G', b'8', b'E', b'J', b'K', b'M', b'C', b'P', b'Q', b'X',
    b'O', b'T', b'1', b'U', b'W', b'I', b'S', b'Z', b'A', b'3', b'4', b'5', b'H', b'7', b'6', b'9',
];
/// Base32 standard alphabet
pub static BASE32_1: &[u8; 32] = &[
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'2', b'3', b'4', b'5', b'6', b'7',
];
/// Base32 Extended Hex Alphabet
pub static BASE32_2: &[u8; 32] = &[
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
];
/// Base32 Alphabet chosen to ensure that words can't be randomly made
pub static BASE32_WORDSAFE: &[u8; 32] = &[
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'C', b'F', b'G', b'H', b'J', b'M', b'P', b'Q',
    b'R', b'V', b'W', b'X', b'c', b'f', b'g', b'h', b'j', b'm', b'p', b'q', b'r', b'v', b'w', b'x',
];
pub static BASE31_VG: &[u8; 31] = &[
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'B', b'C', b'D', b'F', b'G', b'H',
    b'J', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'V', b'W', b'X', b'Y', b'Z',
];
pub static BASE20_WORDSAFE: &[u8; 20] = &[
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'C', b'F', b'G', b'H', b'J', b'M', b'P', b'Q',
    b'R', b'V', b'W', b'X',
];
pub static BASE12: &[u8; 12] = &[
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B',
];
pub fn convert_base10_to_base_n<const N: usize>(base_10: u64, alphabet: &[u8; N]) -> StrBuf<64> {
    let mut out = StrBuf::new();

    if N == 0 {
        return out;
    }

    let mut iter = base_10;
    let n = N as u64;
    while iter > 0 {
        let i = iter % n;
        let _ = out.push(alphabet.get(i as usize).copied().unwrap_or_default());
        iter -= i;
        iter /= n;
    }

    if out.is_empty() {
        let _ = out.push(alphabet.get(0).copied().unwrap_or_default());
    }
    out.reverse();
    out
}

#[cfg(test)]
mod tests {
    use crate::bases::{convert_base10_to_base_n, ZBASE32};

    #[test]
    pub fn testb321() {
        let res = convert_base10_to_base_n(2048, ZBASE32);
        assert_eq!("NYY", res.as_str().unwrap_or_default());
        let res = convert_base10_to_base_n(2047, ZBASE32);
        assert_eq!("B99", res.as_str().unwrap_or_default());

        let res = convert_base10_to_base_n(u64::MAX, ZBASE32);
        assert_eq!("X999999999999", res.as_str().unwrap_or_default());
    }
}
