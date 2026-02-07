// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

mod data;
mod ops;
mod pubkey;
mod signature;

use core::fmt::{Debug, Formatter};
use core::ops::DerefMut;
pub use pubkey::*;
pub use signature::*;

use crate::keybox::{Fingerprint, MultiKeybox};
use crate::packets::data::LiteralData;
use crate::packets::ops::OnePassSignature;
use crate::types::Hash;
use irox_bits::{Bits, BitsWrapper, Error, ErrorKind, MutBits, SerializeToBits};
use irox_enums::{EnumIterItem, EnumName};
use irox_tools::hash::Hasher;

pub const MESSAGE_HEADER: &str = "-----BEGIN PGP MESSAGE-----";
pub const MESSAGE_FOOTER: &str = "-----END PGP MESSAGE-----";
pub const PUBKEY_HEADER: &str = "-----BEGIN PGP PUBLIC KEY BLOCK-----";
pub const PUBKEY_FOOTER: &str = "-----END PGP PUBLIC KEY BLOCK-----";
pub const PRIVKEY_HEADER: &str = "-----BEGIN PGP PRIVATE KEY BLOCK-----";
pub const PRIVKEY_FOOTER: &str = "-----END PGP PRIVATE KEY BLOCK-----";
pub const SIG_HEADER: &str = "-----BEGIN PGP SIGNATURE BLOCK-----";
pub const SIG_FOOTER: &str = "-----END PGP SIGNATURE BLOCK-----";
pub const SIGNED_MESSAGE_HEADER: &str = "-----BEGIN PGP SIGNED MESSAGE-----";
pub const SIGNED_MESSAGE_SIGNATURE: &str = "-----BEGIN PGP SIGNATURE-----";
pub const SIGNED_MESSAGE_FOOTER: &str = "-----END PGP SIGNATURE-----";

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIterItem, EnumName)]
pub enum OpenPGPPacketType {
    PubKeyEncryptedSessionKey,
    Signature,
    SymKeyEncryptedSessionKey,
    OnePassSignature,
    SecretKey,
    PublicKey,
    SecretSubkey,
    CompressedData,
    SymEncryptedData,
    Marker,
    LiteralData,
    Trust,
    UserID,
    PublicSubkey,
    UserAttribute,
    SymEncryptedIntegrityProtectedData,
    Padding,
}
impl OpenPGPPacketType {
    pub fn get_shorthand(&self) -> &'static str {
        match self {
            OpenPGPPacketType::PubKeyEncryptedSessionKey => "PKESK",
            OpenPGPPacketType::Signature => "SIG",
            OpenPGPPacketType::SymKeyEncryptedSessionKey => "SKESK",
            OpenPGPPacketType::OnePassSignature => "OPS",
            OpenPGPPacketType::SecretKey => "SECKEY",
            OpenPGPPacketType::PublicKey => "PUBKEY",
            OpenPGPPacketType::SecretSubkey => "SECSUBKEY",
            OpenPGPPacketType::CompressedData => "COMP",
            OpenPGPPacketType::SymEncryptedData => "SED",
            OpenPGPPacketType::Marker => "MARKER",
            OpenPGPPacketType::LiteralData => "LIT",
            OpenPGPPacketType::Trust => "TRUST",
            OpenPGPPacketType::UserID => "UID",
            OpenPGPPacketType::PublicSubkey => "PUBSUBKEY",
            OpenPGPPacketType::UserAttribute => "UAT",
            OpenPGPPacketType::SymEncryptedIntegrityProtectedData => "SEIPD",
            OpenPGPPacketType::Padding => "PADDING",
        }
    }
    pub fn get_packet_id(&self) -> u8 {
        match self {
            OpenPGPPacketType::PubKeyEncryptedSessionKey => 1,
            OpenPGPPacketType::Signature => 2,
            OpenPGPPacketType::SymKeyEncryptedSessionKey => 3,
            OpenPGPPacketType::OnePassSignature => 4,
            OpenPGPPacketType::SecretKey => 5,
            OpenPGPPacketType::PublicKey => 6,
            OpenPGPPacketType::SecretSubkey => 7,
            OpenPGPPacketType::CompressedData => 8,
            OpenPGPPacketType::SymEncryptedData => 9,
            OpenPGPPacketType::Marker => 10,
            OpenPGPPacketType::LiteralData => 11,
            OpenPGPPacketType::Trust => 12,
            OpenPGPPacketType::UserID => 13,
            OpenPGPPacketType::PublicSubkey => 14,
            OpenPGPPacketType::UserAttribute => 17,
            OpenPGPPacketType::SymEncryptedIntegrityProtectedData => 18,
            OpenPGPPacketType::Padding => 21,
        }
    }
}
impl TryFrom<u8> for OpenPGPPacketType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        for elem in Self::iter_items() {
            if elem.get_packet_id() == value {
                return Ok(elem);
            }
        }
        Err(ErrorKind::InvalidData.into())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenPGPPacketHeader {
    pub packet_type: OpenPGPPacketType,
    pub packet_length: u32,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OpenPGPPacketData {
    PublicKey(PubKeyPacket),
    PublicSubkey(PubKeyPacket),
    UserID(String),
    Signature(SignaturePacket),
    LiteralData(LiteralData),
    OnePassSignature(OnePassSignature),
    Unknown(Vec<u8>),
}
impl SerializeToBits for OpenPGPPacketData {
    fn serialize_to_bits<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        match self {
            OpenPGPPacketData::PublicKey(pk) => pk.serialize_to_bits(bits),
            OpenPGPPacketData::PublicSubkey(sk) => sk.serialize_to_bits(bits),
            OpenPGPPacketData::UserID(uid) => {
                bits.write_all_bytes(uid.as_bytes())?;
                Ok(uid.len())
            }
            OpenPGPPacketData::Signature(sig) => sig.serialize_to_bits(bits),
            // OpenPGPPacketData::LiteralData(_) => {}
            // OpenPGPPacketData::OnePassSignature(_) => {}
            // OpenPGPPacketData::Unknown(_) => {}
            _ => Ok(0),
        }
    }
}
impl TryFrom<(OpenPGPPacketType, Vec<u8>)> for OpenPGPPacketData {
    type Error = Error;

