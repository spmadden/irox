// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::keybox::{Fingerprint, Keybox, Keygrip, PublicKey, PublicKeyData, PublicKeySource};
use crate::keygrip::KeyGrip;
use crate::types::{ECC_Curve, HashAlgorithm, SymmetricKeyAlgorithm};
use core::fmt::{Debug, Formatter};
use irox_bits::{Bits, Error, ErrorKind, MutBits, SerializeToBits};
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
impl SerializeToBits for PubKeyData {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        match self {
            PubKeyData::EdDSALegacy(e) => e.serialize_to_bits(bits),
            PubKeyData::ECDH(ec) => ec.serialize_to_bits(bits),
            _ => todo!(),
        }
    }
}
impl TryFrom<&PubKeyData> for PublicKeyData {
    type Error = Error;

    fn try_from(value: &PubKeyData) -> Result<Self, Self::Error> {
        match value {
            PubKeyData::EdDSALegacy(e) => match e.curve {
                ECC_Curve::Ed25519Legacy => Ok(PublicKeyData::Ed25519(
                    e.pubkey
                        .as_slice()
                        .try_into()
                        .map_err(|_| ErrorKind::InvalidData)?,
                )),
                ECC_Curve::Curve25519Legacy => Ok(PublicKeyData::X25519(
                    e.pubkey
                        .as_slice()
                        .try_into()
                        .map_err(|_| ErrorKind::InvalidData)?,
                )),
                _ => Err(ErrorKind::Unsupported.into()),
            },
            PubKeyData::ECDH(e) => match e.curve {
                ECC_Curve::Ed25519Legacy => Ok(PublicKeyData::Ed25519(
                    e.pubkey
                        .as_slice()
                        .try_into()
                        .map_err(|_| ErrorKind::InvalidData)?,
                )),
                ECC_Curve::Curve25519Legacy => Ok(PublicKeyData::X25519(
                    e.pubkey
                        .as_slice()
                        .try_into()
                        .map_err(|_| ErrorKind::InvalidData)?,
                )),
                _ => Err(ErrorKind::Unsupported.into()),
            },
            _ => Err(ErrorKind::Unsupported.into()),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
impl SerializeToBits for PubKeyPacket {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        match self {
            PubKeyPacket::Version4(v4) => {
                bits.write_u8(4)?;
                let len = v4.serialize_to_bits(bits)?;
                Ok(len + 1)
            }
        }
    }
}
impl TryFrom<&PubKeyPacket> for PublicKey {
    type Error = Error;

    fn try_from(value: &PubKeyPacket) -> Result<Self, Self::Error> {
        match value {
            PubKeyPacket::Version4(v4) => v4.try_into(),
        }
    }
}

impl PubKeyPacket {
    pub fn add_to_keybox(&self, bx: &mut Keybox) -> Result<Fingerprint, Error> {
        match self {
            PubKeyPacket::Version4(v4) => v4.add_to_keybox(bx),
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
impl PubKeyV4 {
    pub fn add_to_keybox(&self, bx: &mut Keybox) -> Result<Fingerprint, Error> {
        let fp = Fingerprint(self.fingerprint.to_vec().into_boxed_slice());
        bx.pubkeys.insert(fp.clone(), self.try_into()?);
        Ok(fp)
    }
}
impl TryFrom<&PubKeyV4> for PublicKey {
    type Error = Error;

    fn try_from(value: &PubKeyV4) -> Result<Self, Self::Error> {
        let fp = Fingerprint(value.fingerprint.to_vec().into_boxed_slice());

        Ok(PublicKey {
            data: (&value.data).try_into()?,
            user_id: None,
            created_on: Some(value.timestamp),
            valid_until: None,
            fingerprint: fp,
            keygrip: value
                .keygrip
                .map(|v| Keygrip(v.as_slice().to_vec().into_boxed_slice())),
            source: PublicKeySource::OpenPGP(),
            issuer: None,
            subkeys: vec![],
        })
    }
}
impl Eq for PubKeyV4 {}
impl PartialEq for PubKeyV4 {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
            && self.algorithm == other.algorithm
            && self.data == other.data
            && self.fingerprint == other.fingerprint
            && self.keygrip == other.keygrip
    }
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
impl SerializeToBits for PubKeyV4 {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let ts: UnixTimestamp = self.timestamp.into();
        let mut len = 0;
        bits.write_be_u32(ts.get_offset().as_seconds() as u32)?;
        len += 4;

        bits.write_u8(self.algorithm.get_id())?;
        len += 1;

        len += self.data.serialize_to_bits(bits)?;

        Ok(len)
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

#[derive(Clone, Eq, PartialEq)]
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
impl SerializeToBits for EdDSALegacy {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let oid = self.curve.get_oid();
        bits.write_u8(oid.len() as u8)?;
        bits.write_all_bytes(oid)?;
        let len = write_mpi(bits, true, self.pubkey.as_slice())?;
        Ok(len + oid.len() + 1)
    }
}
#[derive(Clone, Eq, PartialEq)]
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
impl SerializeToBits for ECDH {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let oid = self.curve.get_oid();
        let len = write_mpi(bits, true, oid)?;
        bits.write_be_u16(0)?;
        bits.write_u8(self.hash_function.get_id())?;
        bits.write_u8(self.sym_algorithm.get_id())?;
        Ok(len + 4)
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

pub fn write_mpi<T: MutBits + ?Sized>(
    o: &mut T,
    is_curve: bool,
    value: &[u8],
) -> Result<usize, Error> {
    let mut len = (value.len() * 8) as u16;
    if is_curve {
        len += 8;
    }
    o.write_be_u16(len)?;
    if is_curve {
        // write curve prefix
        o.write_u8(value.len() as u8)?;
    }
    o.write_all_bytes(value)?;
    Ok(value.len() + 2)
}
