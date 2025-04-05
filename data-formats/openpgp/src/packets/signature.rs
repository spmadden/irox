// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::packets::{read_mpi, PubkeyAlgorithm};
use crate::types::HashAlgorithm;
use core::fmt::{Debug, Formatter};
use irox_bits::{Bits, Error, ErrorKind};
use irox_enums::{EnumIterItem, EnumName};
use irox_time::datetime::UTCDateTime;
use irox_time::epoch::UnixTimestamp;
use irox_time::Duration;
use irox_tools::hex::to_hex_str_upper;

#[derive(Debug, Clone)]
pub enum SignaturePacket {
    Version4(SigV4Packet),
}
impl TryFrom<&[u8]> for SignaturePacket {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let vsn = value.read_u8()?;
        match vsn {
            4 => Ok(SignaturePacket::Version4(SigV4Packet::try_from(value)?)),
            _ => Err(ErrorKind::Unsupported.into()),
        }
    }
}
#[derive(Clone)]
pub struct SigV4Packet {
    pub subtype: SignatureSubtype,
    pub pubkey_algorithm: PubkeyAlgorithm,
    pub hash_algorithm: HashAlgorithm,
    pub hashed_data: Vec<u8>,
    pub hashed_packets: Vec<SignatureSubpacket>,
    pub unhashed_data: Vec<u8>,
    pub unhashed_packets: Vec<SignatureSubpacket>,
    pub upper_signed_hash: u16,
    pub signature_data: SignatureData,
}
impl TryFrom<&[u8]> for SigV4Packet {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let subtype: SignatureSubtype = value.read_u8()?.try_into()?;
        let pubkey_algorithm: PubkeyAlgorithm = value.read_u8()?.try_into()?;
        let hash_algorithm: HashAlgorithm = value.read_u8()?.try_into()?;
        let hash_len = value.read_be_u16()?;
        let hashed_data = value.read_exact_vec(hash_len as usize)?;
        let unhash_len = value.read_be_u16()?;
        let unhashed_data = value.read_exact_vec(unhash_len as usize)?;
        let upper_signed_hash = value.read_be_u16()?;
        let signature_data = SignatureData::try_from(pubkey_algorithm, value)?;

