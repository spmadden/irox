// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::keygrip::KeyGrip;
use crate::types::{ECC_Curve, HashAlgorithm, SymmetricKeyAlgorithm};
use core::fmt::{Debug, Formatter};
use irox_bits::{Bits, Error, ErrorKind, MutBits};
use irox_enums::EnumIterItem;
use irox_time::datetime::UTCDateTime;
use irox_time::epoch::UnixTimestamp;
use irox_tools::hash::SHA1;
use irox_tools::hex::to_hex_str_upper;

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIterItem)]
pub enum PubkeyAlgorithm {
    RSA,
    RSAEncryptOnly,
    RSASignOnly,
    Elgamal,
    DSA,
    ECDH,
    DCDSA,
    EdDSALegacy,
    X25519,
    X448,
    Ed25519,
    Ed448,
}
impl PubkeyAlgorithm {
    pub fn get_id(&self) -> u8 {
        match self {
            PubkeyAlgorithm::RSA => 1,
            PubkeyAlgorithm::RSAEncryptOnly => 2,
            PubkeyAlgorithm::RSASignOnly => 3,
            PubkeyAlgorithm::Elgamal => 16,
            PubkeyAlgorithm::DSA => 17,
            PubkeyAlgorithm::ECDH => 18,
            PubkeyAlgorithm::DCDSA => 19,
            PubkeyAlgorithm::EdDSALegacy => 22,
            PubkeyAlgorithm::X25519 => 25,
            PubkeyAlgorithm::X448 => 26,
            PubkeyAlgorithm::Ed25519 => 27,
            PubkeyAlgorithm::Ed448 => 28,
        }
    }
}

impl TryFrom<u8> for PubkeyAlgorithm {
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
#[derive(Debug, Clone)]
pub enum PubKeyData {
    RSA(),     //TODO
    DSA(),     //TODO,
    ElGamal(), //TODO
    ECDSA(),   //TODO
    EdDSALegacy(EdDSALegacy),
    ECDH(ECDH),
    X25519(),  //TODO
    X448(),    //TODO
    Ed25519(), //TODO
    Ed448(),   //TODO
}
impl KeyGrip for PubKeyData {
    fn generate_keygrip(&self) -> Option<[u8; 20]> {
        match self {
            PubKeyData::EdDSALegacy(e) => Some(e.curve.keygrip(e.pubkey.as_slice())),
            PubKeyData::ECDH(e) => Some(e.curve.keygrip(e.pubkey.as_slice())),
            _ => None,
        }
    }
}
#[derive(Debug, Clone)]
pub enum PubKeyPacket {
    Version4(PubKeyV4),
}
impl TryFrom<&[u8]> for PubKeyPacket {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let vsn = value.read_u8()?;
        match vsn {
            4 => Ok(Self::Version4(PubKeyV4::try_from(value)?)),
            _ => Err(ErrorKind::Unsupported.into()),
        }
    }
}
#[derive(Clone)]
pub struct PubKeyV4 {
    pub timestamp: UTCDateTime,
    pub algorithm: PubkeyAlgorithm,
    pub data: PubKeyData,
    pub fingerprint_data: Vec<u8>,
    pub fingerprint: [u8; 20],
    pub keygrip: Option<[u8; 20]>,
}
impl Debug for PubKeyV4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PubKeyV4")
            .field("timestamp", &self.timestamp.format_iso8601_basic())
            .field("algorithm", &self.algorithm)
            .field("data", &self.data)
            .field("fingerprint", &to_hex_str_upper(&self.fingerprint))
            .field(
                "keygrip",
                &self.keygrip.as_ref().map(|v| to_hex_str_upper(v)),
            )
            .finish()
    }
}
impl TryFrom<&[u8]> for PubKeyV4 {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let fingerprint_data = {
            let mut fingerprint_data = Vec::new();
            fingerprint_data.write_u8(0x99)?;
            fingerprint_data.write_be_u16((value.len() + 1) as u16)?;
            fingerprint_data.write_u8(0x04)?;
            fingerprint_data.write_all_bytes(value)?;
            fingerprint_data
        };
        let fingerprint = SHA1::new().hash(&fingerprint_data);

        let ts = value.read_be_u32()?;
        let timestamp: UTCDateTime = UnixTimestamp::from_seconds(ts).into();
        let algorithm: PubkeyAlgorithm = value.read_u8()?.try_into()?;
        let data = match algorithm {
            PubkeyAlgorithm::EdDSALegacy => PubKeyData::EdDSALegacy(EdDSALegacy::try_from(value)?),
            PubkeyAlgorithm::ECDH => PubKeyData::ECDH(ECDH::try_from(value)?),
            _ => {
                return Err(ErrorKind::Unsupported.into());
            }
        };
        let keygrip = data.generate_keygrip();
        Ok(PubKeyV4 {
            timestamp,
            algorithm,
            data,
            fingerprint_data,
            fingerprint,
            keygrip,
        })
    }
}

#[derive(Clone)]
pub struct EdDSALegacy {
    pub curve: ECC_Curve,
    pub pubkey: Vec<u8>,
}
impl Debug for EdDSALegacy {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EdDSALegacy")
            .field("curve", &self.curve)
            .field("PK", &to_hex_str_upper(self.pubkey.as_slice()))
            .finish()
    }
}
impl TryFrom<&[u8]> for EdDSALegacy {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let olen = value.read_u8()?;
        let oid = value.read_exact_vec(olen as usize)?;
        let curve: ECC_Curve = oid.as_slice().try_into()?;
        let pubkey = read_mpi(&mut value, true)?;
        Ok(EdDSALegacy { curve, pubkey })
    }
}
#[derive(Clone)]
pub struct ECDH {
    pub curve: ECC_Curve,
    pub pubkey: Vec<u8>,
    pub hash_function: HashAlgorithm,
    pub sym_algorithm: SymmetricKeyAlgorithm,
}
impl Debug for ECDH {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ECDH")
            .field("curve", &self.curve)
            .field("PK", &to_hex_str_upper(self.pubkey.as_slice()))
            .field("hash", &self.hash_function)
            .field("sym", &self.sym_algorithm)
            .finish()
    }
}
impl TryFrom<&[u8]> for ECDH {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let olen = value.read_u8()?;
        let oid = value.read_exact_vec(olen as usize)?;
        let curve: ECC_Curve = oid.as_slice().try_into()?;
        let pubkey = read_mpi(&mut value, true)?;
        let _skip = value.read_be_u16()?;
        let hash_function = value.read_u8()?.try_into()?;
        let sym_algorithm = value.read_u8()?.try_into()?;
        Ok(ECDH {
            curve,
            pubkey,
            hash_function,
            sym_algorithm,
        })
    }
}
pub fn read_mpi<T: Bits>(i: &mut T, is_curve: bool) -> Result<Vec<u8>, Error> {
    let bits = i.read_be_u16()?;
    let mut len = (bits + 7) / 8;
    if is_curve {
        i.read_u8()?; // throw away curve prefix
        len -= 1;
    }
    i.read_exact_vec(len as usize)
}
