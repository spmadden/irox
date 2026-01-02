// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{FromLEBytes, MutBits, ToLEBytes};
use irox_tools::buf::RoundU8Buffer;
const E1: u32 = 0x61707865;
const E2: u32 = 0x3320646e;
const E3: u32 = 0x79622d32;
const E4: u32 = 0x6b206574;

macro_rules! qr {
    (
        $a:expr,
        $b:expr,
        $c:expr,
        $d:expr
    ) => {
        *$b ^= ($a.wrapping_add(*$d)).rotate_left(7);
        *$c ^= ($b.wrapping_add(*$a)).rotate_left(9);
        *$d ^= ($c.wrapping_add(*$b)).rotate_left(13);
        *$a ^= ($d.wrapping_add(*$c)).rotate_left(18);
    };
}

macro_rules! rounds {
    ($rounds:expr, $state:ident) => {
        let mut i = 0;
        while (i < $rounds / 2) {
            qr!(
                &mut $state[0],
                &mut $state[4],
                &mut $state[8],
                &mut $state[12]
            );
            qr!(
                &mut $state[5],
                &mut $state[9],
                &mut $state[13],
                &mut $state[1]
            );
            qr!(
                &mut $state[10],
                &mut $state[14],
                &mut $state[2],
                &mut $state[6]
            );
            qr!(
                &mut $state[15],
                &mut $state[3],
                &mut $state[7],
                &mut $state[11]
            );

            qr!(
                &mut $state[0],
                &mut $state[1],
                &mut $state[2],
                &mut $state[3]
            );
            qr!(
                &mut $state[5],
                &mut $state[6],
                &mut $state[7],
                &mut $state[4]
            );
            qr!(
                &mut $state[10],
                &mut $state[11],
                &mut $state[8],
                &mut $state[9]
            );
            qr!(
                &mut $state[15],
                &mut $state[12],
                &mut $state[13],
                &mut $state[14]
            );
            i += 1;
        }
    };
}

///
/// Expands the provided key, counter, and nonce into a single 64-byte/512-bit Salsa block.
/// Xor this block byte-for-byte with the ciphertext/plaintext to perform the decrypt/encrypt
/// operation.
///
pub struct SalsaKeyGenerator<const ROUNDS: usize>;
impl<const ROUNDS: usize> SalsaKeyGenerator<ROUNDS> {
    pub fn generate(key: [u8; 32], counter: u32, nonce: [u8; 12]) -> RoundU8Buffer<64> {
        let [k1, k2, k3, k4, k5, k6, k7, k8] = <[u32; 8]>::from_le_bytes(key);
        let [n1, n2, n3] = <[u32; 3]>::from_le_bytes(nonce);
        let mut state = [
            E1, E2, E3, E4, k1, k2, k3, k4, k5, k6, k7, k8, counter, n1, n2, n3,
        ];
        rounds!(ROUNDS, state);
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = state;
        let mut out = RoundU8Buffer::<64>::default();
        let _ = out.write_le_u32(E1.wrapping_add(a));
        let _ = out.write_le_u32(E2.wrapping_add(b));
        let _ = out.write_le_u32(E3.wrapping_add(c));
        let _ = out.write_le_u32(E4.wrapping_add(d));
        let _ = out.write_le_u32(k1.wrapping_add(e));
        let _ = out.write_le_u32(k2.wrapping_add(f));
        let _ = out.write_le_u32(k3.wrapping_add(g));
        let _ = out.write_le_u32(k4.wrapping_add(h));
        let _ = out.write_le_u32(k5.wrapping_add(i));
        let _ = out.write_le_u32(k6.wrapping_add(j));
        let _ = out.write_le_u32(k7.wrapping_add(k));
        let _ = out.write_le_u32(k8.wrapping_add(l));
        let _ = out.write_le_u32(counter.wrapping_add(m));
        let _ = out.write_le_u32(n1.wrapping_add(n));
        let _ = out.write_le_u32(n2.wrapping_add(o));
        let _ = out.write_le_u32(n3.wrapping_add(p));
        out
    }
}
pub type Salsa20KeyGenerator = SalsaKeyGenerator<20>;
pub type Salsa8KeyGenerator = SalsaKeyGenerator<8>;

/// One-off run of the salsa core function, does NOT perform the usual key management or mixing,
/// just the specified number of rounds.
pub fn salsa_core_function<const ROUNDS: usize>(inp: &[u8; 64]) -> [u8; 64] {
    let mut state: [u32; 16] = FromLEBytes::from_le_bytes(*inp);
    rounds!(ROUNDS, state);
    ToLEBytes::to_le_bytes(&state)
}

/// One-off run of the salsa core function, does NOT perform the usual key management or mixing,
/// just the specified number of rounds.
pub const fn salsa_core_function_u32<const ROUNDS: usize>(state: &mut [u32; 16]) {
    rounds!(ROUNDS, state);
}
