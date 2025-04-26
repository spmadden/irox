// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Message Hash (Digest) functions
//!

#![allow(clippy::indexing_slicing)]
#![deny(clippy::integer_division_remainder_used)]

use crate::cfg_feature_alloc;
pub use blake2::*;
use core::ops::BitXorAssign;
use irox_bits::MutBits;
pub use md5::MD5;
pub use murmur3::{murmur3_128, murmur3_128_seed};
pub use sha1::SHA1;
pub use sha2::{SHA224, SHA256, SHA384, SHA512};

mod blake2;
pub mod md5;
pub mod murmur3;
pub mod sha1;
pub mod sha2;
pub mod sixwords;
pub mod viz;

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
/// HMAC using the SHA224 algorithm
pub type HMACSHA224 = HMAC<{ sha2::SHA224_BLOCK_SIZE }, { sha2::SHA224_OUTPUT_SIZE }, sha2::SHA224>;
/// HMAC using the SHA256 algorithm
pub type HMACSHA256 = HMAC<{ sha2::SHA256_BLOCK_SIZE }, { sha2::SHA256_OUTPUT_SIZE }, sha2::SHA256>;
/// HMAC using the SHA384 algorithm
pub type HMACSHA384 = HMAC<{ sha2::SHA384_BLOCK_SIZE }, { sha2::SHA384_OUTPUT_SIZE }, sha2::SHA384>;
/// HMAC using the SHA512 algorithm
pub type HMACSHA512 = HMAC<{ sha2::SHA512_BLOCK_SIZE }, { sha2::SHA512_OUTPUT_SIZE }, sha2::SHA512>;
/// HMAC using the BLAKE2s algorithm
pub type HMACBLAKE2s = HMAC<64, 32, BLAKE2s256>;
/// HMAC using the BLAKE2b algorithm
pub type HMACBLAKE2b = HMAC<128, 64, BLAKE2b512>;

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
        if keylen > BLOCK_SIZE {
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

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HashAlgorithm {
    MD5,
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    Murmur3_128,
    Murmur3_32,
    BLAKE2s128,
    BLAKE2s160,
    BLAKE2s224,
    BLAKE2s256,
    BLAKE2b160,
    BLAKE2b224,
    BLAKE2b256,
    BLAKE2b384,
    BLAKE2b512,
}
impl TryFrom<&str> for HashAlgorithm {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "md5" => Self::MD5,
            "sha1" => Self::SHA1,
            "sha256" => Self::SHA256,
            "sha512" => Self::SHA512,
            "murmur3_128" | "murmur3" | "m3" => Self::Murmur3_128,
            "b2" | "b2b" | "blake2b" | "blake2b512" => Self::BLAKE2b512,
            "b2s" | "blake2s" | "blake2s256" => Self::BLAKE2s256,
            _ => return Err(()),
        })
    }
}

cfg_feature_alloc! {
    extern crate alloc;
    use irox_bits::{Error, ToBEBytes};
    use crate::hash::murmur3::{Murmur3_128, Murmur3_32};


    pub enum Hasher {
        MD5(MD5),
        SHA1(SHA1),
        SHA224(SHA224),
        SHA256(SHA256),
        SHA384(SHA384),
        SHA512(SHA512),
        Murmur3_128(Murmur3_128),
        Murmur3_32(Murmur3_32),
        BLAKE2b512(BLAKE2b512),
        BLAKE2s256(BLAKE2s256),
        BLAKE2s128(BLAKE2s128),
        BLAKE2s160(BLAKE2s160),
        BLAKE2s224(BLAKE2s224),
        BLAKE2b160(BLAKE2b160),
        BLAKE2b224(BLAKE2b224),
        BLAKE2b256(BLAKE2b256),
        BLAKE2b384(BLAKE2b384),
    }
    macro_rules! impl_hash_from {
        ($value:ident, [$($hash:ident),+]) => {
            match $value {
                $(
                    HashAlgorithm::$hash => Ok(Hasher::$hash(<$hash>::default())),
                )*
                _ => todo!()
            }
        };
    }
    impl TryFrom<HashAlgorithm> for Hasher {
        type Error = Error;

        fn try_from(value: HashAlgorithm) -> Result<Self, Self::Error> {
            impl_hash_from!(value,
                [
                    MD5, SHA1, SHA256, SHA384, SHA512, Murmur3_128, Murmur3_32,
                    BLAKE2s128, BLAKE2s160, BLAKE2s224, BLAKE2s256,
                    BLAKE2b160, BLAKE2b224, BLAKE2b256, BLAKE2b384, BLAKE2b512
                ])
        }
    }
     macro_rules! impl_hash_write {
        ($value:ident, $val:ident, [$($hash:ident),+]) => {
            match $value {
                $(
                    Hasher::$hash(h) => h.write($val),
                )*
                _ => todo!()
            }
        };
    }

    macro_rules! impl_hash_finish {
        ($value:ident, [$($hash:ident),+]) => {
            match $value {
                $(
                    Hasher::$hash(v) => alloc::boxed::Box::from(v.finish().to_be_bytes()),
                )*
                _ => todo!()
            }
        };
    }
    impl Hasher {
        pub fn write(&mut self, val: &[u8]) {
            impl_hash_write!(self, val,
                [
                    MD5, SHA1, SHA256, SHA384, SHA512, Murmur3_128, Murmur3_32,
                    BLAKE2s128, BLAKE2s160, BLAKE2s224, BLAKE2s256,
                    BLAKE2b160, BLAKE2b224, BLAKE2b256, BLAKE2b384, BLAKE2b512
                ])
        }
        pub fn finish(self) -> alloc::boxed::Box<[u8]> {
            impl_hash_finish!(self,
                [
                    MD5, SHA1, SHA256, SHA384, SHA512, Murmur3_128, Murmur3_32,
                    BLAKE2s128, BLAKE2s160, BLAKE2s224, BLAKE2s256,
                    BLAKE2b160, BLAKE2b224, BLAKE2b256, BLAKE2b384, BLAKE2b512
                ])
        }
    }

}
#[cfg(test)]
mod hmac_tests {
    use crate::hash::*;

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
