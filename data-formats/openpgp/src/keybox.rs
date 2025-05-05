// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use crate::packets::{OpenPGPMessage, OpenPGPPacketData, SignatureSubpacket};
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use core::fmt::{Debug, Formatter};
use core::ops::DerefMut;
use irox_bits::{BitsErrorKind, Error};
use irox_cryptids::ed25519::Ed25519PublicKey;
use irox_cryptids::x25519::Curve25519PublicKey;
use irox_time::datetime::UTCDateTime;
use irox_tools::hex::to_hex_str_upper;
use irox_tools::sync::MaybeLocked;
use std::sync::{Mutex, MutexGuard};

#[derive(Default, Debug)]
pub struct Keybox {
    pubkeys: BTreeMap<Fingerprint, SharedPubkey>,
}
impl Keybox {
    pub fn find_fingerprint(&self, fp: &Fingerprint) -> Option<SharedPubkey> {
        for pk in self.pubkeys.values() {
            if &pk.get_fingerprint()? == fp {
                return Some(pk.clone());
            }
            for subkey in &pk.get_subkeys() {
                if &subkey.get_fingerprint()? == fp {
                    return Some(subkey.clone());
                }
            }
        }
        None
    }
    pub fn add_pubkey(&mut self, pk: PublicKey) -> Result<Fingerprint, Error> {
        let fp = pk.fingerprint.clone();
        self.pubkeys.insert(fp.clone(), SharedPubkey::new(pk));
        Ok(fp)
    }

