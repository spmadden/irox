// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::buf::StrBuf;
use crate::hash::SHA1;
use crate::hex;
use core::fmt::Write;
use irox_bits::FromBEBytes;

///
/// Takes the OTP value of the hash and returns 6 hex [0-F] characters for a security
/// strength of 24 bits.
pub fn otp_hex_6<const N: usize>(hash: &[u8; N]) -> StrBuf<6> {
    if N < 16 {
        return otp_hex_6(&SHA1::new().hash(hash));
    }
    let offset = (hash.last().copied().unwrap_or_default() & 0xF) as usize;

    let binary = [hash[offset + 1], hash[offset + 2], hash[offset + 3]];
    let mut out = StrBuf::new();
    let _ = hex::to_hex_strbuf_upper(&binary, &mut out);

    out
}

///
/// Takes the OTP value of the hash and returns 6 decimal characters [0-9] for a security
/// strength of almost 20 bits.
#[allow(clippy::integer_division_remainder_used)]
pub fn otp_6<const N: usize>(hash: &[u8; N]) -> StrBuf<6> {
    if N < 16 {
        return otp_6(&SHA1::new().hash(hash));
    }
    let offset = (hash.last().copied().unwrap_or_default() & 0xF) as usize;

    let binary = [
        hash[offset] & 0x7F,
        hash[offset + 1],
        hash[offset + 2],
        hash[offset + 3],
    ];
    let binary: u32 = FromBEBytes::<4>::from_be_bytes(binary);
    let binary = binary % 1_000_000;
    let mut out = StrBuf::new();
    let _ = write!(out, "{binary}");
    out
}

#[cfg(test)]
mod test {
    use crate::hash::viz::{otp_6, otp_hex_6};

    #[test]
    pub fn test1() {
        let hash = [
            0xcc, 0x93, 0xcf, 0x18, 0x50, 0x8d, 0x94, 0x93, 0x4c, 0x64, 0xb6, 0x5d, 0x8b, 0xa7,
            0x66, 0x7f, 0xb7, 0xcd, 0xe4, 0xb0,
        ];
        let res = otp_6(&hash);
        assert_eq!("755224", res.as_str().unwrap());
        let res = otp_hex_6(&hash);
        assert_eq!("93CF18", res.as_str().unwrap());
    }

    #[test]
    pub fn test2() {
        let hash = [
            0xad, 0xc8, 0x3b, 0x19, 0xe7, 0x93, 0x49, 0x1b, 0x1c, 0x6e, 0xa0, 0xfd, 0x8b, 0x46,
            0xcd, 0x9f, 0x32, 0xe5, 0x92, 0xfc,
        ];
        let res = otp_6(&hash);
        assert_eq!("189535", res.as_str().unwrap());
        let res = otp_hex_6(&hash);
        assert_eq!("46CD9F", res.as_str().unwrap());
    }
}
