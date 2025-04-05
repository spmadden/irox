// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::types::{HashAlgorithm, SymmetricKeyAlgorithm};
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
    pub fingerprint: [u8; 20],
}
impl Debug for PubKeyV4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PubKeyV4")
            .field("timestamp", &self.timestamp.format_iso8601_basic())
            .field("algorithm", &self.algorithm)
            .field("data", &self.data)
            .field("fingerprint", &to_hex_str_upper(&self.fingerprint))
            .finish()
    }
}
impl TryFrom<&[u8]> for PubKeyV4 {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let fingerprint = {
            let mut fp = SHA1::new();
            fp.write_u8(0x99)?;
            fp.write_be_u16((value.len() + 1) as u16)?;
            fp.write_u8(0x04)?;
            fp.write_all_bytes(value)?;
            fp.finish()
        };
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
        Ok(PubKeyV4 {
            timestamp,
            algorithm,
            data,
            fingerprint,
        })
    }
}

#[derive(Clone)]
pub struct EdDSALegacy {
    pub oid: Vec<u8>,
    pub pubkey: Vec<u8>,
}
impl Debug for EdDSALegacy {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EdDSALegacy")
            .field("OID", &to_hex_str_upper(self.oid.as_slice()))
            .field("PK", &to_hex_str_upper(self.pubkey.as_slice()))
            .finish()
    }
}
impl TryFrom<&[u8]> for EdDSALegacy {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let olen = value.read_u8()?;
        let oid = value.read_exact_vec(olen as usize)?;
        let pubkey = read_mpi(&mut value)?;
        Ok(EdDSALegacy { oid, pubkey })
    }
}
#[derive(Clone)]
pub struct ECDH {
    pub oid: Vec<u8>,
    pub pubkey: Vec<u8>,
    pub hash_function: HashAlgorithm,
    pub sym_algorithm: SymmetricKeyAlgorithm,
}
impl Debug for ECDH {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ECDH")
            .field("OID", &to_hex_str_upper(self.oid.as_slice()))
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
        let pubkey = read_mpi(&mut value)?;
        let _skip = value.read_be_u16()?;
        let hash_function = value.read_u8()?.try_into()?;
        let sym_algorithm = value.read_u8()?.try_into()?;
        Ok(ECDH {
            oid,
            pubkey,
            hash_function,
            sym_algorithm,
        })
    }
}
pub fn read_mpi<T: Bits>(i: &mut T) -> Result<Vec<u8>, Error> {
    let bits = i.read_be_u16()?;
    let len = (bits + 7) / 8;
    i.read_exact_vec(len as usize)
}
