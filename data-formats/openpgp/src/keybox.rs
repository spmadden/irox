// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use alloc::collections::BTreeMap;
use irox_cryptids::ed25519::Ed25519PublicKey;
use irox_cryptids::x25519::Curve25519PublicKey;
use irox_time::datetime::UTCDateTime;

pub struct Keybox {
    pub pubkeys: BTreeMap<Box<[u8]>, PublicKey>,
}

pub struct PublicKey {
    pub data: PublicKeyData,
    pub user_id: Option<String>,
    pub created_on: Option<UTCDateTime>,
    pub valid_until: Option<UTCDateTime>,
    pub fingerprint: Option<[u8; 20]>,
    pub keygrip: Option<[u8; 20]>,
    pub source: PublicKeySource,
    pub subkeys: Vec<PublicKey>,
}

pub enum PublicKeyData {
    X25519(Curve25519PublicKey),
    Ed25519(Ed25519PublicKey),
}

pub enum PublicKeySource {
    Raw,
    OpenPGP(),
}
pub struct OpenPGPPubKeyInfo;
