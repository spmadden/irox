// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{array_split_16, u64_to_u32, FromBEBytes, MutBits};
use irox_tools::buf::{Buffer, RoundU8Buffer};

const E1: u32 = 0x65787061;
const E2: u32 = 0x6e642033;
const E3: u32 = 0x322d6279;
const E4: u32 = 0x7465206b;
const ROUNDS: usize = 20;
macro_rules! qr {
    (
        $a:expr,
        $b:expr,
        $c:expr,
        $d:expr
    ) => {
        *$a = ($a.wrapping_add(*$b));
        *$d ^= *$a;
        *$d = ($d.rotate_left(16));
        *$c = ($c.wrapping_add(*$d));
        *$b ^= *$c;
        *$b = ($b.rotate_left(12));
        *$a = ($a.wrapping_add(*$b));
        *$d ^= *$a;
        *$d = ($d.rotate_left(8));
        *$c = ($c.wrapping_add(*$d));
        *$b ^= *$c;
        *$b = ($b.rotate_left(7));
    };
}

pub struct Chacha20KeyGenerator;
impl Chacha20KeyGenerator {
    pub fn generate(key: [u8; 32], counter: u64, nonce: u64) -> RoundU8Buffer<64> {
        let (a, b) = array_split_16(key);
        let ku = <[u32; 4]>::from_be_bytes(a);
        let kl = <[u32; 4]>::from_be_bytes(b);
        let [k1, k2, k3, k4] = ku;
        let [k5, k6, k7, k8] = kl;
        let [c1, c2] = u64_to_u32(counter);
        let [n1, n2] = u64_to_u32(nonce);
        let mut state = [
            E1, E2, E3, E4, k1, k2, k3, k4, k5, k6, k7, k8, c1, c2, n1, n2,
        ];
        for _i in 0..(ROUNDS / 2) {
            qr!(&mut state[0], &mut state[4], &mut state[8], &mut state[12]);
            qr!(&mut state[1], &mut state[5], &mut state[9], &mut state[13]);
            qr!(&mut state[2], &mut state[6], &mut state[10], &mut state[14]);
            qr!(&mut state[3], &mut state[7], &mut state[11], &mut state[15]);

            qr!(&mut state[0], &mut state[5], &mut state[10], &mut state[15]);
            qr!(&mut state[1], &mut state[6], &mut state[11], &mut state[12]);
            qr!(&mut state[2], &mut state[7], &mut state[8], &mut state[13]);
            qr!(&mut state[3], &mut state[4], &mut state[9], &mut state[14]);
        }
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = state;
        let mut out = RoundU8Buffer::<64>::default();
        let _ = out.write_be_u32(E1.wrapping_add(a));
        let _ = out.write_be_u32(E2.wrapping_add(b));
        let _ = out.write_be_u32(E3.wrapping_add(c));
        let _ = out.write_be_u32(E4.wrapping_add(d));
        let _ = out.write_be_u32(k1.wrapping_add(e));
        let _ = out.write_be_u32(k2.wrapping_add(f));
        let _ = out.write_be_u32(k3.wrapping_add(g));
        let _ = out.write_be_u32(k4.wrapping_add(h));
        let _ = out.write_be_u32(k5.wrapping_add(i));
        let _ = out.write_be_u32(k6.wrapping_add(j));
        let _ = out.write_be_u32(k7.wrapping_add(k));
        let _ = out.write_be_u32(k8.wrapping_add(l));
        let _ = out.write_be_u32(c1.wrapping_add(m));
        let _ = out.write_be_u32(c2.wrapping_add(n));
        let _ = out.write_be_u32(n1.wrapping_add(o));
        let _ = out.write_be_u32(n2.wrapping_add(p));
        out
    }
}

pub struct Chacha20KeyStream {
    key: [u8; 32],
    counter: u64,
    nonce: u64,
    buf: RoundU8Buffer<64>,
}
impl Chacha20KeyStream {
    pub fn new(key: [u8; 32], nonce: u64) -> Chacha20KeyStream {
        Self {
            key,
            nonce,
            counter: 0,
            buf: RoundU8Buffer::<64>::default(),
        }
    }

    pub fn next_key(&mut self) -> u8 {
        if self.buf.is_empty() {
            self.buf = Chacha20KeyGenerator::generate(self.key, self.counter, self.nonce);
            self.counter += 1;
        }
        self.buf.pop_front().unwrap_or_default()
    }
}
pub struct Chacha20 {
    keystream: Chacha20KeyStream,
}
impl Chacha20 {
    pub fn new(key: [u8; 32], nonce: u64) -> Chacha20 {
        Self {
            keystream: Chacha20KeyStream::new(key, nonce),
        }
    }
    pub fn encrypt(&mut self, input: &[u8], output: &mut [u8]) -> usize {
        let mut used = 0;
        for (o, i) in output.iter_mut().zip(input.iter()) {
            *o = *i ^ self.keystream.next_key();
            used += 1;
        }
        used
    }
}
