// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::keybox::{Fingerprint, Keybox, PublicKey};
use crate::packets::PubkeyAlgorithm;
use crate::types::{
    CompressionAlgorithm, Features, HashAlgorithm, KeyFlag, KeyServerPreference,
    SymmetricKeyAlgorithm, MPI,
};
use core::fmt::{Debug, Formatter};
use core::ops::Deref;
use irox_bits::{Bits, BitsErrorKind, Error, ErrorKind, MutBits, SerializeToBits};
use irox_cryptids::ed25519::Ed25519PublicKey;
use irox_enums::{EnumIterItem, EnumName};
use irox_time::datetime::UTCDateTime;
use irox_time::epoch::UnixTimestamp;
use irox_time::format::iso8601::ISO8601Format;
use irox_time::Duration;
use irox_tools::hash::HasherCounting;
use irox_tools::hex::to_hex_str_upper;

#[derive(Debug, Clone, Eq, PartialEq)]
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
    pub fn validate_signature(&self, bx: &Keybox, mut hash: HasherCounting) -> Result<(), Error> {
        let Some(fp) = self.get_signature_issuer() else {
            return Err(Error::new(
                BitsErrorKind::NotFound,
                "Signature issuer not present",
            ));
        };
        let Some(pk) = bx.find_fingerprint(&fp) else {
            return Err(Error::new(
                BitsErrorKind::NotFound,
                "Signature issuer pubkey not found in keybox",
            ));
        };
        if self.get_pubkey_alg() != PubkeyAlgorithm::EdDSALegacy {
            return Err(Error::new(
                BitsErrorKind::Unsupported,
                "Signature algorithm not supported",
            ));
        };

        let data = self.get_hashed_data();
        let cnt = data.len();
        // irox_tools::hex::HexDump::hexdump(&data);
        hash.write(data);
        hash.write_u8(self.get_version())?;
        hash.write_u8(0xFF)?;
        hash.write_be_u32(cnt as u32)?;
        let (_, data) = hash.finish();
        // data.as_ref().hexdump();
        let sig = self.try_into_ed25519_sig()?;
        let pk: &Ed25519PublicKey = (&pk.data).try_into()?;
        if let Err(e) = pk.verify_signed_message(&data, &sig) {
            return Err(Error::new(BitsErrorKind::InvalidInput, e.msg()));
        }
        Ok(())
    }
    pub fn get_signature_issuer(&self) -> Option<Fingerprint> {
        self.get_hashed_packets().iter().find_map(|p| {
            if let SignatureSubpacket::IssuerFingerprint(f) = p {
                Some(Fingerprint::from(f.issuer.as_ref()))
            } else {
                None
            }
        })
    }
    pub fn get_hashed_packets(&self) -> &[SignatureSubpacket] {
        match self {
            SignaturePacket::Version4(v) => v.hashed_packets.as_slice(),
        }
    }
    pub fn get_unhashed_packets(&self) -> &[SignatureSubpacket] {
        match self {
            SignaturePacket::Version4(v) => v.unhashed_packets.as_slice(),
        }
    }
    pub fn update_pubkey(&self, pk: &mut PublicKey) {
        if let Some(fp) = self.get_signature_issuer() {
            if pk.fingerprint != fp {
                return;
            }
        }
        for pkt in self.get_hashed_packets() {
            match pkt {
                SignatureSubpacket::KeyExpirationTime(ket) => {
                    if let Some(ct) = pk.created_on {
                        pk.valid_until = Some(ct + ket.0)
                    }
                }
                _ => {
                    // skip
                }
            }
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
impl SerializeToBits for SignaturePacket {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        match self {
            SignaturePacket::Version4(v4) => {
                bits.write_u8(4)?;
                let cnt = v4.serialize_to_bits(bits)?;
                Ok(cnt + 1)
            }
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
impl Eq for SigV4Packet {}
impl PartialEq for SigV4Packet {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
            && self.pubkey_algorithm == other.pubkey_algorithm
            && self.hash_algorithm == other.hash_algorithm
            && self.hashed_packets == other.hashed_packets
            && self.unhashed_packets == other.unhashed_packets
            && self.upper_signed_hash == other.upper_signed_hash
            && self.signature_data == other.signature_data
    }
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
impl SerializeToBits for SigV4Packet {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let mut len = 9;
        bits.write_u8(self.subtype.get_id())?;
        bits.write_u8(self.pubkey_algorithm.get_id())?;
        bits.write_u8(self.hash_algorithm.get_id())?;
        if !self.hashed_data.is_empty() {
            bits.write_be_u16(self.hashed_data.len() as u16)?;
            bits.write_all_bytes(self.hashed_data.as_slice())?;
            len += self.hashed_data.len();
        } else {
            let mut buf = Vec::new();
            for pkt in &self.hashed_packets {
                pkt.serialize_to_bits(&mut buf)?;
            }
            bits.write_be_u16(buf.len() as u16)?;
            bits.write_all_bytes(buf.as_slice())?;
            len += buf.len();
        }
        if !self.unhashed_data.is_empty() {
            bits.write_be_u16(self.unhashed_data.len() as u16)?;
            bits.write_all_bytes(self.unhashed_data.as_slice())?;
            len += self.unhashed_data.len();
        } else {
            let mut buf = Vec::new();
            for pkt in &self.unhashed_packets {
                pkt.serialize_to_bits(&mut buf)?;
            }
            bits.write_be_u16(buf.len() as u16)?;
            bits.write_all_bytes(buf.as_slice())?;
            len += buf.len();
        }
        bits.write_be_u16(self.upper_signed_hash)?;
        len += self.signature_data.serialize_to_bits(bits)?;
        Ok(len)
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
#[derive(Default, Clone)]
pub struct SigV4PacketBuilder {
    pub subtype: Option<SignatureSubtype>,
    pub pubkey_algorithm: Option<PubkeyAlgorithm>,
    pub hash_algorithm: Option<HashAlgorithm>,
    pub hashed_packets: Vec<SignatureSubpacket>,
    pub unhashed_packets: Vec<SignatureSubpacket>,
    pub upper_signed_hash: Option<u16>,
    pub signature_data: Option<SignatureData>,
}
impl SigV4PacketBuilder {
    #[must_use]
    pub fn with_subtype(mut self, subtype: SignatureSubtype) -> Self {
        self.subtype = Some(subtype);
        self
    }
    #[must_use]
    pub fn with_pubkey_algorithm(mut self, pubkey_algorithm: PubkeyAlgorithm) -> Self {
        self.pubkey_algorithm = Some(pubkey_algorithm);
        self
    }
    #[must_use]
    pub fn with_hash_algorithm(mut self, hash_algorithm: HashAlgorithm) -> Self {
        self.hash_algorithm = Some(hash_algorithm);
        self
    }
    #[must_use]
    pub fn with_hashed_packet(mut self, pkt: SignatureSubpacket) -> Self {
        self.hashed_packets.push(pkt);
        self
    }
    #[must_use]
    pub fn with_unhashed_packet(mut self, pkt: SignatureSubpacket) -> Self {
        self.unhashed_packets.push(pkt);
        self
    }
    #[must_use]
    pub fn with_upper_signed_hash(mut self, hash: u16) -> Self {
        self.upper_signed_hash = Some(hash);
        self
    }
    #[must_use]
    pub fn with_signature_data(mut self, data: SignatureData) -> Self {
        self.signature_data = Some(data);
        self
    }
    pub fn build(self) -> Result<SigV4Packet, Error> {
        let subtype = self
            .subtype
            .ok_or(Error::new(ErrorKind::InvalidInput, "Missing subtype"))?;
        let pubkey_algorithm = self.pubkey_algorithm.ok_or(Error::new(
            ErrorKind::InvalidInput,
            "Missing pubkey algorithm",
        ))?;
        let hash_algorithm = self.hash_algorithm.ok_or(Error::new(
            ErrorKind::InvalidInput,
            "Missing hash algorithm",
        ))?;
        let upper_signed_hash = self.upper_signed_hash.ok_or(Error::new(
            ErrorKind::InvalidInput,
            "Missing upper signed hash",
        ))?;
        let signature_data = self.signature_data.ok_or(Error::new(
            ErrorKind::InvalidInput,
            "Missing signature data",
        ))?;

        // let unhashed_data = write_to_vec(&self.unhashed_packets)?;
        // let hashed_data = write_to_vec(&self.hashed_packets)?;

        Ok(SigV4Packet {
            subtype,
            pubkey_algorithm,
            hash_algorithm,
            hashed_data: vec![],
            hashed_packets: self.hashed_packets,
            unhashed_data: vec![],
            unhashed_packets: self.unhashed_packets,
            upper_signed_hash,
            signature_data,
        })
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIterItem)]
pub enum SignatureSubpacketType {
    SigCreationTime,
    SigExpirationTime,
    ExportableCertification,
    TrustSignature,
    RegularExpression,
    Revocable,
    KeyExpirationTime,
    PreferredV1SEIPDSymCiphers,
    IssuerKeyId,
    NotationData,
    PreferredHashAlgorithms,
    PreferredCompressionAlgorithms,
    KeyServerPreferences,
    PreferredKeyServer,
    PrimaryUserId,
    PolicyURI,
    KeyFlags,
    SignersUserId,
    ReasonForRevocation,
    Features,
    SignatureTarget,
    EmbeddedSignature,
    IssuerFingerprint,
    IntendedRecipientFingerprint,
    PreferredAEADCipherSuites,
    #[skip]
    Unknown(u8),
}
impl SignatureSubpacketType {
    pub fn get_id(&self) -> u8 {
        match self {
            SignatureSubpacketType::SigCreationTime => 2,
            SignatureSubpacketType::SigExpirationTime => 3,
            SignatureSubpacketType::ExportableCertification => 4,
            SignatureSubpacketType::TrustSignature => 5,
            SignatureSubpacketType::RegularExpression => 6,
            SignatureSubpacketType::Revocable => 7,

            SignatureSubpacketType::KeyExpirationTime => 9,
            SignatureSubpacketType::PreferredV1SEIPDSymCiphers => 11,
            SignatureSubpacketType::IssuerKeyId => 16,
            SignatureSubpacketType::NotationData => 20,
            SignatureSubpacketType::PreferredHashAlgorithms => 21,
            SignatureSubpacketType::PreferredCompressionAlgorithms => 22,
            SignatureSubpacketType::KeyServerPreferences => 23,
            SignatureSubpacketType::PreferredKeyServer => 24,
            SignatureSubpacketType::PrimaryUserId => 25,
            SignatureSubpacketType::PolicyURI => 26,
            SignatureSubpacketType::KeyFlags => 27,
            SignatureSubpacketType::SignersUserId => 28,
            SignatureSubpacketType::ReasonForRevocation => 29,
            SignatureSubpacketType::Features => 30,
            SignatureSubpacketType::SignatureTarget => 31,
            SignatureSubpacketType::EmbeddedSignature => 32,
            SignatureSubpacketType::IssuerFingerprint => 33,
            SignatureSubpacketType::PreferredAEADCipherSuites => 34,
            SignatureSubpacketType::IntendedRecipientFingerprint => 35,
            SignatureSubpacketType::Unknown(v) => *v,
        }
    }
    pub fn try_parse_from(&self, pkt_data: Vec<u8>) -> Result<SignatureSubpacket, Error> {
        match self {
            SignatureSubpacketType::SigCreationTime => {
                return Ok(SignatureSubpacket::SigCreationTime(CreationTime::try_from(
                    pkt_data.as_slice(),
                )?));
            }
            SignatureSubpacketType::TrustSignature => {
                return Ok(SignatureSubpacket::TrustSignature(Trust::try_from(
                    pkt_data.as_slice(),
                )?));
            }
            SignatureSubpacketType::KeyExpirationTime => {
                return Ok(SignatureSubpacket::KeyExpirationTime(
                    KeyExpiration::try_from(pkt_data.as_slice())?,
                ))
            }
            SignatureSubpacketType::PreferredV1SEIPDSymCiphers => {
                return Ok(SignatureSubpacket::PreferredV1SEIPDSymCiphers(
                    PreferredV1SEIPDSymCiphers::try_from(pkt_data.as_slice())?,
                ));
            }
            SignatureSubpacketType::PreferredHashAlgorithms => {
                return Ok(SignatureSubpacket::PreferredHashAlgorithms(
                    PreferredHashAlgorithms::try_from(pkt_data.as_slice())?,
                ))
            }
            SignatureSubpacketType::PreferredCompressionAlgorithms => {
                return Ok(SignatureSubpacket::PreferredCompressionAlgorithms(
                    PreferredCompressionAlgorithms::try_from(pkt_data.as_slice())?,
                ))
            }
            SignatureSubpacketType::PreferredKeyServer => {
                return Ok(SignatureSubpacket::PreferredKeyServer(
                    String::from_utf8_lossy(pkt_data.as_slice()).to_string(),
                ))
            }
            SignatureSubpacketType::KeyFlags => {
                return Ok(SignatureSubpacket::KeyFlags(KeyFlags::try_from(
                    pkt_data.as_slice(),
                )?))
            }
            SignatureSubpacketType::Features => {
                return Ok(SignatureSubpacket::Features(FeaturesSubpkt::try_from(
                    pkt_data.as_slice(),
                )?));
            }
            SignatureSubpacketType::KeyServerPreferences => {
                return Ok(SignatureSubpacket::KeyServerPreferences(
                    KeyServerPreferences::try_from(pkt_data.as_slice())?,
                ));
            }
            SignatureSubpacketType::IssuerFingerprint => {
                return Ok(SignatureSubpacket::IssuerFingerprint(Issuer::try_from(
                    SignatureSubpacketType::IssuerFingerprint,
                    pkt_data.as_slice(),
                )?));
            }
            SignatureSubpacketType::PreferredAEADCipherSuites => {
                return Ok(SignatureSubpacket::PreferredAEADCiphersuites(
                    PreferredAEADSymCiphers::try_from(pkt_data.as_slice())?,
                ))
            }
            _ => {}
        }
        let pkt_type = self.get_id();
        Ok(SignatureSubpacket::Unknown(UnknownSubpacket {
            pkt_data,
            pkt_type,
        }))
    }
}
impl From<u8> for SignatureSubpacketType {
    fn from(value: u8) -> Self {
        for v in Self::iter_items() {
            if v.get_id() == value {
                return v;
            }
        }
        SignatureSubpacketType::Unknown(value)
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SignatureSubpacket {
    SigCreationTime(CreationTime),
    SigExpirationTime(),       //TODO
    ExportableCertification(), //TODO
    TrustSignature(Trust),
    RegularExpression(), //TODO
    Revocable(),         //TODO
    KeyExpirationTime(KeyExpiration),
    PreferredV1SEIPDSymCiphers(PreferredV1SEIPDSymCiphers), //TODO
    IssuerKeyId(Box<[u8]>),                                 //TODO
    NotationData(),                                         //TODO
    PreferredHashAlgorithms(PreferredHashAlgorithms),
    PreferredCompressionAlgorithms(PreferredCompressionAlgorithms),
    KeyServerPreferences(KeyServerPreferences),
    PreferredKeyServer(String),
    PrimaryUserId(), //TODO
    PolicyURI(),     //TODO
    KeyFlags(KeyFlags),
    SignersUserId(String),
    ReasonForRevocation(), //TODO
    Features(FeaturesSubpkt),
    SignatureTarget(),   //TODO
    EmbeddedSignature(), //TODO
    IssuerFingerprint(Issuer),
    IntendedRecipientFingerprint(), //TODO
    PreferredAEADCiphersuites(PreferredAEADSymCiphers),
    Unknown(UnknownSubpacket),
}
impl SerializeToBits for SignatureSubpacket {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        match self {
            SignatureSubpacket::SigCreationTime(s) => s.serialize_to_bits(bits),
            SignatureSubpacket::TrustSignature(t) => t.serialize_to_bits(bits),
            SignatureSubpacket::KeyExpirationTime(k) => k.serialize_to_bits(bits),
            SignatureSubpacket::PreferredV1SEIPDSymCiphers(p) => p.serialize_to_bits(bits),
            SignatureSubpacket::PreferredHashAlgorithms(p) => p.serialize_to_bits(bits),
            SignatureSubpacket::PreferredCompressionAlgorithms(p) => p.serialize_to_bits(bits),
            SignatureSubpacket::PreferredKeyServer(p) => {
                let len = write_subpkthdr(
                    bits,
                    p.len() as u32,
                    SignatureSubpacketType::PreferredKeyServer,
                )?;
                bits.write_all_bytes(p.as_bytes())?;
                Ok(len)
            }
            SignatureSubpacket::KeyFlags(k) => k.serialize_to_bits(bits),
            SignatureSubpacket::SignersUserId(s) => {
                let len = write_subpkthdr(
                    bits,
                    s.len() as u32,
                    SignatureSubpacketType::PreferredKeyServer,
                )?;
                bits.write_all_bytes(s.as_bytes())?;
                Ok(len)
            }
            SignatureSubpacket::IssuerFingerprint(i) => i.serialize_to_bits(bits),
            SignatureSubpacket::PreferredAEADCiphersuites(p) => p.serialize_to_bits(bits),
            SignatureSubpacket::Features(f) => f.serialize_to_bits(bits),
            SignatureSubpacket::KeyServerPreferences(k) => k.serialize_to_bits(bits),
            SignatureSubpacket::Unknown(u) => u.serialize_to_bits(bits),
            SignatureSubpacket::IssuerKeyId(kid) => {
                let len = write_subpkthdr(
                    bits,
                    kid.len() as u32 + 1,
                    SignatureSubpacketType::IssuerKeyId,
                )?;
                bits.write_all_bytes(kid.deref())?;
                Ok(len)
            }
            _ => todo!(),
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
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
impl SerializeToBits for UnknownSubpacket {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let len = write_subpkthdr(
            bits,
            self.pkt_data.len() as u32,
            SignatureSubpacketType::Unknown(self.pkt_type),
        )?;
        bits.write_all_bytes(&self.pkt_data)?;
        Ok(len)
    }
}
#[derive(Clone, Eq, PartialEq)]
pub struct Issuer {
    pub vsn: u8,
    pub subtype: SignatureSubpacketType,
    pub issuer: Box<[u8]>,
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
impl Issuer {
    pub fn try_from(subtype: SignatureSubpacketType, mut value: &[u8]) -> Result<Self, Error> {
        let vsn = value.read_u8()?;
        let issuer = match vsn {
            4 => value.read_exact_vec(20)?.into(),
            6 => value.read_exact_vec(32)?.into(),
            _ => return Err(ErrorKind::InvalidData.into()),
        };
        Ok(Issuer {
            vsn,
            subtype,
            issuer,
        })
    }
}
impl SerializeToBits for Issuer {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let len = write_subpkthdr(bits, self.issuer.len() as u32 + 2, self.subtype)?;
        bits.write_u8(self.vsn)?;
        bits.write_all_bytes(&self.issuer)?;
        Ok(len)
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
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
impl SerializeToBits for CreationTime {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let len = write_subpkthdr(bits, 5, SignatureSubpacketType::SigCreationTime)?;
        let time: UnixTimestamp = self.0.into();
        bits.write_be_u32(time.get_offset().as_seconds() as u32)?;
        Ok(len)
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
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
impl SerializeToBits for KeyExpiration {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let len = write_subpkthdr(bits, 4, SignatureSubpacketType::KeyExpirationTime)?;
        let time: UnixTimestamp = self.0.into();
        bits.write_be_u32(time.elapsed().as_seconds() as u32)?;
        Ok(len)
    }
}
#[derive(Clone, Eq, PartialEq)]
pub struct PreferredV1SEIPDSymCiphers(pub Vec<SymmetricKeyAlgorithm>);
impl Debug for PreferredV1SEIPDSymCiphers {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut t = f.debug_tuple("PreferredV1SEIPDSymCiphers");
        for a in &self.0 {
            t.field(a);
        }
        t.finish()
    }
}
impl TryFrom<&[u8]> for PreferredV1SEIPDSymCiphers {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let mut out = Vec::new();
        while let Some(v) = value.next_u8()? {
            out.push(v.try_into()?);
        }
        Ok(PreferredV1SEIPDSymCiphers(out))
    }
}
impl SerializeToBits for PreferredV1SEIPDSymCiphers {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let len = self.0.len();
        let len = write_subpkthdr(
            bits,
            len as u32 + 1,
            SignatureSubpacketType::PreferredV1SEIPDSymCiphers,
        )?;
        for v in &self.0 {
            bits.write_u8(v.get_id())?;
        }
        Ok(len)
    }
}
#[derive(Clone, Eq, PartialEq)]
pub struct PreferredAEADSymCiphers(pub Vec<SymmetricKeyAlgorithm>);
impl Debug for PreferredAEADSymCiphers {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut t = f.debug_tuple("PreferredAEADSymCiphers");
        for a in &self.0 {
            t.field(a);
        }
        t.finish()
    }
}
impl TryFrom<&[u8]> for PreferredAEADSymCiphers {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let mut out = Vec::new();
        while let Some(v) = value.next_u8()? {
            out.push(v.try_into()?);
        }
        Ok(PreferredAEADSymCiphers(out))
    }
}
impl SerializeToBits for PreferredAEADSymCiphers {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let len = self.0.len();
        let len = write_subpkthdr(
            bits,
            len as u32 + 1,
            SignatureSubpacketType::PreferredAEADCipherSuites,
        )?;
        for v in &self.0 {
            bits.write_u8(v.get_id())?;
        }
        Ok(len)
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
impl SerializeToBits for Trust {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let len = write_subpkthdr(bits, 2, SignatureSubpacketType::TrustSignature)?;
        bits.write_u8(self.depth)?;
        bits.write_u8(self.trust_amount)?;
        Ok(len)
    }
}
#[derive(Clone, Eq, PartialEq)]
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
impl SerializeToBits for PreferredHashAlgorithms {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let len = self.0.len();
        let len = write_subpkthdr(
            bits,
            len as u32 + 1,
            SignatureSubpacketType::PreferredHashAlgorithms,
        )?;
        for v in &self.0 {
            bits.write_u8(v.get_id())?;
        }
        Ok(len)
    }
}
#[derive(Clone, Eq, PartialEq)]
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
impl SerializeToBits for PreferredCompressionAlgorithms {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let len = self.0.len();
        let len = write_subpkthdr(
            bits,
            len as u32 + 1,
            SignatureSubpacketType::PreferredCompressionAlgorithms,
        )?;
        for v in &self.0 {
            bits.write_u8(v.get_id())?;
        }
        Ok(len)
    }
}
#[derive(Clone, Eq, PartialEq)]
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
impl SerializeToBits for KeyFlags {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_u8(0x02)?;
        bits.write_u8(SignatureSubpacketType::KeyFlags.get_id())?;
        let mut flags = 0u8;
        for v in &self.0 {
            flags |= v.get_id();
        }
        bits.write_u8(flags)?;
        Ok(3)
    }
}
#[derive(Clone, Eq, PartialEq)]
pub struct KeyServerPreferences(pub Vec<KeyServerPreference>);
impl Debug for KeyServerPreferences {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut t = f.debug_tuple("KeyServerPreferences");
        for a in &self.0 {
            t.field(a);
        }
        t.finish()
    }
}
impl TryFrom<&[u8]> for KeyServerPreferences {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(KeyServerPreferences(KeyServerPreference::try_from(value)?))
    }
}
impl SerializeToBits for KeyServerPreferences {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_u8(0x02)?;
        bits.write_u8(SignatureSubpacketType::KeyServerPreferences.get_id())?;
        let mut flags = 0u8;
        for v in &self.0 {
            flags |= v.get_id();
        }
        bits.write_u8(flags)?;
        Ok(2)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct FeaturesSubpkt(pub Vec<Features>);
impl Debug for FeaturesSubpkt {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut t = f.debug_tuple("Features");
        for a in &self.0 {
            t.field(a);
        }
        t.finish()
    }
}
impl TryFrom<&[u8]> for FeaturesSubpkt {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(FeaturesSubpkt(Features::try_from(value)?))
    }
}
impl SerializeToBits for FeaturesSubpkt {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_u8(0x02)?;
        bits.write_u8(SignatureSubpacketType::Features.get_id())?;
        let mut flags = 0u8;
        for v in &self.0 {
            flags |= v.get_id();
        }
        bits.write_u8(flags)?;
        Ok(2)
    }
}
impl SignatureSubpacket {
    fn try_from<T: Bits>(value: &mut T) -> Result<Self, Error> {
        let len = read_subpktlen(value)? - 1;
        let pkt_type: SignatureSubpacketType = value.read_u8()?.into();
        let pkt_data = value.read_exact_vec(len as usize)?;

        pkt_type.try_parse_from(pkt_data)
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
impl SerializeToBits for SignatureData {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        match self {
            // SignatureData::RSA() => {}
            // SignatureData::ECSDA() => {}
            SignatureData::EdDSALegacy(ed) => ed.serialize_to_bits(bits),
            // SignatureData::Ed25519Legacy() => {}
            // SignatureData::Ed25519() => {}
            // SignatureData::Ed448() => {
            _ => Err(ErrorKind::Unsupported.into()),
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
pub struct EdDSALegacySignature {
    pub r: MPI,
    pub s: MPI,
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
        let r = MPI::try_from(&mut value, false)?;
        let s = MPI::try_from(&mut value, false)?;
        Ok(EdDSALegacySignature { r, s })
    }
}
impl SerializeToBits for EdDSALegacySignature {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let mut len = self.r.serialize_to_bits(bits)?;
        len += self.s.serialize_to_bits(bits)?;
        Ok(len)
    }
}

pub(crate) fn read_subpktlen<T: Bits>(source: &mut T) -> Result<u32, Error> {
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
pub(crate) fn write_subpktlen<T: MutBits + ?Sized>(bits: &mut T, len: u32) -> Result<usize, Error> {
    if len < 192 {
        bits.write_u8(len as u8)?;
        Ok(1)
    } else if len <= 0x3FBF {
        let len = (len - 192) as u16 | 0xC000;
        bits.write_be_u16(len)?;
        Ok(2)
    } else {
        bits.write_u8(0xFF)?;
        bits.write_be_u32(len)?;
        Ok(5)
    }
}

pub(crate) fn write_subpkthdr<T: MutBits + ?Sized>(
    bits: &mut T,
    len: u32,
    pkt_type: SignatureSubpacketType,
) -> Result<usize, Error> {
    let len = write_subpktlen(bits, len)?;
    bits.write_u8(pkt_type.get_id())?;
    Ok(len + 1)
}
