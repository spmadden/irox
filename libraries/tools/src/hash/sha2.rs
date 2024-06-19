// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Bog standard implementation of SHA2 / RFC6234.
//!

#![allow(clippy::indexing_slicing)]

use core::ops::{Index, IndexMut, Not};
use irox_bits::{Error, ErrorKind, MutBits};
pub use sha224_256::{SHA224, SHA256};
pub use sha384_512::{SHA384, SHA512};

/// Output size (bytes) for SHA224
pub const SHA224_OUTPUT_SIZE: usize = 28;
/// Output size (bytes) for SHA256
pub const SHA256_OUTPUT_SIZE: usize = 32;
/// Output size (bytes) for SHA384
pub const SHA384_OUTPUT_SIZE: usize = 48;
/// Output size (bytes) for SHA512
pub const SHA512_OUTPUT_SIZE: usize = 64;

macro_rules! CH {
    ($x:expr,$y:expr,$z:expr) => {{
        let x = $x;
        let y = $y;
        let z = $z;
        (x.bitand(y)).bitxor(x.not().bitand(z))
    }};
}
macro_rules! MAJ {
    ($x:expr,$y:expr,$z:expr) => {{
        let x = $x;
        let y = $y;
        let z = $z;
        x.bitand(y).bitxor(x.bitand(z)).bitxor(y.bitand(z))
    }};
}
macro_rules! sha2_impl {
    ($name:ident, $typ: ident, $init: expr, $block_size: expr, $word_size: expr, $output_size: expr) => {
        pub struct $name {
            alg: $typ<{ $block_size }, { $word_size }, { $output_size }>,
        }
        impl $name {
            pub fn new() -> Self {
                Self {
                    alg: $typ::new($init),
                }
            }
            pub fn write(&mut self, bytes: &[u8]) {
                self.alg.write(bytes)
            }
            pub fn finish(self) -> [u8; $output_size] {
                self.alg.finish()
            }
            pub fn hash(self, bytes: &[u8]) -> [u8; $output_size] {
                self.alg.hash(bytes)
            }
        }
        impl Default for $name {
            fn default() -> Self {
                $name::new()
            }
        }
        impl crate::hash::HashDigest<{ $block_size }, { $output_size }> for $name {
            fn write(&mut self, bytes: &[u8]) {
                $name::write(self, bytes)
            }

            fn hash(self, bytes: &[u8]) -> [u8; $output_size] {
                $name::hash(self, bytes)
            }

            fn finish(self) -> [u8; $output_size] {
                $name::finish(self)
            }
        }
        impl MutBits for $name {
            fn write_u8(&mut self, val: u8) -> Result<(), Error> {
                self.alg.write_u8(val)
            }
        }
    };
}

struct ShaU32Buf<const N: usize> {
    pub buf: [u32; N],
    pub size_bytes: usize,
}
impl<const N: usize> ShaU32Buf<N> {
    pub fn new() -> Self {
        Self {
            buf: [0u32; N],
            size_bytes: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.size_bytes
    }
    pub fn push_back(&mut self, val: u8) -> Result<(), Error> {
        self.write_u8(val)
    }
}

impl<const N: usize> MutBits for ShaU32Buf<N> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        let size = self.size_bytes;
        if size == (N << 2) {
            return Err(ErrorKind::OutOfMemory.into());
        }
        let idx = size >> 2;
        let shift = 24 - ((size & 0x3) << 3);
        self.buf[idx] &= (0xFFu32.wrapping_shl(shift as u32)).not();
        self.buf[idx] |= (val as u32).wrapping_shl(shift as u32);
        self.size_bytes += 1;
        Ok(())
    }

    fn write_be_u32(&mut self, val: u32) -> Result<(), Error> {
        let size = self.size_bytes;
        if size == (N << 2) {
            return Err(ErrorKind::OutOfMemory.into());
        }
        let idx = size >> 2;
        self.buf[idx] = val;
        self.size_bytes += 4;
        Ok(())
    }
}
impl<const N: usize> Index<usize> for ShaU32Buf<N> {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}
impl<const N: usize> IndexMut<usize> for ShaU32Buf<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}

mod sha224_256 {
    use crate::sha2::ShaU32Buf;
    use core::ops::{BitAnd, BitXor, Not};
    use irox_bits::{Error, MutBits};