        let mut hashed_packets = Vec::new();
        let mut data = hashed_data.as_slice();
        while !data.is_empty() {
            hashed_packets.push(SignatureSubpacket::try_from(&mut data)?);
        }
        let unhashed_packets = Vec::new();
        Ok(SigV4Packet {
            subtype,
            pubkey_algorithm,
            hash_algorithm,
            hashed_data,
            hashed_packets,
            unhashed_data,
            unhashed_packets,
            upper_signed_hash,
            signature_data,
        })
    }
}
impl Debug for SigV4Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut st = f.debug_struct("SigV4Packet");
        st.field("type", &self.subtype)
            .field("pubalg", &self.pubkey_algorithm)
            .field("hashalg", &self.hash_algorithm);
        for p in &self.hashed_packets {
            st.field("hpkt", p);
        }
        for p in &self.unhashed_packets {
            st.field("upkt", p);
        }
        st.field("hash_val", &format!("{:02X}", self.upper_signed_hash))
            .field("sig", &self.signature_data)
            .finish()
    }
}
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumName, EnumIterItem)]
pub enum SignatureSubtype {
    Binary,
    Text,
    Standalone,
    GenericCertification,
    PersonaCertification,
    CasualCertification,
    PositiveCertification,
    SubkeyBinding,
    PrimaryKeyBinding,
    DirectKey,
    KeyRevocation,
    SubkeyRevocation,
    CertificationRevocation,
    Timestamp,
    ThirdPartyConfirmation,
}
impl SignatureSubtype {
    pub fn get_id(&self) -> u8 {
        match self {
            SignatureSubtype::Binary => 0,
            SignatureSubtype::Text => 1,
            SignatureSubtype::Standalone => 2,
            SignatureSubtype::GenericCertification => 0x10,
            SignatureSubtype::PersonaCertification => 0x11,
            SignatureSubtype::CasualCertification => 0x12,
            SignatureSubtype::PositiveCertification => 0x13,
            SignatureSubtype::SubkeyBinding => 0x18,
            SignatureSubtype::PrimaryKeyBinding => 0x19,
            SignatureSubtype::DirectKey => 0x1F,
            SignatureSubtype::KeyRevocation => 0x20,
            SignatureSubtype::SubkeyRevocation => 0x28,
            SignatureSubtype::CertificationRevocation => 0x30,
            SignatureSubtype::Timestamp => 0x40,
            SignatureSubtype::ThirdPartyConfirmation => 0x50,
        }
    }
}
impl TryFrom<u8> for SignatureSubtype {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        for ty in Self::iter_items() {
            if ty.get_id() == value {
                return Ok(ty);
            }
        }
        Err(ErrorKind::InvalidData.into())
    }
}
#[derive(Debug, Clone)]
pub enum SignatureSubpacket {
    SigCreationTime(CreationTime),
    SigExpirationTime(),       //TODO
    ExportableCertification(), //TODO
    TrustSignature(Trust),
    RegularExpression(), //TODO
    Revocable(),         //TODO
    KeyExpirationTime(KeyExpiration),
    PreferredV1SEIPDSymCiphers(),     //TODO
    IssuerKeyId(),                    //TODO
    NotationData(),                   //TODO
    PreferredHashAlgorithms(),        //TODO
    PreferredCompressionAlgorithms(), //TODO
    PreferredKeyServer(),             //TODO
    PrimaryUserId(),                  //TODO
    PolicyURI(),                      //TODO
    KeyFlags(),                       //TODO
    SignersUserId(),                  //TODO
    ReasonForRevocation(),            //TODO
    Features(),                       //TODO
    SignatureTarget(),                //TODO
    EmbeddedSignature(),              //TODO
    IssuerFingerprint(Issuer),
    IntendedRecipientFingerprint(), //TODO
    PreferredAEADCiphersuites(),    //TODO
    Unknown(UnknownSubpacket),
}
#[derive(Clone)]
pub struct UnknownSubpacket {
    pub pkt_type: u8,
    pub pkt_data: Vec<u8>,
}
impl Debug for UnknownSubpacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Unknown")
            .field("type", &self.pkt_type)
            .field("data", &to_hex_str_upper(&self.pkt_data))
            .finish()
    }
}
#[derive(Clone)]
pub struct Issuer {
    pub vsn: u8,
    pub issuer: Vec<u8>,
}
impl Debug for Issuer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IssuerSubpacket")
            .field("vsn", &self.vsn)
            .field("issuer", &to_hex_str_upper(&self.issuer))
            .finish()
    }
}
impl TryFrom<&[u8]> for Issuer {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let vsn = value.read_u8()?;
        let issuer = match vsn {
            4 => value.read_exact_vec(20)?,
            6 => value.read_exact_vec(32)?,
            _ => return Err(ErrorKind::InvalidData.into()),
        };
        Ok(Issuer { vsn, issuer })
    }
}
#[derive(Copy, Clone)]
pub struct CreationTime(pub UTCDateTime);
impl Debug for CreationTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CreationTime")
            .field(&self.0.format_iso8601_basic())
            .finish()
    }
}
impl TryFrom<&[u8]> for CreationTime {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let time = value.read_be_u32()?;
        let time: UTCDateTime = UnixTimestamp::from_seconds(time).into();
        Ok(CreationTime(time))
    }
}
#[derive(Copy, Clone)]
pub struct KeyExpiration(pub Duration);
impl Debug for KeyExpiration {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KeyExpiration")
            .field(&self.0.to_string())
            .finish()
    }
}
impl TryFrom<&[u8]> for KeyExpiration {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let time = value.read_be_u32()?;
        Ok(KeyExpiration(Duration::from_seconds(time as u64)))
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Trust {
    pub depth: u8,
    pub trust_amount: u8,
}
impl TryFrom<&[u8]> for Trust {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let depth = value.read_u8()?;
        let trust_amount = value.read_u8()?;
        Ok(Trust {
            depth,
            trust_amount,
        })
    }
}
impl SignatureSubpacket {
    fn try_from<T: Bits>(value: &mut T) -> Result<Self, Error> {
        let len = read_subpktlen(value)? - 1;
        let pkt_type = value.read_u8()?;
        let pkt_data = value.read_exact_vec(len as usize)?;

        match pkt_type {
            2 => {
                return Ok(SignatureSubpacket::SigCreationTime(CreationTime::try_from(
                    pkt_data.as_slice(),
                )?));
            }
            5 => {
                return Ok(SignatureSubpacket::TrustSignature(Trust::try_from(
                    pkt_data.as_slice(),
                )?));
            }
            9 => {
                return Ok(SignatureSubpacket::KeyExpirationTime(
                    KeyExpiration::try_from(pkt_data.as_slice())?,
                ))
            }
            33 => {
                return Ok(SignatureSubpacket::IssuerFingerprint(Issuer::try_from(
                    pkt_data.as_slice(),
                )?));
            }
            _ => {}
        }

        Ok(SignatureSubpacket::Unknown(UnknownSubpacket {
            pkt_data,
            pkt_type,
        }))
    }
}
#[derive(Debug, Clone)]
pub enum SignatureData {
    RSA(),   //TODO
    ECSDA(), //TODO
    EdDSALegacy(EdDSALegacySignature),
    Ed25519Legacy(), //TODO
    Ed25519(),       //TODO
    Ed448(),         //TODO
}
impl SignatureData {
    fn try_from(alg: PubkeyAlgorithm, value: &[u8]) -> Result<Self, Error> {
        match alg {
            PubkeyAlgorithm::EdDSALegacy => Ok(SignatureData::EdDSALegacy(
                EdDSALegacySignature::try_from(value)?,
            )),
            _ => Err(ErrorKind::Unsupported.into()),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct EdDSALegacySignature {
    pub r: Vec<u8>,
    pub s: Vec<u8>,
}
impl Debug for EdDSALegacySignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EdDSALegacySignature")
            .field("r", &to_hex_str_upper(self.r.as_slice()))
            .field("s", &to_hex_str_upper(self.s.as_slice()))
            .finish()
    }
}
impl TryFrom<&[u8]> for EdDSALegacySignature {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let r = read_mpi(&mut value)?;
        let s = read_mpi(&mut value)?;
        Ok(EdDSALegacySignature { r, s })
    }
}

pub fn read_subpktlen<T: Bits>(source: &mut T) -> Result<u32, Error> {
    let o1 = source.read_u8()?;
    if o1 < 192 {
        Ok(o1 as u32)
    } else if o1 < 255 {
        let o2 = source.read_u8()?;
        let o1 = ((o1 as u32) - 192) << 8;
        Ok(o1 + o2 as u32 + 192)
    } else {
        source.read_be_u32()
    }
}
