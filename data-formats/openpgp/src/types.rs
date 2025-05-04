// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::keygrip::gen_keygrip;
use core::fmt::{Debug, Formatter};
use irox_bits::{Bits, Error, ErrorKind};
use irox_cryptids::ed25519::{ED25519_BASE, ED25519_G};
use irox_cryptids::x25519::{ED25519_ORDER, X25519_G, X255M19};
use irox_enums::{EnumIterItem, EnumName};
use irox_tools::arrays::SliceTools;
use irox_tools::hash::HasherCounting;
use irox_tools::hex;
use irox_tools::hex::to_hex_str_upper;

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
impl TryFrom<HashAlgorithm> for HasherCounting {
    type Error = Error;

    fn try_from(value: HashAlgorithm) -> Result<Self, Self::Error> {
        match value {
            HashAlgorithm::MD5 => irox_tools::hash::HashAlgorithm::MD5.try_into(),
            HashAlgorithm::SHA1 => irox_tools::hash::HashAlgorithm::SHA1.try_into(),
            HashAlgorithm::SHA256 => irox_tools::hash::HashAlgorithm::SHA256.try_into(),
            HashAlgorithm::SHA384 => irox_tools::hash::HashAlgorithm::SHA384.try_into(),
            HashAlgorithm::SHA512 => irox_tools::hash::HashAlgorithm::SHA512.try_into(),
            HashAlgorithm::SHA224 => irox_tools::hash::HashAlgorithm::SHA224.try_into(),
            _ => Err(ErrorKind::InvalidInput.into()),
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
pub struct Hash {
    pub hash: Box<[u8]>,
    pub algorithm: HashAlgorithm,
}
impl Debug for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Hash({:?}:{})",
            self.algorithm,
            to_hex_str_upper(&self.hash)
        )
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

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIterItem, EnumName)]
pub enum CompressionAlgorithm {
    Uncompressed,
    ZIP,
    ZLIB,
    BZip2,
}
impl CompressionAlgorithm {
    pub fn get_id(&self) -> u8 {
        match self {
            CompressionAlgorithm::Uncompressed => 0,
            CompressionAlgorithm::ZIP => 1,
            CompressionAlgorithm::ZLIB => 2,
            CompressionAlgorithm::BZip2 => 3,
        }
    }
}
impl TryFrom<u8> for CompressionAlgorithm {
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
pub enum KeyFlag {
    Certify,
    Sign,
    EncryptCommunications,
    EncryptStorage,
    SplitKey,
    Authentication,
    Shared,
}
impl KeyFlag {
    pub fn get_id(&self) -> u8 {
        match self {
            KeyFlag::Certify => 0x01,
            KeyFlag::Sign => 0x02,
            KeyFlag::EncryptCommunications => 0x04,
            KeyFlag::EncryptStorage => 0x08,
            KeyFlag::SplitKey => 0x10,
            KeyFlag::Authentication => 0x20,
            KeyFlag::Shared => 0x80,
        }
    }

