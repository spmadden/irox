// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::packets::{read_mpi, PubkeyAlgorithm};
use crate::types::{CompressionAlgorithm, HashAlgorithm, KeyFlag};
use core::fmt::{Debug, Formatter};
use irox_bits::{Bits, Error, ErrorKind, MutBits};
use irox_enums::{EnumIterItem, EnumName};
use irox_time::datetime::UTCDateTime;
use irox_time::epoch::UnixTimestamp;
use irox_time::format::iso8601::ISO8601Format;
use irox_time::Duration;
use irox_tools::hex::to_hex_str_upper;

#[derive(Debug, Clone)]
pub enum SignaturePacket {
    Version4(SigV4Packet),
}
impl SignaturePacket {
    pub fn get_version(&self) -> u8 {
        match self {
            SignaturePacket::Version4(_) => 0x4,
        }
    }
    pub fn get_subtype(&self) -> SignatureSubtype {
        match self {
            SignaturePacket::Version4(v) => v.subtype,
        }
    }
    pub fn get_pubkey_alg(&self) -> PubkeyAlgorithm {
        match self {
            SignaturePacket::Version4(v) => v.pubkey_algorithm,
        }
    }
    pub fn get_hash_alg(&self) -> HashAlgorithm {
        match self {
            SignaturePacket::Version4(v) => v.hash_algorithm,
        }
    }
    pub fn get_hashed_data(&self) -> &[u8] {
        match self {
            SignaturePacket::Version4(v) => v.hashed_data.as_slice(),
        }
    }
    pub fn try_into_ed25519_sig(&self) -> Result<[u8; 64], Error> {
        match self {
            SignaturePacket::Version4(v) => v.try_into_ed25519_sig(),
        }
    }
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
impl SigV4Packet {
    #[allow(clippy::match_same_arms)]
    pub fn try_into_ed25519_sig(&self) -> Result<[u8; 64], Error> {
        match self.pubkey_algorithm {
            PubkeyAlgorithm::EdDSALegacy => {
                let SignatureData::EdDSALegacy(e) = &self.signature_data else {
                    return Err(ErrorKind::InvalidInput.into());
                };
                let mut sig = [0u8; 64];
                let mut s = sig.as_mut_slice();
                s.write_all_bytes(e.r.as_slice())?;
                s.write_all_bytes(e.s.as_slice())?;
                Ok(sig)
            }
            PubkeyAlgorithm::X25519 => {
                //TODO
                Err(ErrorKind::Unsupported.into())
            }
            PubkeyAlgorithm::X448 => {
                //TODO
                Err(ErrorKind::Unsupported.into())
            }
            PubkeyAlgorithm::Ed25519 => {
                //TODO
                Err(ErrorKind::Unsupported.into())
            }
            PubkeyAlgorithm::Ed448 => {
                //TODO
                Err(ErrorKind::Unsupported.into())
            }
            _ => Err(ErrorKind::Unsupported.into()),
        }
    }
}
impl TryFrom<&[u8]> for SigV4Packet {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let subtype: SignatureSubtype = value.read_u8()?.try_into()?;
        let pubkey_algorithm: PubkeyAlgorithm = value.read_u8()?.try_into()?;
        let hash_algorithm: HashAlgorithm = value.read_u8()?.try_into()?;
        let hash_len = value.read_be_u16()?;
        let hashed_data_pkts = value.read_exact_vec(hash_len as usize)?;
        let unhash_len = value.read_be_u16()?;
        let unhashed_data = value.read_exact_vec(unhash_len as usize)?;
        let upper_signed_hash = value.read_be_u16()?;
        let signature_data = SignatureData::try_from(pubkey_algorithm, value)?;

        let mut hashed_packets = Vec::new();
        let mut data = hashed_data_pkts.as_slice();
        while !data.is_empty() {
            hashed_packets.push(SignatureSubpacket::try_from(&mut data)?);
        }
        let unhashed_packets = Vec::new();
        //TODO
        let mut hashed_data = Vec::new();
        hashed_data.write_u8(0x04)?;
        hashed_data.write_u8(subtype.get_id())?;
        hashed_data.write_u8(pubkey_algorithm.get_id())?;
        hashed_data.write_u8(hash_algorithm.get_id())?;
        hashed_data.write_be_u16(hash_len)?;
        hashed_data.write_all_bytes(hashed_data_pkts.as_slice())?;
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
    PreferredV1SEIPDSymCiphers(), //TODO
    IssuerKeyId(),                //TODO
    NotationData(),               //TODO
    PreferredHashAlgorithms(PreferredHashAlgorithms),
    PreferredCompressionAlgorithms(PreferredCompressionAlgorithms),
    PreferredKeyServer(String),
    PrimaryUserId(), //TODO
    PolicyURI(),     //TODO
    KeyFlags(KeyFlags),
    SignersUserId(String),
    ReasonForRevocation(), //TODO
    Features(),            //TODO
    SignatureTarget(),     //TODO
    EmbeddedSignature(),   //TODO
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
        f.write_str(&format!(
            "[v{}] {}",
            self.vsn,
            to_hex_str_upper(&self.issuer)
        ))
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
        f.write_str(self.0.format_iso8601_basic().as_str())
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
        f.write_str(&self.0.format_iso8601_basic())
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
#[derive(Clone)]
pub struct PreferredHashAlgorithms(pub Vec<HashAlgorithm>);
impl Debug for PreferredHashAlgorithms {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut t = f.debug_tuple("PreferredHashAlgorithms");
        for a in &self.0 {
            t.field(a);
        }
        t.finish()
    }
}
impl TryFrom<&[u8]> for PreferredHashAlgorithms {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let mut out = Vec::new();
        while let Some(v) = value.next_u8()? {
            out.push(v.try_into()?);
        }
        Ok(PreferredHashAlgorithms(out))
    }
}
#[derive(Clone)]
pub struct PreferredCompressionAlgorithms(pub Vec<CompressionAlgorithm>);
impl Debug for PreferredCompressionAlgorithms {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut t = f.debug_tuple("PreferredCompressionAlgorithms");
        for a in &self.0 {
            t.field(a);
        }
        t.finish()
    }
}
impl TryFrom<&[u8]> for PreferredCompressionAlgorithms {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let mut out = Vec::new();
        while let Some(v) = value.next_u8()? {
            out.push(v.try_into()?);
        }
        Ok(PreferredCompressionAlgorithms(out))
    }
}
#[derive(Clone)]
pub struct KeyFlags(pub Vec<KeyFlag>);
impl Debug for KeyFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut t = f.debug_tuple("KeyFlags");
        for a in &self.0 {
            t.field(a);
        }
        t.finish()
    }
}
impl TryFrom<&[u8]> for KeyFlags {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(KeyFlags(KeyFlag::try_from(value)?))
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
            21 => {
                return Ok(SignatureSubpacket::PreferredHashAlgorithms(
                    PreferredHashAlgorithms::try_from(pkt_data.as_slice())?,
                ))
            }
            22 => {
                return Ok(SignatureSubpacket::PreferredCompressionAlgorithms(
                    PreferredCompressionAlgorithms::try_from(pkt_data.as_slice())?,
                ))
            }
            24 => {
                return Ok(SignatureSubpacket::PreferredKeyServer(
                    String::from_utf8_lossy(pkt_data.as_slice()).to_string(),
                ))
            }
            27 => {
                return Ok(SignatureSubpacket::KeyFlags(KeyFlags::try_from(
                    pkt_data.as_slice(),
                )?))
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
        let r = read_mpi(&mut value, false)?;
        let s = read_mpi(&mut value, false)?;
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