    /// Block size (bytes) for SHA224
    pub const SHA224_BLOCK_SIZE: usize = 64;
    pub const SHA224_WORD_SIZE: usize = 16;
    /// Block size (bytes) for SHA256
    pub const SHA256_BLOCK_SIZE: usize = 64;
    pub const SHA256_WORD_SIZE: usize = 16;

    static KONSTANTS: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    macro_rules! BSIG0 {
        ($x:expr) => {{
            let x = $x;
            x.rotate_right(2)
                .bitxor(x.rotate_right(13))
                .bitxor(x.rotate_right(22))
        }};
    }
    macro_rules! BSIG1 {
        ($x:expr) => {{
            let x = $x;
            x.rotate_right(6)
                .bitxor(x.rotate_right(11))
                .bitxor(x.rotate_right(25))
        }};
    }
    macro_rules! SSIG0 {
        ($x:expr) => {{
            let x = $x;
            x.rotate_right(7)
                .bitxor(x.rotate_right(18))
                .bitxor(x.wrapping_shr(3))
        }};
    }
    macro_rules! SSIG1 {
        ($x:expr) => {{
            let x = $x;
            x.rotate_right(17)
                .bitxor(x.rotate_right(19))
                .bitxor(x.wrapping_shr(10))
        }};
    }
    const SHA224_INIT: [u32; 8] = [
        0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7,
        0xbefa4fa4,
    ];
    const SHA256_INIT: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    pub struct LittleSha2<const BLOCK_SIZE: usize, const WORD_SIZE: usize, const OUTPUT_SIZE: usize> {
        buf: ShaU32Buf<64>,
        written_length: u64,
        h0: u32,
        h1: u32,
        h2: u32,
        h3: u32,
        h4: u32,
        h5: u32,
        h6: u32,
        h7: u32,
    }

    impl<const BLOCK_SIZE: usize, const WORD_SIZE: usize, const OUTPUT_SIZE: usize>
        LittleSha2<BLOCK_SIZE, WORD_SIZE, OUTPUT_SIZE>
    {
        pub fn new(init: [u32; 8]) -> Self {
            let [h0, h1, h2, h3, h4, h5, h6, h7] = init;
            Self {
                buf: ShaU32Buf::new(),
                written_length: 0,
                h0,
                h1,
                h2,
                h3,
                h4,
                h5,
                h6,
                h7,
            }
        }

        fn try_chomp(&mut self) {
            if self.buf.len() < BLOCK_SIZE {
                return;
            }

            let words = &mut self.buf.buf;
            self.buf.size_bytes = 0;
            for idx in 16..64 {
                words[idx] = SSIG1!(words[idx - 2])
                    .wrapping_add(words[idx - 7])
                    .wrapping_add(SSIG0!(words[idx - 15]))
                    .wrapping_add(words[idx - 16]);
            }
            let mut a = self.h0;
            let mut b = self.h1;
            let mut c = self.h2;
            let mut d = self.h3;
            let mut e = self.h4;
            let mut f = self.h5;
            let mut g = self.h6;
            let mut h = self.h7;

            for t in 0..64 {
                let t1 = h
                    .wrapping_add(BSIG1!(e))
                    .wrapping_add(CH!(e, f, g))
                    .wrapping_add(KONSTANTS[t])
                    .wrapping_add(words[t]);
                let t2 = BSIG0!(a).wrapping_add(MAJ!(a, b, c));
                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(t1);
                d = c;
                c = b;
                b = a;
                a = t1.wrapping_add(t2);
            }
            self.h0 = self.h0.wrapping_add(a);
            self.h1 = self.h1.wrapping_add(b);
            self.h2 = self.h2.wrapping_add(c);
            self.h3 = self.h3.wrapping_add(d);
            self.h4 = self.h4.wrapping_add(e);
            self.h5 = self.h5.wrapping_add(f);
            self.h6 = self.h6.wrapping_add(g);
            self.h7 = self.h7.wrapping_add(h);
        }
        pub fn finish(mut self) -> [u8; OUTPUT_SIZE] {
            let mut modlen_bksize = (self.written_length & (BLOCK_SIZE - 1) as u64) as usize;
            let mut pad: usize = 0;

            let bitslen_minus_8 = BLOCK_SIZE - 8;

            if modlen_bksize >= bitslen_minus_8 {
                // append N bits/8 bytes;
                pad += BLOCK_SIZE - modlen_bksize;
                modlen_bksize = 0;
            }
            pad += bitslen_minus_8 - modlen_bksize;
            let _ = self.buf.push_back(0x80);
            pad -= 1;
            for _ in 0..pad {
                self.try_chomp();
                let _ = self.buf.push_back(0);
            }
            let _ = self.buf.write_be_u64(self.written_length << 3);
            self.try_chomp();

            let mut out: [u8; OUTPUT_SIZE] = [0; OUTPUT_SIZE];
            let mut v = out.as_mut_slice();
            let _ = v.write_be_u32(self.h0);
            let _ = v.write_be_u32(self.h1);
            let _ = v.write_be_u32(self.h2);
            let _ = v.write_be_u32(self.h3);
            let _ = v.write_be_u32(self.h4);
            let _ = v.write_be_u32(self.h5);
            let _ = v.write_be_u32(self.h6);
            let _ = v.write_be_u32(self.h7);
            out
        }
        ///
        /// Appends the bytes to the internal buffer.  NOTE: You must call 'finish' to get the final result.
        pub fn write(&mut self, bytes: &[u8]) {
            for b in bytes {
                let _ = self.buf.push_back(*b);
                self.written_length += 1;
                self.try_chomp();
            }
        }

        ///
        /// Hashes the provided bytes.
        pub fn hash(mut self, bytes: &[u8]) -> [u8; OUTPUT_SIZE] {
            self.write(bytes);
            self.finish()
        }
    }

