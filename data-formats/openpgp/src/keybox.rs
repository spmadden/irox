// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use alloc::collections::BTreeMap;
use core::fmt::{Debug, Formatter};
use irox_bits::{BitsErrorKind, Error};
use irox_cryptids::ed25519::Ed25519PublicKey;
use irox_cryptids::x25519::Curve25519PublicKey;
use irox_time::datetime::UTCDateTime;
use irox_tools::hex::to_hex_str_upper;

#[derive(Default, Debug)]
pub struct Keybox {
    pub pubkeys: BTreeMap<Fingerprint, PublicKey>,
}
impl Keybox {
    pub fn find_fingerprint(&self, fp: &Fingerprint) -> Option<&PublicKey> {
        for pk in self.pubkeys.values() {
            if &pk.fingerprint == fp {
                return Some(pk);
            }
            for subkey in &pk.subkeys {
                if &subkey.fingerprint == fp {
                    return Some(subkey);
                }
            }
        }
        None
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fingerprint(pub Box<[u8]>);
impl Debug for Fingerprint {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Fingerprint")
            .field(&to_hex_str_upper(self.0.as_ref()))
            .finish()
    }
}
impl From<&[u8]> for Fingerprint {
    fn from(value: &[u8]) -> Self {
        Fingerprint(value.into())
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keygrip(pub Box<[u8]>);
impl Debug for Keygrip {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Keygrip")
            .field(&to_hex_str_upper(self.0.as_ref()))
            .finish()
    }
}
pub struct PublicKey {
    pub data: PublicKeyData,
    pub user_id: Option<String>,
    pub created_on: Option<UTCDateTime>,
    pub valid_until: Option<UTCDateTime>,
    pub fingerprint: Fingerprint,
    pub keygrip: Option<Keygrip>,
    pub source: PublicKeySource,
    pub issuer: Option<Fingerprint>,
    pub subkeys: Vec<PublicKey>,
}
impl Debug for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PublicKey")
            .field("data", &self.data)
            .field("user_id", &self.user_id)
            .field(
                "created_on",
                &self.created_on.map(|v| v.format_iso8601_basic()),
            )
            .field(
                "valid_until",
                &self.valid_until.map(|v| v.format_iso8601_basic()),
            )
            .field("fingerprint", &self.fingerprint)
            .field("keygrip", &self.keygrip)
            .field("source", &self.source)
            .field("issuer", &self.issuer)
            .field("subkeys", &self.subkeys)
            .finish()
    }
}

pub enum PublicKeyData {
    X25519(Curve25519PublicKey),
    Ed25519(Ed25519PublicKey),
}
impl<'a> TryFrom<&'a PublicKeyData> for &'a Ed25519PublicKey {
    type Error = Error;
    fn try_from(value: &'a PublicKeyData) -> Result<Self, Self::Error> {
        match value {
            PublicKeyData::Ed25519(e) => Ok(e),
            _ => Err(Error::new(BitsErrorKind::FormatError, "Not Ed25519 PK")),
        }
    }
}
impl Debug for PublicKeyData {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            PublicKeyData::X25519(x) => f
                .debug_tuple("X25519")
                .field(&to_hex_str_upper(x.as_ref()))
                .finish(),
            PublicKeyData::Ed25519(e) => f
                .debug_tuple("Ed25519")
                .field(&to_hex_str_upper(e.as_ref()))
                .finish(),
        }
    }
}

#[derive(Debug)]
pub enum PublicKeySource {
    Raw,
    OpenPGP(),
}
pub struct OpenPGPPubKeyInfo;
