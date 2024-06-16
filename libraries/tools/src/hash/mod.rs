// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Message Hash (Digest) functions
//!

#![allow(clippy::indexing_slicing)]

use core::ops::BitXorAssign;
use irox_bits::MutBits;

pub mod md5;
pub mod murmur3;
pub mod sha1;
pub mod sha2;

/// Generic trait to describe a hash function
pub trait HashDigest<const BLOCK_SIZE: usize, const OUTPUT_SIZE: usize>: Default {
    fn write(&mut self, bytes: &[u8]);
    fn hash(self, bytes: &[u8]) -> [u8; OUTPUT_SIZE];
    fn finish(self) -> [u8; OUTPUT_SIZE];
}

/// HMAC using the SHA1 algorithm
pub type HMACSHA1 = HMAC<{ sha1::BLOCK_SIZE }, { sha1::OUTPUT_SIZE }, sha1::SHA1>;
/// HMAC using the MD5 algorithm
pub type HMACMD5 = HMAC<{ md5::BLOCK_SIZE }, { md5::OUTPUT_SIZE }, md5::MD5>;

///
/// Implementation of [RFC 2104](https://datatracker.ietf.org/doc/html/rfc2104) based on the [Wikipedia](https://en.wikipedia.org/wiki/HMAC#Implementation) algorithm
///
///
/// **THIS SHOULD NOT BE USED FOR ANYTHING SECURITY RELATED**
pub struct HMAC<
    const BLOCK_SIZE: usize,
    const OUTPUT_SIZE: usize,
    T: HashDigest<BLOCK_SIZE, OUTPUT_SIZE>,
> {
    opad: [u8; BLOCK_SIZE],
    alg: T,
}

impl<const BLOCK_SIZE: usize, const OUTPUT_SIZE: usize, T: HashDigest<BLOCK_SIZE, OUTPUT_SIZE>>
    HMAC<BLOCK_SIZE, OUTPUT_SIZE, T>
{
    /// Creates a new HMAC, initialized with the provided key
    pub fn new(in_key: &[u8]) -> Self {
        let keylen = in_key.len();
        let mut key = [0u8; BLOCK_SIZE];
        if keylen >= BLOCK_SIZE {
            let hash = T::default().hash(in_key) as [u8; OUTPUT_SIZE];

            let _ = key.as_mut_slice().write_all_bytes(&hash);
        } else {
            let _ = key.as_mut_slice().write_all_bytes(in_key);
        }
        let mut ipad = [0x36u8; BLOCK_SIZE];
        let mut opad = [0x5Cu8; BLOCK_SIZE];
        let mut alg = T::default();

        for idx in 0..BLOCK_SIZE {
            let k = key[idx];
            ipad[idx].bitxor_assign(k);
            opad[idx].bitxor_assign(k);
        }
        alg.write(ipad.as_slice());

        Self { alg, opad }
    }

    pub fn write(&mut self, bytes: &[u8]) {
        self.alg.write(bytes)
    }
    ///
    /// Hashes the provided bytes.
    pub fn hash(mut self, bytes: &[u8]) -> [u8; OUTPUT_SIZE] {
        self.write(bytes);
        self.finish()
    }
    ///
    /// Finishes the hash and returns the result.
    pub fn finish(self) -> [u8; OUTPUT_SIZE] {
        let Self { alg, opad } = self;
        let inner = alg.finish();

        let mut outer = T::default();
        outer.write(&opad);
        outer.hash(&inner)
    }
}

#[cfg(test)]
mod hmac_tests {
    use crate::*;

    #[test]
    pub fn wikitest1() {
        assert_eq_hex_slice!(
            0x80070713463e7749b90c2dc24911e275u128.to_be_bytes(),
            HMACMD5::new("key".as_bytes())
                .hash("The quick brown fox jumps over the lazy dog".as_bytes())
        );
    }

    #[test]
    pub fn wikitest2() {
        assert_eq_hex_slice!(
            [
                0xde, 0x7c, 0x9b, 0x85, 0xb8, 0xb7, 0x8a, 0xa6, 0xbc, 0x8a, 0x7a, 0x36, 0xf7, 0x0a,
                0x90, 0x70, 0x1c, 0x9d, 0xb4, 0xd9
            ],
            HMACSHA1::new("key".as_bytes())
                .hash("The quick brown fox jumps over the lazy dog".as_bytes())
        );
    }

    #[test]
    pub fn rfctest1() {
        assert_eq_hex_slice!(
            0x9294727a3638bb1c13f48ef8158bfc9d_u128.to_be_bytes(),
            HMACMD5::new(&[0x0B; 16]).hash("Hi There".as_bytes())
        );
    }
    #[test]
    pub fn rfctest2() {
        assert_eq_hex_slice!(
            0x750c783e6ab0b503eaa86e310a5db738_u128.to_be_bytes(),
            HMACMD5::new("Jefe".as_bytes()).hash("what do ya want for nothing?".as_bytes())
        );
    }
    #[test]
    pub fn rfctest3() {
        assert_eq_hex_slice!(
            0x56be34521d144c88dbb8c733f0e8b3f6_u128.to_be_bytes(),
            HMACMD5::new(&[0xAA; 16]).hash(&[0xDD; 50])
        );
    }
}
