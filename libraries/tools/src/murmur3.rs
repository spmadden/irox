// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::ops::BitXorAssign;

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
        let (a, b) = chunk_16.split_at(8);
        let Ok(k1) = crate::bits::read_be_u64(a) else {
            return 0;
        };
        let Ok(k2) = crate::bits::read_be_u64(b) else {
            return 0;
        };
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
    let mut k1: u64 = 0;
    let mut k2: u64 = 0;
    let rem = chunks.remainder();
    let len = rem.len();
    let mut iter = rem.iter();

    let mut shift = 0;
    for i in 0..len {
        let Some(val) = iter.next() else {
            break;
        };
        let val: u64 = *val as u64;
        if i > 8 {
            k2.bitxor_assign(val.wrapping_shl(shift as u32));
        } else {
            k1.bitxor_assign(val.wrapping_shl(shift as u32));
        }
        shift += 8;
    }
    h2.bitxor_assign(k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1));
    h1.bitxor_assign(k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2));

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
    use crate::murmur3::murmur3_128;

    #[test]
    pub fn tests() {
        let tests: Vec<(&'static str, u128)> = vec![
            ("", 0x00000000000000000000000000000000_u128),
            ("1", 0x71FBBBFE8A7B7C71942AEB9BF9F0F637_u128),
            ("12", 0x4A533C6209E3FD9588C72C695E0B311D_u128),
            ("123", 0x985B2D1B0D667F6A427EA1E3CE0ECF69_u128),
            ("1234", 0x0897364D218FE7B4341E8BD92437FDA5_u128),
            ("12345", 0x20F83A176B21DFCBF13C5C41325CA9F4_u128),
            ("123456", 0xE417CF050BBBD0D651A48091002531FE_u128),
            // ("Lorem ipsum dolor sit amet, consectetur adipisicing elit", 0x0),
        ];
        for (data, exp) in tests {
            let hash = murmur3_128(data.as_bytes());
            println!("{hash:X} {data}");
            assert_eq!(exp, hash);
        }
    }
}
