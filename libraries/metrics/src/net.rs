// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crypto_box::aead::rand_core::RngCore;
use crypto_box::aead::{Aead, OsRng};
use crypto_secretbox::{Key, KeyInit};
use irox_bits::Bits;
use irox_tools::random::PRNG;
use std::io::Read;
use x25519_dalek::PublicKey;

const TAG_SIZE: usize = 16;
const PUBLIC: [u8; 32] = [
    0xA5, 0xF4, 0xFF, 0xDB, 0xC3, 0x8E, 0x35, 0xC9, 0xAB, 0xA4, 0xD9, 0x7D, 0x12, 0x49, 0x60, 0x02,
    0x4C, 0x78, 0x27, 0x6A, 0x19, 0x7A, 0xB9, 0x2F, 0x64, 0xF7, 0xE8, 0x13, 0xF3, 0xAF, 0x3D, 0x29,
];
const PRIVATE: [u8; 32] = [
    0xF8, 0x20, 0x18, 0x33, 0xD6, 0xFB, 0x43, 0x5F, 0x94, 0x7F, 0x6E, 0x2D, 0x3B, 0xF7, 0x05, 0x97,
    0x03, 0x7F, 0x5B, 0x80, 0xE0, 0x34, 0x11, 0x41, 0xA1, 0xE9, 0xDF, 0x0C, 0xFF, 0xDB, 0x55, 0xD4,
];

pub struct RoadWarrior {
    eph_public: x25519_dalek::PublicKey,
    ext_key: x25519_dalek::PublicKey,
    shared_secret: crypto_secretbox::Key,
    buf: Vec<u8>,
    rand: irox_tools::random::PcgXshRR,
}
impl RoadWarrior {
    pub fn new() -> RoadWarrior {
        let secret = x25519_dalek::EphemeralSecret::random_from_rng(&mut OsRng);
        let public = x25519_dalek::PublicKey::from(&secret);
        let ext_key = x25519_dalek::PublicKey::from(PUBLIC);

        let shared_secret = secret.diffie_hellman(&ext_key);
        let shared_secret = Key::from(shared_secret.to_bytes());
        let buf = Vec::with_capacity(4096);
        Self {
            shared_secret,
            eph_public: public,
            ext_key,
            buf,
            rand: irox_tools::random::PcgXshRR::default(),
        }
    }
    pub fn seal(&mut self, plaintext: &[u8]) -> &[u8] {
        self.buf.clear();
        self.buf.extend_from_slice(self.eph_public.as_bytes());

        let crypto = crypto_secretbox::XChaCha20Poly1305::new(&self.shared_secret);
        let nonce = self.nonce();
        let res = crypto.encrypt((&nonce).into(), plaintext).unwrap();
        self.buf.extend_from_slice(&nonce);
        self.buf.extend_from_slice(&res);
        self.buf.as_slice()
    }
    fn nonce(&mut self) -> [u8; 24] {
        let mut nonce = [0u8; 24];
        let v = nonce.as_mut();
        self.rand.fill(v);
        nonce
    }
}

pub struct HomeBase {
    secret: x25519_dalek::StaticSecret,
}
impl HomeBase {
    pub fn new() -> HomeBase {
        let secret = x25519_dalek::StaticSecret::from(PRIVATE);
        HomeBase { secret }
    }
    pub fn unseal(&self, mut data: &[u8]) -> Vec<u8> {
        let pubkey: [u8; 32] = Bits::read_exact(&mut data).unwrap();
        let nonce: [u8; 24] = Bits::read_exact(&mut data).unwrap();
        let ss = self.secret.diffie_hellman(&PublicKey::from(pubkey));
        let ss = Key::from(ss.to_bytes());
        let crypto = crypto_secretbox::XChaCha20Poly1305::new(&ss);
        let out = crypto.decrypt((&nonce).into(), data.as_ref()).unwrap();
        out
    }
}

pub fn random_data() -> Vec<u8> {
    // let length = (1200 - 48)/8;
    // let length = 1200/8;
    let length = 600 / 8;
    let mut data = Vec::with_capacity(length);
    let rng = &mut OsRng;
    for _ in 0..length {
        data.extend(rng.next_u64().to_be_bytes());
    }
    data
}

#[cfg(test)]
mod tests {
    use crate::{random_data, HomeBase, RoadWarrior};
    use irox_tools::assert_eq_hex_slice;
    use irox_tools::hex::HexDump;

    #[test]
    pub fn test() {
        let mut e = RoadWarrior::new();
        let data = random_data();
        (&data).hexdump();

        let ciphertext = e.seal(&data);
        ciphertext.hexdump();

        let hb = HomeBase::new();
        let decrypted = hb.unseal(ciphertext);
        decrypted.hexdump();
        assert_eq_hex_slice!(decrypted, data);
    }
}