    impl<const BLOCK_SIZE: usize, const WORD_SIZE: usize, const OUTPUT_SIZE: usize> MutBits
        for LittleSha2<BLOCK_SIZE, WORD_SIZE, OUTPUT_SIZE>
    {
        fn write_u8(&mut self, val: u8) -> Result<(), Error> {
            self.buf.write_u8(val)
        }
    }

    sha2_impl!(
        SHA224,
        LittleSha2,
        SHA224_INIT,
        SHA224_BLOCK_SIZE,
        SHA224_WORD_SIZE,
        super::SHA224_OUTPUT_SIZE
    );
    sha2_impl!(
        SHA256,
        LittleSha2,
        SHA256_INIT,
        SHA256_BLOCK_SIZE,
        SHA256_WORD_SIZE,
        super::SHA256_OUTPUT_SIZE
    );
}

mod sha384_512 {
    use crate::buf::{Buffer, RoundBuffer};
    use core::ops::{BitAnd, BitXor, Not};
    use irox_bits::{Bits, Error, MutBits};

    /// Block size (bytes) for SHA384
    pub const SHA384_BLOCK_SIZE: usize = 128;
    pub const SHA384_WORD_SIZE: usize = 16;
    /// Block size (bytes) for SHA512
    pub const SHA512_BLOCK_SIZE: usize = 128;
    pub const SHA512_WORD_SIZE: usize = 16;

    macro_rules! BSIG0 {
        ($x:expr) => {{
            let x = $x;
            x.rotate_right(2)
                .bitxor(x.rotate_right(13))
                .bitxor(x.rotate_right(22))
        }};
    }
    macro_rules! BSIG1 {
        ($x:expr) => {{
            let x = $x;
            x.rotate_right(6)
                .bitxor(x.rotate_right(11))
                .bitxor(x.rotate_right(25))
        }};
    }
    macro_rules! SSIG0 {
        ($x:expr) => {{
            let x = $x;
            x.rotate_right(7)
                .bitxor(x.rotate_right(18))
                .bitxor(x.wrapping_shr(3))
        }};
    }
    macro_rules! SSIG1 {
        ($x:expr) => {{
            let x = $x;
            x.rotate_right(17)
                .bitxor(x.rotate_right(19))
                .bitxor(x.wrapping_shr(10))
        }};
    }