    pub fn add_to_keybox(&mut self, msg: &OpenPGPMessage) -> Result<(), Error> {
        let mut last_pubkey = None;
        for pkt in &msg.packets {
            if let OpenPGPPacketData::PublicKey(pk) = &pkt.data {
                last_pubkey = Some(pk.add_to_keybox(self)?);
            } else if let OpenPGPPacketData::UserID(uid) = &pkt.data {
                if let Some(pk) = last_pubkey.as_ref().and_then(|fp| self.pubkeys.get_mut(fp)) {
                    pk.set_user_id(uid.clone());
                }
            } else if let OpenPGPPacketData::PublicSubkey(sk) = &pkt.data {
                if let Some(pk) = last_pubkey.as_ref().and_then(|fp| self.pubkeys.get_mut(fp)) {
                    pk.add_subkey(sk.try_into()?);
                }
            } else if let OpenPGPPacketData::Signature(sig) = &pkt.data {
                if let Some(pk) = last_pubkey.as_ref().and_then(|fp| self.pubkeys.get_mut(fp)) {
                    sig.update_pubkey(pk);
                }
                for pkt in sig.get_hashed_packets() {
                    if let SignatureSubpacket::KeyBlock(kb) = pkt {
                        self.add_to_keybox(&kb.msg)?;
                    }
                }
            }
        }
        Ok(())
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fingerprint(pub Box<[u8]>);
impl Debug for Fingerprint {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "fpr({})", to_hex_str_upper(self.0.as_ref()))
    }
}
impl From<&[u8]> for Fingerprint {
    fn from(value: &[u8]) -> Self {
        Fingerprint(value.into())
    }
}
impl<const N: usize> From<[u8; N]> for Fingerprint {
    fn from(value: [u8; N]) -> Self {
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
    pub(crate) data: PublicKeyData,
    pub(crate) user_id: Option<String>,
    pub(crate) created_on: Option<UTCDateTime>,
    pub(crate) valid_until: Option<UTCDateTime>,
    pub(crate) fingerprint: Fingerprint,
    pub(crate) keygrip: Option<Keygrip>,
    pub(crate) source: PublicKeySource,
    pub(crate) issuer: Option<Fingerprint>,
    pub(crate) subkeys: Vec<SharedPubkey>,
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
#[derive(Debug, Clone)]
pub struct SharedPubkey {
    pubkey: Arc<Mutex<PublicKey>>,
}
impl SharedPubkey {
    pub fn new(pubkey: PublicKey) -> Self {
        Self {
            pubkey: Arc::new(Mutex::new(pubkey)),
        }
    }
    pub fn get_user_id(&self) -> Option<String> {
        if let Ok(pk) = self.pubkey.try_lock() {
            pk.user_id.clone()
        } else {
            None
        }
    }
    pub fn set_user_id(&self, user_id: String) {
        if let Ok(mut pk) = self.pubkey.try_lock() {
            pk.user_id = Some(user_id);
        }
    }
    pub fn get_created_on(&self) -> Option<UTCDateTime> {
        if let Ok(pk) = self.pubkey.try_lock() {
            pk.created_on
        } else {
            None
        }
    }
    pub fn set_created_on(&self, created_on: UTCDateTime) {
        if let Ok(mut pk) = self.pubkey.try_lock() {
            pk.created_on = Some(created_on);
        }
    }
    pub fn get_valid_until(&self) -> Option<UTCDateTime> {
        if let Ok(pk) = self.pubkey.try_lock() {
            pk.valid_until
        } else {
            None
        }
    }
    pub fn set_valid_until(&self, valid_until: UTCDateTime) {
        if let Ok(mut pk) = self.pubkey.try_lock() {
            pk.valid_until = Some(valid_until);
        }
    }
    pub fn get_fingerprint(&self) -> Option<Fingerprint> {
        if let Ok(pk) = self.pubkey.try_lock() {
            Some(pk.fingerprint.clone())
        } else {
            None
        }
    }
    pub fn set_fingerprint(&self, fingerprint: Fingerprint) {
        if let Ok(mut pk) = self.pubkey.try_lock() {
            pk.fingerprint = fingerprint;
        }
    }
    pub fn get_keygrip(&self) -> Option<Keygrip> {
        if let Ok(pk) = self.pubkey.try_lock() {
            pk.keygrip.clone()
        } else {
            None
        }
    }
    pub fn set_keygrip(&self, keygrip: Keygrip) {
        if let Ok(mut pk) = self.pubkey.try_lock() {
            pk.keygrip = Some(keygrip);
        }
    }
    pub fn get_subkeys(&self) -> Vec<SharedPubkey> {
        if let Ok(pk) = self.pubkey.try_lock() {
            pk.subkeys.clone()
        } else {
            Vec::new()
        }
    }
    pub fn add_subkey(&self, subkey: PublicKey) {
        let subkey = SharedPubkey::new(subkey);
        if let Ok(mut pk) = self.pubkey.try_lock() {
            pk.subkeys.push(subkey.clone());
        }
    }
    pub fn read_lock(&self) -> Result<MutexGuard<'_, PublicKey>, Error> {
        self.pubkey
            .lock()
            .map_err(|_| Error::new(BitsErrorKind::FormatError, "Keybox lock error"))
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

pub enum MultiKeybox<'a> {
    Owned(Keybox),
    Borrowed(&'a mut Keybox),
    Shared(Arc<Mutex<Keybox>>),
}

impl<'a> MultiKeybox<'a> {
    pub fn new_empty() -> Self {
        MultiKeybox::Owned(Keybox::default())
    }
    pub fn as_ref(&'a self) -> Option<MaybeLocked<'a, Keybox>> {
        match self {
            MultiKeybox::Owned(b) => Some(MaybeLocked::Borrowed(b)),
            MultiKeybox::Borrowed(b) => Some(MaybeLocked::Borrowed(b)),
            MultiKeybox::Shared(b) => Some(MaybeLocked::Locked(b.lock().ok()?)),
        }
    }
    pub fn as_ref_mut(&'a mut self) -> Option<MaybeLocked<'a, Keybox>> {
        match self {
            MultiKeybox::Owned(b) => Some(MaybeLocked::MutBorrowed(b)),
            MultiKeybox::Borrowed(b) => Some(MaybeLocked::MutBorrowed(b)),
            MultiKeybox::Shared(b) => Some(MaybeLocked::Locked(b.lock().ok()?)),
        }
    }
    pub fn map_mut<F: FnMut(&mut Keybox) -> R, R>(&mut self, mut func: F) -> Option<R> {
        Some(match self {
            MultiKeybox::Owned(o) => func(o),
            MultiKeybox::Borrowed(o) => func(o),
            MultiKeybox::Shared(o) => {
                if let Ok(mut l) = o.lock() {
                    func(l.deref_mut())
                } else {
                    return None;
                }
            }
        })
    }
    pub fn find_fingerprint(&self, fp: &Fingerprint) -> Option<SharedPubkey> {
        match self {
            MultiKeybox::Owned(k) => k.find_fingerprint(fp),
            MultiKeybox::Borrowed(k) => k.find_fingerprint(fp),
            MultiKeybox::Shared(k) => {
                if let Ok(k) = k.try_lock() {
                    k.find_fingerprint(fp)
                } else {
                    None
                }
            }
        }
    }
}
