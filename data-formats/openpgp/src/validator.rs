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

    pub fn s2k(
        alg: HashAlgorithm,
        iter: usize,
        salt: &[u8],
        data: &[u8],
    ) -> Result<Box<[u8]>, Error> {
        let mut h: Self = alg.try_into()?;
        let mut rem = iter;
        while rem > 0 {
            let l = rem.min(salt.len());
            let Some(s) = salt.get(0..l) else {
                break;
            };
            h.write(s);
            rem -= l;
            let l = rem.min(data.len());
            let Some(d) = data.get(0..l) else {
                break;
            };
            h.write(d);
            rem -= l;
        }
        Ok(h.finish())
    }
}
#[cfg(test)]
mod tests {
    use crate::types::HashAlgorithm;
    use crate::validator::Hasher;
    use irox_bits::Error;
    use irox_tools::hash::SHA256;
    use irox_tools::{assert_eq_hex_slice, hex};

    #[test]
    pub fn test_s2k_1() {
        let s = hex!("3031323334353637313233343536");
        let c = 0x000186A0;
        let mut rem = c;
        let mut h = SHA256::new();
        while rem > 0 {
            let l = rem.min(s.len());
            h.write(&s[0..l]);
            rem -= l;
        }
        let h = h.finish();
        assert_eq_hex_slice!(
            h,
            hex!("773784A602B6C81E3F092F4D7D00E17CC822D88F7360FCF2D2EF2D9D901F44B6")
        );
    }

    #[test]
    pub fn test_s2k_2() -> Result<(), Error> {
        let s = b"01234567";
        let d = b"123456";
        let c = 0x000186A0;
        let h = Hasher::s2k(HashAlgorithm::SHA256, c, s, d)?;
        assert_eq_hex_slice!(
            h,
            hex!("773784A602B6C81E3F092F4D7D00E17CC822D88F7360FCF2D2EF2D9D901F44B6")
        );
        Ok(())
    }
    #[test]
    pub fn test_s2k_3() -> Result<(), Error> {
        let s = hex!("4142434445464748");
        let d = b"12345678";
        let c = 0x000186A0;
        let h = Hasher::s2k(HashAlgorithm::SHA256, c, &s, d)?;
        assert_eq_hex_slice!(
            h,
            hex!("2675D6164A0D4827D1D00C7EEA620D015C00030A1CAB38B4D0DD600B27DC9630")
        );
        Ok(())
    }
}