    pub static KONSTANTS: [u64; 80] = [
        0x428a2f98d728ae22,
        0x7137449123ef65cd,
        0xb5c0fbcfec4d3b2f,
        0xe9b5dba58189dbbc,
        0x3956c25bf348b538,
        0x59f111f1b605d019,
        0x923f82a4af194f9b,
        0xab1c5ed5da6d8118,
        0xd807aa98a3030242,
        0x12835b0145706fbe,
        0x243185be4ee4b28c,
        0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f,
        0x80deb1fe3b1696b1,
        0x9bdc06a725c71235,
        0xc19bf174cf692694,
        0xe49b69c19ef14ad2,
        0xefbe4786384f25e3,
        0x0fc19dc68b8cd5b5,
        0x240ca1cc77ac9c65,
        0x2de92c6f592b0275,
        0x4a7484aa6ea6e483,
        0x5cb0a9dcbd41fbd4,
        0x76f988da831153b5,
        0x983e5152ee66dfab,
        0xa831c66d2db43210,
        0xb00327c898fb213f,
        0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2,
        0xd5a79147930aa725,
        0x06ca6351e003826f,
        0x142929670a0e6e70,
        0x27b70a8546d22ffc,
        0x2e1b21385c26c926,
        0x4d2c6dfc5ac42aed,
        0x53380d139d95b3df,
        0x650a73548baf63de,
        0x766a0abb3c77b2a8,
        0x81c2c92e47edaee6,
        0x92722c851482353b,
        0xa2bfe8a14cf10364,
        0xa81a664bbc423001,
        0xc24b8b70d0f89791,
        0xc76c51a30654be30,
        0xd192e819d6ef5218,
        0xd69906245565a910,
        0xf40e35855771202a,
        0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8,
        0x1e376c085141ab53,
        0x2748774cdf8eeb99,
        0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63,
        0x4ed8aa4ae3418acb,
        0x5b9cca4f7763e373,
        0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc,
        0x78a5636f43172f60,
        0x84c87814a1f0ab72,
        0x8cc702081a6439ec,
        0x90befffa23631e28,
        0xa4506cebde82bde9,
        0xbef9a3f7b2c67915,
        0xc67178f2e372532b,
        0xca273eceea26619c,
        0xd186b8c721c0c207,
        0xeada7dd6cde0eb1e,
        0xf57d4f7fee6ed178,
        0x06f067aa72176fba,
        0x0a637dc5a2c898a6,
        0x113f9804bef90dae,
        0x1b710b35131c471b,
        0x28db77f523047d84,
        0x32caab7b40c72493,
        0x3c9ebe0a15c9bebc,
        0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6,
        0x597f299cfc657e2a,
        0x5fcb6fab3ad6faec,
        0x6c44198c4a475817,
    ];