    pub fn try_from(mut value: &[u8]) -> Result<Vec<KeyFlag>, Error> {
        let mut out = Vec::new();
        if let Some(v) = value.next_u8()? {
            for f in Self::iter_items() {
                let i = f.get_id();
                if v & i == i {
                    out.push(f);
                }
            }
        }
        Ok(out)
    }
}

#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, EnumIterItem, EnumName)]
pub enum ECC_Curve {
    NIST_P256,
    NIST_P384,
    NIST_P521,
    BrainpoolP256r1,
    BrainpoolP384r1,
    BrainpoolP512r1,
    Ed25519Legacy,
    Curve25519Legacy,
}
pub struct CurveParameters {
    pub oid: &'static [u8],
    pub asn_oid: &'static str,
    pub keysize_bits: u16,
    pub p: &'static [u8],
    pub a: &'static [u8],
    pub b: &'static [u8],
    pub g: &'static [u8],
    pub n: &'static [u8],
}
impl CurveParameters {
    pub const fn get_oid(&self) -> &'static [u8] {
        self.oid
    }
    pub fn keygrip(&self, pk: &[u8]) -> [u8; 20] {
        gen_keygrip(&[
            ("p", &self.p.reversed()),
            ("a", self.a),
            ("b", &self.b.reversed()),
            ("g", &self.g.reversed()),
            ("n", &self.n.reversed()),
            ("q", pk),
        ])
    }
}
pub static NIST_P256_PARAMS: &CurveParameters = &CurveParameters {
    oid: &hex!("2A8648CE3D030107"),
    asn_oid: "1.2.840.10045.3.1.7",
    keysize_bits: 256,
    p: &[],
    a: &[],
    b: &[],
    g: &[],
    n: &[],
};
pub static NIST_P384_PARAMS: &CurveParameters = &CurveParameters {
    oid: &hex!("2B81040022"),
    asn_oid: "1.3.132.0.34",
    keysize_bits: 384,
    p: &[],
    a: &[],
    b: &[],
    g: &[],
    n: &[],
};
pub static NIST_P521_PARAMS: &CurveParameters = &CurveParameters {
    oid: &hex!("2B81040023"),
    asn_oid: "1.3.132.0.35",
    keysize_bits: 521,
    p: &[],
    a: &[],
    b: &[],
    g: &[],
    n: &[],
};
pub static BRAINPOOL_P256_PARAMS: &CurveParameters = &CurveParameters {
    oid: &hex!("2B2403030208010107"),
    asn_oid: "1.3.36.3.3.2.8.1.1.7",
    keysize_bits: 256,
    p: &[],
    a: &[],
    b: &[],
    g: &[],
    n: &[],
};
pub static BRAINPOOL_P384_PARAMS: &CurveParameters = &CurveParameters {
    oid: &hex!("2B240303020801010B"),
    asn_oid: "1.3.36.3.3.2.8.1.1.11",
    keysize_bits: 384,
    p: &[],
    a: &[],
    b: &[],
    g: &[],
    n: &[],
};
pub static BRAINPOOL_P512_PARAMS: &CurveParameters = &CurveParameters {
    oid: &hex!("2B240303020801010D"),
    asn_oid: "1.3.36.3.3.2.8.1.1.13",
    keysize_bits: 512,
    p: &[],
    a: &[],
    b: &[],
    g: &[],
    n: &[],
};
pub static ED25519_PARAMS: &CurveParameters = &CurveParameters {
    oid: &hex!("2B06010401DA470F01"),
    asn_oid: "1.3.6.1.4.1.11591.15.1",
    keysize_bits: 255,
    p: X255M19,
    a: &hex!("01"),
    b: ED25519_BASE,
    g: ED25519_G,
    n: ED25519_ORDER,
};
pub static X25519_PARAMS: &CurveParameters = &CurveParameters {
    oid: &hex!("2B060104019755010501"),
    asn_oid: "1.3.6.1.4.1.3029.1.5.1",
    keysize_bits: 255,
    p: X255M19,
    a: &hex!("01DB41"),
    b: &hex!("01"),
    g: X25519_G,
    n: ED25519_ORDER,
};
impl ECC_Curve {
    pub fn get_params(&self) -> &CurveParameters {
        match self {
            ECC_Curve::NIST_P256 => NIST_P256_PARAMS,
            ECC_Curve::NIST_P384 => NIST_P384_PARAMS,
            ECC_Curve::NIST_P521 => NIST_P521_PARAMS,
            ECC_Curve::BrainpoolP256r1 => BRAINPOOL_P256_PARAMS,
            ECC_Curve::BrainpoolP384r1 => BRAINPOOL_P384_PARAMS,
            ECC_Curve::BrainpoolP512r1 => BRAINPOOL_P512_PARAMS,
            ECC_Curve::Ed25519Legacy => ED25519_PARAMS,
            ECC_Curve::Curve25519Legacy => X25519_PARAMS,
        }
    }
    pub fn get_oid(&self) -> &[u8] {
        self.get_params().oid
    }
    pub fn get_asn_oid(&self) -> &str {
        self.get_params().asn_oid
    }
    pub fn keygrip(&self, pk: &[u8]) -> [u8; 20] {
        self.get_params().keygrip(pk)
    }
}
impl Debug for ECC_Curve {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!("ECC_Curve({})", self.name()))
            .field("OID", &to_hex_str_upper(self.get_oid()))
            .field("ASN OID", &self.get_asn_oid())
            .finish()
    }
}
impl TryFrom<&[u8]> for ECC_Curve {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        for curve in Self::iter_items() {
            if value == curve.get_oid() {
                return Ok(curve);
            }
        }
        Err(ErrorKind::InvalidInput.into())
    }
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIterItem, EnumName)]
pub enum Features {
    Version1SymEncIPD,
    Version2SymEncIPD,
}
impl Features {
    pub fn get_id(&self) -> u8 {
        match self {
            Features::Version1SymEncIPD => 0x01,
            Features::Version2SymEncIPD => 0x08,
        }
    }
    pub fn try_from(mut value: &[u8]) -> Result<Vec<Self>, Error> {
        let mut out = Vec::new();
        let flags = match value.len() {
            1 => value.next_u8()?.unwrap_or_default() as u32,
            _ => return Err(ErrorKind::InvalidData.into()),
        };
        for f in Self::iter_items() {
            if flags & f.get_id() as u32 != 0 {
                out.push(f);
            }
        }
        Ok(out)
    }
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIterItem, EnumName)]
pub enum KeyServerPreference {
    NoModify,
}
impl KeyServerPreference {
    pub fn get_id(&self) -> u8 {
        match self {
            KeyServerPreference::NoModify => 0x80,
        }
    }
    pub fn try_from(mut value: &[u8]) -> Result<Vec<Self>, Error> {
        let mut out = Vec::new();
        let flags = match value.len() {
            1 => value.next_u8()?.unwrap_or_default() as u32,
            _ => return Err(ErrorKind::InvalidData.into()),
        };
        for f in Self::iter_items() {
            if flags & f.get_id() as u32 != 0 {
                out.push(f);
            }
        }
        Ok(out)
    }
}
