// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{Error, ErrorKind};
use irox_enums::{EnumIterItem, EnumName};

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIterItem, EnumName)]
pub enum HashAlgorithm {
    MD5,
    SHA1,
    RIPEMD160,
    SHA256,
    SHA384,
    SHA512,
    SHA224,
    SHA3_256,
    SHA3_512,
}

impl HashAlgorithm {
    pub fn get_id(&self) -> u8 {
        match self {
            HashAlgorithm::MD5 => 1,
            HashAlgorithm::SHA1 => 2,
            HashAlgorithm::RIPEMD160 => 3,
            HashAlgorithm::SHA256 => 8,
            HashAlgorithm::SHA384 => 9,
            HashAlgorithm::SHA512 => 10,
            HashAlgorithm::SHA224 => 11,
            HashAlgorithm::SHA3_256 => 12,
            HashAlgorithm::SHA3_512 => 14,
        }
    }
}
impl TryFrom<u8> for HashAlgorithm {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        for alg in Self::iter_items() {
            if alg.get_id() == value {
                return Ok(alg);
            }
        }
        Err(ErrorKind::InvalidData.into())
    }
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIterItem, EnumName)]
pub enum SymmetricKeyAlgorithm {
    IDEA,
    TripleDES,
    CAST5,
    Blowfish128,
    AES128,
    AES192,
    AES256,
    Twofish256,
    Camellia128,
    Camellia192,
    Camellia256,
}
impl SymmetricKeyAlgorithm {
    pub fn get_id(&self) -> u8 {
        match self {
            SymmetricKeyAlgorithm::IDEA => 1,
            SymmetricKeyAlgorithm::TripleDES => 2,
            SymmetricKeyAlgorithm::CAST5 => 3,
            SymmetricKeyAlgorithm::Blowfish128 => 4,
            SymmetricKeyAlgorithm::AES128 => 7,
            SymmetricKeyAlgorithm::AES192 => 8,
            SymmetricKeyAlgorithm::AES256 => 9,
            SymmetricKeyAlgorithm::Twofish256 => 10,
            SymmetricKeyAlgorithm::Camellia128 => 11,
            SymmetricKeyAlgorithm::Camellia192 => 12,
            SymmetricKeyAlgorithm::Camellia256 => 13,
        }
    }
}
impl TryFrom<u8> for SymmetricKeyAlgorithm {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        for alg in Self::iter_items() {
            if alg.get_id() == value {
                return Ok(alg);
            }
        }
        Err(ErrorKind::InvalidData.into())
    }
}