    pub const SHA384_INIT: [u64; 8] = [
        0xcbbb9d5dc1059ed8,
        0x629a292a367cd507,
        0x9159015a3070dd17,
        0x152fecd8f70e5939,
        0x67332667ffc00b31,
        0x8eb44a8768581511,
        0xdb0c2e0d64f98fa7,
        0x47b5481dbefa4fa4,
    ];
    pub const SHA512_INIT: [u64; 8] = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];

    pub struct BiggerSha2<const BLOCK_SIZE: usize, const WORD_SIZE: usize, const OUTPUT_SIZE: usize> {
        buf: RoundBuffer<BLOCK_SIZE, u8>,
        written_length: u128,
        h0: u64,
        h1: u64,
        h2: u64,
        h3: u64,
        h4: u64,
        h5: u64,
        h6: u64,
        h7: u64,
    }

    impl<const BLOCK_SIZE: usize, const WORD_SIZE: usize, const OUTPUT_SIZE: usize>
        BiggerSha2<BLOCK_SIZE, WORD_SIZE, OUTPUT_SIZE>
    {
        pub fn new(init: [u64; 8]) -> Self {
            let [h0, h1, h2, h3, h4, h5, h6, h7] = init;
            Self {
                buf: RoundBuffer::default(),
                written_length: 0,
                h0,
                h1,
                h2,
                h3,
                h4,
                h5,
                h6,
                h7,
            }
        }

        fn try_chomp(&mut self) {
            if self.buf.len() < BLOCK_SIZE {
                return;
            }

            let mut words = [0u64; 16];
            for word in &mut words {
                *word = self.buf.read_be_u64().unwrap_or_default().swap_bytes();
            }
            for idx in 16..80 {
                words[idx] = SSIG1!(words[idx - 2])
                    .wrapping_add(words[idx - 7])
                    .wrapping_add(SSIG0!(words[idx - 15]))
                    .wrapping_add(words[idx - 16]);
            }
            let mut a = self.h0;
            let mut b = self.h1;
            let mut c = self.h2;
            let mut d = self.h3;
            let mut e = self.h4;
            let mut f = self.h5;
            let mut g = self.h6;
            let mut h = self.h7;

            for t in 0..80 {
                let t1 = h
                    .wrapping_add(BSIG1!(e))
                    .wrapping_add(CH!(e, f, g))
                    .wrapping_add(KONSTANTS[t])
                    .wrapping_add(words[t]);
                let t2 = BSIG0!(a).wrapping_add(MAJ!(a, b, c));
                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(t1);
                d = c;
                c = b;
                b = a;
                a = t1.wrapping_add(t2);
            }
            self.h0 = self.h0.wrapping_add(a);
            self.h1 = self.h1.wrapping_add(b);
            self.h2 = self.h2.wrapping_add(c);
            self.h3 = self.h3.wrapping_add(d);
            self.h4 = self.h4.wrapping_add(e);
            self.h5 = self.h5.wrapping_add(f);
            self.h6 = self.h6.wrapping_add(g);
            self.h7 = self.h7.wrapping_add(h);
        }
        pub fn finish(mut self) -> [u8; OUTPUT_SIZE] {
            let mut modlen_bksize = (self.written_length & (BLOCK_SIZE - 1) as u128) as usize;
            let mut pad: usize = 0;

            let bitslen_minus_8 = BLOCK_SIZE - 8;

            if modlen_bksize >= bitslen_minus_8 {
                // append 64 bits/8 bytes;
                pad += BLOCK_SIZE - modlen_bksize;
                modlen_bksize = 0;
            }
            pad += bitslen_minus_8 - modlen_bksize;
            let _ = self.buf.push_back(0x80);
            pad -= 1;
            for _ in 0..pad {
                self.try_chomp();
                let _ = self.buf.push_back(0);
            }
            let _ = self.buf.write_be_u128(self.written_length << 3);
            self.try_chomp();

            let mut out: [u8; OUTPUT_SIZE] = [0; OUTPUT_SIZE];
            let mut v = out.as_mut_slice();
            let _ = v.write_be_u64(self.h0);
            let _ = v.write_be_u64(self.h1);
            let _ = v.write_be_u64(self.h2);
            let _ = v.write_be_u64(self.h3);
            let _ = v.write_be_u64(self.h4);
            let _ = v.write_be_u64(self.h5);
            let _ = v.write_be_u64(self.h6);
            let _ = v.write_be_u64(self.h7);
            out
        }
        ///
        /// Appends the bytes to the internal buffer.  NOTE: You must call 'finish' to get the final result.
        pub fn write(&mut self, bytes: &[u8]) {
            for b in bytes {
                let _ = self.buf.push_back(*b);
                self.written_length += 1;
                self.try_chomp();
            }
        }

        ///
        /// Hashes the provided bytes.
        pub fn hash(mut self, bytes: &[u8]) -> [u8; OUTPUT_SIZE] {
            self.write(bytes);
            self.finish()
        }
    }

    impl<const BLOCK_SIZE: usize, const WORD_SIZE: usize, const OUTPUT_SIZE: usize> MutBits
        for BiggerSha2<BLOCK_SIZE, WORD_SIZE, OUTPUT_SIZE>
    {
        fn write_u8(&mut self, val: u8) -> Result<(), Error> {
            self.buf.write_u8(val)
        }
    }
    sha2_impl!(
        SHA384,
        BiggerSha2,
        SHA384_INIT,
        SHA384_BLOCK_SIZE,
        SHA384_WORD_SIZE,
        super::SHA384_OUTPUT_SIZE
    );
    sha2_impl!(
        SHA512,
        BiggerSha2,
        SHA512_INIT,
        SHA512_BLOCK_SIZE,
        SHA512_WORD_SIZE,
        super::SHA512_OUTPUT_SIZE
    );
}

#[cfg(test)]
mod test {
    use crate::sha2::{SHA224, SHA224_OUTPUT_SIZE, SHA256, SHA256_OUTPUT_SIZE};
    use irox_bits::MutBits;