    fn try_from(value: (OpenPGPPacketType, Vec<u8>)) -> Result<Self, Self::Error> {
        let (ty, value) = value;
        match ty {
            OpenPGPPacketType::PublicKey => Ok(OpenPGPPacketData::PublicKey(
                PubKeyPacket::try_from(value.as_slice())?,
            )),
            OpenPGPPacketType::PublicSubkey => Ok(OpenPGPPacketData::PublicSubkey(
                PubKeyPacket::try_from(value.as_slice())?,
            )),
            OpenPGPPacketType::UserID => Ok(OpenPGPPacketData::UserID(
                String::from_utf8_lossy(value.as_slice()).to_string(),
            )),
            OpenPGPPacketType::Signature => Ok(OpenPGPPacketData::Signature(
                SignaturePacket::try_from(value.as_slice())?,
            )),
            OpenPGPPacketType::LiteralData => Ok(OpenPGPPacketData::LiteralData(
                LiteralData::try_from(value.as_slice())?,
            )),
            OpenPGPPacketType::OnePassSignature => Ok(OpenPGPPacketData::OnePassSignature(
                OnePassSignature::try_from(value.as_slice())?,
            )),
            _ => Ok(OpenPGPPacketData::Unknown(value)),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenPGPPacket {
    pub header: OpenPGPPacketHeader,
    pub data: OpenPGPPacketData,
}

pub struct OpenPGPPackeStream<'a, T: Bits> {
    inner: BitsWrapper<'a, T>,
}
impl<'a, T: Bits> OpenPGPPackeStream<'a, T> {
    pub fn new(inner: BitsWrapper<'a, T>) -> Self {
        Self { inner }
    }
    pub fn read_next_packet(&mut self) -> Result<Option<(u8, u32, Vec<u8>)>, Error> {
        let mut out = Vec::new();

        let Some(first) = self.inner.next_u8()? else {
            return Ok(None);
        };
        // out.push(first);
        let delim = (first & 0b11000000) >> 6;
        let (pkt_type, len) = match delim {
            0b10 => {
                // legacy
                let pkt_type = (first & 0b111100) >> 2;
                let len_ty = first & 0b11;
                let len = match len_ty {
                    0 => self.inner.read_u8()? as u32,
                    1 => self.inner.read_be_u16()? as u32,
                    2 => self.inner.read_be_u32()?,
                    _ => {
                        todo!()
                    }
                };
                // out.push(len);
                (pkt_type, len)
            }
            0b11 => {
                // new format
                let pkt_type = first & 0b111111;

                let len = read_newlength(self.inner.deref_mut())?;
                (pkt_type, len)
            }
            _ => {
                // invalid
                return Err(ErrorKind::InvalidData.into());
            }
        };

        self.inner.read_exact_into(len as usize, &mut out)?;

        Ok(Some((pkt_type, len, out)))
    }
}
pub fn read_newlength<T: Bits>(source: &mut T) -> Result<u32, Error> {
    let len_ty = source.read_u8()?;
    if len_ty < 192 {
        Ok(len_ty as u32)
    } else if len_ty < 224 {
        let o2 = source.read_u8()?;
        let len = (len_ty as u32 - 192) << 8;
        let len = len + o2 as u32 + 192;
        Ok(len)
    } else {
        let len = source.read_be_u32()?;
        Ok(len)
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenPGPMessage {
    pub packets: Vec<OpenPGPPacket>,
}

impl OpenPGPMessage {
    pub fn build_from<T: Bits>(source: &mut T) -> Result<Self, Error> {
        let mut packets = Vec::new();
        let mut stream = OpenPGPPackeStream::new(BitsWrapper::Borrowed(source));
        while let Some((ty, len, pkt)) = stream.read_next_packet()? {
            let ty: OpenPGPPacketType = ty.try_into()?;
            let header = OpenPGPPacketHeader {
                packet_type: ty,
                packet_length: len,
            };
            let data = OpenPGPPacketData::try_from((ty, pkt))?;
            packets.push(OpenPGPPacket { header, data });
        }
        Ok(Self { packets })
    }

    pub fn validate_signatures(
        &self,
        bx: &MultiKeybox,
    ) -> Result<Vec<SignatureValidationResult>, Error> {
        let mut out = Vec::new();
        let mut keydata: Option<(Vec<u8>, Fingerprint)> = None;
        let mut last_subkey: Option<(Vec<u8>, Fingerprint)> = None;
        let mut last_uid: Option<Vec<u8>> = None;
        for pkt in &self.packets {
            match &pkt.data {
                OpenPGPPacketData::Signature(sig) => {
                    let mut hasher: Hasher = sig.get_hash_alg().try_into()?;
                    let sigtype = sig.get_subtype();
                    let target = match sigtype {
                        SignatureSubtype::GenericCertification
                        | SignatureSubtype::PersonaCertification
                        | SignatureSubtype::CasualCertification
                        | SignatureSubtype::PositiveCertification => {
                            if let Some(pk) = keydata.as_ref() {
                                hasher.write(pk.0.as_ref());
                                if let Some(uid) = last_uid.as_ref() {
                                    hasher.write(uid.as_ref());
                                }
                                SignatureTarget::PublicKey(pk.1.clone())
                            } else {
                                SignatureTarget::UnknownTarget
                            }
                        }
                        SignatureSubtype::SubkeyBinding => {
                            if let Some(pk) = keydata.as_ref() {
                                hasher.write(pk.0.as_ref());
                            }
                            if let Some(sk) = last_subkey.as_ref() {
                                hasher.write(sk.0.as_ref());
                                SignatureTarget::Subkey(sk.1.clone())
                            } else {
                                SignatureTarget::UnknownTarget
                            }
                        }
                        _ => SignatureTarget::UnknownTarget,
                    };
                    out.push(SignatureValidationResult {
                        sigtype,
                        target,
                        signer: sig.get_signature_issuer(),
                        result: sig.validate_signature(bx, hasher),
                    });
                }
                OpenPGPPacketData::UserID(uid) => {
                    let mut buf = Vec::new();
                    buf.push(0xB4);
                    buf.extend_from_slice((uid.len() as u32).to_be_bytes().as_slice());
                    buf.extend_from_slice(uid.as_bytes());
                    last_uid = Some(buf);
                }
                OpenPGPPacketData::PublicKey(pk) => {
                    last_uid = None;
                    let mut buf = Vec::new();
                    let cnt = pkt.data.serialize_to_bits(&mut buf)? as u16;
                    let mut buf1 = Vec::new();
                    buf1.push(0x99);
                    buf1.write_be_u16(cnt)?;
                    buf1.append(&mut buf);
                    keydata = Some((buf1, pk.get_fingerprint().into()));
                }
                OpenPGPPacketData::PublicSubkey(sk) => {
                    let mut buf = Vec::new();
                    let cnt = pkt.data.serialize_to_bits(&mut buf)?;
                    let mut buf1 = Vec::new();
                    buf1.push(0x99);
                    buf1.write_be_u16(cnt as u16)?;
                    buf1.append(&mut buf);
                    last_subkey = Some((buf1, sk.get_fingerprint().into()));
                }
                _ => {}
            }
        }
        Ok(out)
    }
}
#[derive(Clone, Eq, PartialEq)]
pub enum SignatureTarget {
    PublicKey(Fingerprint),
    Subkey(Fingerprint),
    Data(Hash),
    EmbeddedData(Hash, Vec<u8>),
    UnknownTarget,
}
impl Debug for SignatureTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            SignatureTarget::PublicKey(fp) => write!(f, "PublicKey({fp:?})"),
            SignatureTarget::Subkey(fp) => write!(f, "Subkey({fp:?})"),
            SignatureTarget::Data(h) => write!(f, "Data({h:?})"),
            SignatureTarget::EmbeddedData(h, d) => {
                write!(f, "EmbeddedData({h:?}, {} bytes)", d.len())
            }
            SignatureTarget::UnknownTarget => write!(f, "Unknown"),
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
pub struct SignatureValidationResult {
    pub sigtype: SignatureSubtype,
    pub target: SignatureTarget,
    pub signer: Option<Fingerprint>,
    pub result: Result<(), Error>,
}
impl Debug for SignatureValidationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let name = if self.result.is_ok() {
            format!("valid signature ({:?})", self.sigtype)
        } else {
            format!("invalid signature ({:?})", self.sigtype)
        };
        let mut s = f.debug_struct(name.as_str());
        s.field("target", &self.target);
        match &self.signer {
            None => s.field("signer", &"None"),
            Some(fp) => s.field("signer", &fp),
        };
        if let Err(e) = &self.result {
            s.field("errors", &e);
        }
        s.finish()
    }
}
