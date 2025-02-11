// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::buf::{Buffer, StrBuf};

/// Base32 Alphabet chosen to maximize letter disparity
pub static ZBASE32: &[u8; 32] = b"YBNDRFG8EJKMCPQXOT1UWISZA345H769";
/// Base32 standard alphabet
pub static BASE32_1: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
/// Base32 Extended Hex Alphabet
pub static BASE32_2: &[u8; 32] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
/// Base32 Alphabet chosen to ensure that words can't be randomly made
pub static BASE32_WORDSAFE: &[u8; 32] = b"23456789CFGHJMPQRVWXcfghjmpqrvwx";
pub static BASE31_VG: &[u8; 31] = b"0123456789BCDFGHJKLMNPQRSTVWXYZ";
pub static BASE20_WORDSAFE: &[u8; 20] = b"23456789CFGHJMPQRVWX";
pub static BASE12: &[u8; 12] = b"0123456789AB";
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
        let _ = out.push(alphabet.first().copied().unwrap_or_default());
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