    fn u32s_to_arr<const T: usize>(input: &[u32]) -> [u8; T] {
        let mut out = [0u8; T];
        let mut pos = out.as_mut_slice();
        for v in input {
            let _ = pos.write_all_bytes(&v.to_be_bytes());
        }
        out
    }
    macro_rules! to_arr {
        ($out:expr, $($elem:expr) +) => {
            u32s_to_arr::<$out>(&[
                $($elem),+
            ])
        };
    }
    #[test]
    pub fn sha224_wiki1() {
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0xd14a028c 0x2a3a2bc9 0x476102bb 0x288234c4 0x15a2b01f 0x828ea62a 0xc5b3e42f),
            SHA224::default().hash(&[])
        );
    }
    #[test]
    pub fn sha224_wiki2() {
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0x730e109b 0xd7a8a32b 0x1cb9d9a0 0x9aa2325d 0x2430587d 0xdbc0c38b 0xad911525),
            SHA224::default().hash("The quick brown fox jumps over the lazy dog".as_bytes())
        );
    }
    #[test]
    pub fn sha224_wiki3() {
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0x619cba8e 0x8e05826e 0x9b8c519c 0x0a5c68f4 0xfb653e8a 0x3d8aa04b 0xb2c8cd4c),
            SHA224::default().hash("The quick brown fox jumps over the lazy dog.".as_bytes())
        );
    }
    #[test]
    pub fn sha224_nist1() {
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0xe33f9d75 0xe6ae1369 0xdbabf81b 0x96b4591a 0xe46bba30 0xb591a6b6 0xc62542b5),
            SHA224::default().hash(&[0xff])
        );
    }
    #[test]
    pub fn sha224_nist2() {
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0xfd19e746 0x90d29146 0x7ce59f07 0x7df31163 0x8f1c3a46 0xe510d0e4 0x9a67062d ),
            SHA224::default().hash(&0xe5e09924u32.to_be_bytes())
        );
    }
    #[test]
    pub fn sha224_nist3() {
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0x5c3e25b6 0x9d0ea26f 0x260cfae8 0x7e23759e 0x1eca9d1e 0xcc9fbf3c 0x62266804),
            SHA224::default().hash(&[0u8; 56])
        );
    }
    #[test]
    pub fn sha224_nist4() {
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0x3706197f 0x66890a41 0x779dc879 0x1670522e 0x136fafa2 0x48746857 0x15bd0a8a),
            SHA224::default().hash(&[0x51u8; 1000])
        );
    }
    #[test]
    pub fn sha224_nist5() {
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0xa8d0c66b 0x5c6fdfd8 0x36eb3c6d 0x04d32dfe 0x66c3b1f1 0x68b488bf 0x4c9c66ce),
            SHA224::default().hash(&[0x41u8; 1000])
        );
    }
    #[test]
    pub fn sha224_nist6() {
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0xcb00ecd0 0x3788bf6c 0x0908401e 0x0eb053ac 0x61f35e7e 0x20a2cfd7 0xbd96d640),
            SHA224::default().hash(&[0x99u8; 1005])
        );
    }
    #[test]
    pub fn sha224_nist7() {
        let mut sha = SHA224::new();
        for _ in 0..1000000 {
            sha.write(&[0]);
        }
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0x3a5d74b6 0x8f14f3a4 0xb2be9289 0xb8d37067 0x2d0b3d2f 0x53bc303c 0x59032df3),
            sha.finish()
        );
    }
    #[test]
    #[cfg_attr(not(feature = "_toobig-tests"), ignore)]
    pub fn sha224_nist8() {
        let mut sha = SHA224::new();
        for _ in 0..0x20000000 {
            sha.write(&[0x41]);
        }
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0xc4250083 0xcf8230bf 0x21065b30 0x14baaaf9 0xf76fecef 0xc21f91cf 0x237dedc9),
            sha.finish()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "_toobig-tests"), ignore)]
    pub fn sha224_nist9() {
        let mut sha = SHA224::new();
        for _ in 0..0x41000000 {
            sha.write(&[0x0]);
        }
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0x014674ab 0xc5cb9801 0x99935695 0xaf22fab6 0x83748f42 0x61d4c649 0x2b77c543
            ),
            sha.finish()
        );
    }
    #[test]
    #[cfg_attr(not(feature = "_toobig-tests"), ignore)]
    pub fn sha224_nist10() {
        let mut sha = SHA224::new();
        for _ in 0..0x6000003f {
            sha.write(&[0x84]);
        }
        assert_eq_hex_slice!(
            to_arr!(SHA224_OUTPUT_SIZE, 0xa654b50b 0x767a8323 0xc5b519f4 0x67d86698 0x37142881 0xdc7ad368 0xa7d5ef8f),
            sha.finish()
        );
    }

    #[test]
    pub fn sha256_nist1() {
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0x68325720 0xaabd7c82 0xf30f554b 0x313d0570 0xc95accbb 0x7dc4b5aa 0xe11204c0 0x8ffe732b),
            SHA256::default().hash(&[0xbd])
        );
    }
    #[test]
    pub fn sha256_nist2() {
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0x7abc22c0 0xae5af26c 0xe93dbb94 0x433a0e0b 0x2e119d01 0x4f8e7f65 0xbd56c61c 0xcccd9504),
            SHA256::default().hash(&0xc98c8e55u32.to_be_bytes())
        );
    }
    #[test]
    pub fn sha256_nist3() {
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0x02779466 0xcdec1638 0x11d07881 0x5c633f21 0x90141308 0x1449002f 0x24aa3e80 0xf0b88ef7),
            SHA256::default().hash(&[0u8; 55])
        );
    }
    #[test]
    pub fn sha256_nist4() {
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0xd4817aa5 0x497628e7 0xc77e6b60 0x6107042b 0xbba31308 0x88c5f47a 0x375e6179 0xbe789fbb),
            SHA256::default().hash(&[0x0u8; 56])
        );
    }
    #[test]
    pub fn sha256_nist5() {
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0x65a16cb7 0x861335d5 0xace3c607 0x18b5052e 0x44660726 0xda4cd13b 0xb745381b 0x235a1785),
            SHA256::default().hash(&[0x0u8; 57])
        );
    }
    #[test]
    pub fn sha256_nist6() {
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0xf5a5fd42 0xd16a2030 0x2798ef6e 0xd309979b 0x43003d23 0x20d9f0e8 0xea9831a9 0x2759fb4b),
            SHA256::default().hash(&[0x0u8; 64])
        );
    }
    #[test]
    pub fn sha256_nist7() {
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0x541b3e9d 0xaa09b20b 0xf85fa273 0xe5cbd3e8 0x0185aa4e 0xc298e765 0xdb87742b 0x70138a53),
            SHA256::default().hash(&[0x0u8; 1000])
        );
    }
    #[test]
    pub fn sha256_nist8() {
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0xc2e68682 0x3489ced2 0x017f6059 0xb8b23931 0x8b6364f6 0xdcd835d0 0xa519105a 0x1eadd6e4),
            SHA256::default().hash(&[0x41u8; 1000])
        );
    }
    #[test]
    pub fn sha256_nist9() {
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0xf4d62dde 0xc0f3dd90 0xea1380fa 0x16a5ff8d 0xc4c54b21 0x740650f2 0x4afc4120 0x903552b0),
            SHA256::default().hash(&[0x55u8; 1005])
        );
    }
    #[test]
    #[cfg_attr(not(feature = "_toobig-tests"), ignore)]
    pub fn sha256_nist10() {
        let mut sha = SHA256::new();
        for _ in 0..1000000 {
            sha.write(&[0]);
        }
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0xd29751f2 0x649b32ff 0x572b5e0a 0x9f541ea6 0x60a50f94 0xff0beedf 0xb0b692b9 0x24cc8025),
            sha.finish()
        );
    }
    #[test]
    #[cfg_attr(not(feature = "_toobig-tests"), ignore)]
    pub fn sha256_nist11() {
        let mut sha = SHA256::new();
        for _ in 0..0x20000000 {
            sha.write(&[0x5a]);
        }
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0x15a1868c 0x12cc5395 0x1e182344 0x277447cd 0x0979536b 0xadcc512a 0xd24c67e9 0xb2d4f3dd),
            sha.finish()
        );
    }
    #[test]
    #[cfg_attr(not(feature = "_toobig-tests"), ignore)]
    pub fn sha256_nist12() {
        let mut sha = SHA256::new();
        for _ in 0..0x41000000 {
            sha.write(&[0x0]);
        }
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0x461c19a9 0x3bd4344f 0x9215f5ec 0x64357090 0x342bc66b 0x15a14831 0x7d276e31 0xcbc20b53),
            sha.finish()
        );
    }
    #[test]
    #[cfg_attr(not(feature = "_toobig-tests"), ignore)]
    pub fn sha256_nist13() {
        let mut sha = SHA256::new();
        for _ in 0..0x6000003e {
            sha.write(&[0x42]);
        }
        assert_eq_hex_slice!(
            to_arr!(SHA256_OUTPUT_SIZE, 0xc23ce8a7 0x895f4b21 0xec0daf37 0x920ac0a2 0x62a22004 0x5a03eb2d 0xfed48ef9 0xb05aabea),
            sha.finish()
        );
    }
}
