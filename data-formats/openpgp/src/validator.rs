// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::packets::SignaturePacket;
use crate::types::HashAlgorithm;
use irox_bits::{BitsErrorKind, Error, ErrorKind};
use irox_cryptids::ed25519::{Ed25519PublicKey, Ed25519Signature};
use irox_tools::hash::{MD5, SHA1, SHA224, SHA256, SHA384, SHA512};

pub struct SignatureValidator {
    hasher: Hasher,
    signature_packet: SignaturePacket,
    pubkey: Ed25519PublicKey,
}
impl SignatureValidator {
    pub fn new(pubkey: Ed25519PublicKey, signature_packet: SignaturePacket) -> Result<Self, Error> {
        let hasher: Hasher = signature_packet.get_hash_alg().try_into()?;

        Ok(SignatureValidator {
            hasher,
            signature_packet,
            pubkey,
        })
    }
    pub fn write(&mut self, data: &[u8]) {
        self.hasher.write(data);
    }
    pub fn finish_check(mut self) -> Result<(), Error> {
        let data = self.signature_packet.get_hashed_data();
        self.hasher.write(data);
        self.hasher
            .write(&[self.signature_packet.get_version(), 0xFF]);
        self.hasher.write(&data.len().to_be_bytes());
        let hash = self.hasher.finish();

        let sig = Ed25519Signature {
            signature: self.signature_packet.try_into_ed25519_sig()?,
            pubkey: self.pubkey,
        };
        sig.validate_hash(&hash)
            .map_err(|_| Error::new(BitsErrorKind::InvalidInput, "Signature validation failure"))
    }
}

pub enum Hasher {
    MD5(MD5),
    SHA1(SHA1),
    SHA224(SHA224),
    SHA256(SHA256),
    SHA384(SHA384),
    SHA512(SHA512),
}
impl TryFrom<HashAlgorithm> for Hasher {
    type Error = Error;

    fn try_from(value: HashAlgorithm) -> Result<Self, Self::Error> {
        match value {
            HashAlgorithm::MD5 => Ok(Hasher::MD5(MD5::default())),
            HashAlgorithm::SHA1 => Ok(Hasher::SHA1(SHA1::default())),
            HashAlgorithm::SHA256 => Ok(Hasher::SHA256(SHA256::default())),
            HashAlgorithm::SHA384 => Ok(Hasher::SHA384(SHA384::default())),
            HashAlgorithm::SHA512 => Ok(Hasher::SHA512(SHA512::default())),
            HashAlgorithm::SHA224 => Ok(Hasher::SHA224(SHA224::default())),
            _ => Err(ErrorKind::Unsupported.into()),
        }
    }
}

impl Hasher {
    pub fn write(&mut self, val: &[u8]) {
        match self {
            Hasher::MD5(h) => h.write(val),
            Hasher::SHA1(h) => h.write(val),
            Hasher::SHA224(h) => h.write(val),
            Hasher::SHA256(h) => h.write(val),
            Hasher::SHA384(h) => h.write(val),
            Hasher::SHA512(h) => h.write(val),
        }
    }
    pub fn finish(self) -> Box<[u8]> {
        match self {
            Hasher::MD5(v) => Box::from(v.finish().to_be_bytes()),
            Hasher::SHA1(v) => Box::from(v.finish()),
            Hasher::SHA224(v) => Box::from(v.finish()),
            Hasher::SHA256(v) => Box::from(v.finish()),
            Hasher::SHA384(v) => Box::from(v.finish()),
            Hasher::SHA512(v) => Box::from(v.finish()),
        }
    }
}
