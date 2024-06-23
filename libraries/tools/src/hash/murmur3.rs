// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Implementation of Murmurhash3.  Currently only x128 implemented.
//!

use core::ops::BitXorAssign;
use irox_bits::Bits;

const C1: u64 = 0x87c3_7b91_1142_53d5;
const C2: u64 = 0x4cf5_ad43_2745_937f;
const C3: u64 = 0xff51_afd7_ed55_8ccd;
const C4: u64 = 0xc4ce_b9fe_1a85_ec53;

///
/// Runs the murmur3_128 hash function with no seed
pub fn murmur3_128<T: AsRef<[u8]>>(key: T) -> u128 {
    murmur3_128_seed(key, 0)
}

macro_rules! fmix64 {
    ($k:ident) => {
        $k.bitxor_assign($k >> 33);
        $k = $k.wrapping_mul(C3);
        $k.bitxor_assign($k >> 33);
        $k = $k.wrapping_mul(C4);
        $k.bitxor_assign($k >> 33);
    };
}

///
/// Runs the murmur3_128 hash function with the specified seed.
pub fn murmur3_128_seed<T: AsRef<[u8]>>(key: T, seed: u32) -> u128 {
    let data = key.as_ref();
    let orig_len = data.len() as u64;
    let mut h1: u64 = seed as u64;
    let mut h2: u64 = seed as u64;

    let mut chunks = data.chunks_exact(16);
    for chunk_16 in chunks.by_ref() {
        let (mut a, mut b) = chunk_16.split_at(8);
        let Ok(k1) = a.read_be_u64() else {
            return 0;
        };
        let Ok(k2) = b.read_be_u64() else {
            return 0;
        };
        let k1 = k1.swap_bytes();
        let k2 = k2.swap_bytes();
        h1.bitxor_assign(k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2));
        h1 = h1
            .rotate_left(27)
            .wrapping_add(h2)
            .wrapping_mul(5)
            .wrapping_add(0x52dce729);
        h2.bitxor_assign(k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1));
        h2 = h2
            .rotate_left(31)
            .wrapping_add(h1)
            .wrapping_mul(5)
            .wrapping_add(0x38495ab5);
    }
    let rem = chunks.remainder();
    let len = rem.len();
    if len > 0 {
        let mut k1: u64 = 0;
        let mut k2: u64 = 0;
        let mut iter = rem.iter();

        let mut shift: u32 = 0;
        for i in 0..len {
            let Some(val) = iter.next() else {
                break;
            };
            let val: u64 = *val as u64;
            if i == 8 {
                shift = 0;
            }
            if i >= 8 {
                k2.bitxor_assign(val.wrapping_shl(shift));
            } else {
                k1.bitxor_assign(val.wrapping_shl(shift));
            }
            shift += 8;
        }
        if len > 8 {
            h2.bitxor_assign(k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1));
        }
        h1.bitxor_assign(k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2));
    }

    h1.bitxor_assign(orig_len);
    h2.bitxor_assign(orig_len);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    fmix64!(h1);
    fmix64!(h2);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    (h1 as u128) << 64 | h2 as u128
}

#[cfg(test)]
mod test {
    use crate::hash::murmur3_128;
    use alloc::vec;
    extern crate alloc;
    use alloc::vec::Vec;

    #[test]
    pub fn tests() {
        let tests: Vec<(&'static str, u128)> = vec![
            ("", 0x0000000000000000_0000000000000000_u128),
            ("1", 0x71FBBBFE8A7B7C71_942AEB9BF9F0F637_u128),
            ("12", 0x4A533C6209E3FD95_88C72C695E0B311D_u128),
            ("123", 0x985B2D1B0D667F6A_427EA1E3CE0ECF69_u128),
            ("1234", 0x0897364D218FE7B4_341E8BD92437FDA5_u128),
            ("12345", 0x20F83A176B21DFCB_F13C5C41325CA9F4_u128),
            ("123456", 0xE417CF050BBBD0D6_51A48091002531FE_u128),
            ("1234567", 0x2CDAC5F7F2C623A2_37DC518BCAE1D955_u128),
            ("12345678", 0x3B4A640638B1419C_913B0E676BD42557_u128),
            ("123456789", 0x3C84645EDB66CCA4_99f8FAC73A1EA105_u128),
            ("1234567890", 0xECFA4AE68079870A_C1D017C820EBD22B_u128),
            ("12345678901", 0x2A84FB1385B327D3_DAEB95857DE0DFC1_u128),
            ("123456789012", 0xDDA6E38B7C022914_75A23983FD719D1E_u128),
            ("1234567890123", 0xE3DDF2853772DF49_1BC521F05EEF2497_u128),
            ("12345678901234", 0x7D51E170E83CCC91_C63D6CBEFAF85AD0_u128),
            ("123456789012345", 0x887001AEA2AFCFD6_1EC326364F0801B3_u128),
            ("1234567890123456", 0x4FBE5DC5C0E32CF8_C0C8E96B60C322C1_u128),
            (
                "12345678901234567",
                0x748617968026B77E_291E6386473F7103_u128,
            ),
            (
                "123456789012345678",
                0xEAEAE51CCFA961AF_754C657D52CC0469_u128,
            ),
            (
                "1234567890123456789",
                0x0C722FBA0A479959_4EBBCD6912218A2A_u128,
            ),
            (
                "12345678901234567890",
                0xB11CD81925DC8C3A_719F603CE8F1367D_u128,
            ),
            (
                "123456789012345678901",
                0xA2D7F23C16EE6855_FEE63702A5F53DD3_u128,
            ),
            (
                "1234567890123456789010",
                0x37208BC7AE7E7076_EFA979587AABB8AF_u128,
            ),
            ("Hello, world!", 0xF1512DD1D2D665DF_2C326650A8F3C564_u128),
        ];
        for (data, exp) in tests {
            let hash = murmur3_128(data.as_bytes());
            assert_eq!(exp, hash);
        }
    }
}
