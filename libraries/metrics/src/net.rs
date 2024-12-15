// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::Error;
use crypto_box::aead::rand_core::RngCore;
use crypto_box::aead::{Aead, OsRng};
use crypto_secretbox::{Key, KeyInit};
use ed25519_dalek::Signer;
use irox_bits::Bits;
use irox_tools::random::PRNG;
use x25519_dalek::{PublicKey, StaticSecret};

pub struct RoadWarrior {
    public: x25519_dalek::PublicKey,
    ext_key: x25519_dalek::PublicKey,
    shared_secret: crypto_secretbox::Key,
    buf: Vec<u8>,
    rand: irox_tools::random::PcgXshRR,
}
impl RoadWarrior {
    pub fn new(pubkey: [u8; 32]) -> RoadWarrior {
        let secret = x25519_dalek::EphemeralSecret::random_from_rng(OsRng);
        let public = x25519_dalek::PublicKey::from(&secret);
        let ext_key = x25519_dalek::PublicKey::from(pubkey);

        let shared_secret = secret.diffie_hellman(&ext_key);
        let shared_secret = Key::from(shared_secret.to_bytes());
        let buf = Vec::with_capacity(4096);
        Self {
            shared_secret,
            public,
            ext_key,
            buf,
            rand: irox_tools::random::PcgXshRR::default(),
        }
    }
    pub fn seal(&mut self, plaintext: &[u8]) -> Result<&[u8], Error> {
        self.buf.clear();
        self.buf.extend_from_slice(self.public.as_bytes());

        let crypto = crypto_secretbox::XChaCha20Poly1305::new(&self.shared_secret);
        let nonce = self.nonce();
        let res = crypto.encrypt((&nonce).into(), plaintext)?;
        self.buf.extend_from_slice(&nonce);
        self.buf.extend_from_slice(&res);
        Ok(self.buf.as_slice())
    }
    fn nonce(&mut self) -> [u8; 24] {
        let mut nonce = [0u8; 24];
        let v = nonce.as_mut();
        self.rand.fill(v);
        nonce
    }
    pub fn verify(&self, msg: &[u8], sig: &[u8; 64]) -> Result<bool, Error> {
        let key = ed25519_dalek::VerifyingKey::from_bytes(self.ext_key.as_bytes())?;
        Ok(key
            .verify_strict(msg, &ed25519_dalek::Signature::from_bytes(sig))
            .is_ok())
    }
}

pub struct HomeBase {
    secret: x25519_dalek::StaticSecret,
}
impl HomeBase {
    pub fn new(privkey: [u8; 32]) -> HomeBase {
        let secret = x25519_dalek::StaticSecret::from(privkey);
        HomeBase { secret }
    }
    pub fn unseal(&self, mut data: &[u8]) -> Result<Vec<u8>, Error> {
        let pubkey: [u8; 32] = Bits::read_exact(&mut data)?;
        let nonce: [u8; 24] = Bits::read_exact(&mut data)?;
        let ss = self.secret.diffie_hellman(&PublicKey::from(pubkey));
        let ss = Key::from(ss.to_bytes());
        let crypto = crypto_secretbox::XChaCha20Poly1305::new(&ss);
        let out = crypto.decrypt((&nonce).into(), data.as_ref())?;
        Ok(out)
    }

    pub fn sign(&self, data: &[u8]) -> Result<[u8; 64], Error> {
        let sk = ed25519_dalek::SigningKey::from_bytes(self.secret.as_bytes());
        let sig = sk.sign(data).to_bytes();
        Ok(sig)
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

pub fn random_keys() -> ([u8; 32], [u8; 32]) {
    let sec = StaticSecret::random();
    let pubk = PublicKey::from(&sec);
    let sec = sec.to_bytes();
    let pubk = pubk.to_bytes();
    (pubk, sec)
}

#[cfg(test)]
mod tests {
    use crate::{random_data, random_keys, Error, HomeBase, RoadWarrior};
    use irox_tools::assert_eq_hex_slice;
    use irox_tools::hex::HexDump;

    #[test]
    pub fn test() -> Result<(), Error> {
        let (pubkey, privkey) = random_keys();

        let mut e = RoadWarrior::new(pubkey);
        let data = random_data();
        (&data).hexdump();

        let ciphertext = e.seal(&data)?;
        ciphertext.hexdump();

        let hb = HomeBase::new(privkey);
        let decrypted = hb.unseal(ciphertext)?;
        decrypted.hexdump();
        assert_eq_hex_slice!(decrypted, data);
        Ok(())
    }

    const PK2: [u8; 120] = [
        0x30, 0x76, 0x30, 0x10, 0x06, 0x07, 0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x02, 0x01, 0x06, 0x05,
        0x2B, 0x81, 0x04, 0x00, 0x22, 0x03, 0x62, 0x00, 0x04, 0x28, 0x6C, 0x16, 0x89, 0x92, 0x27,
        0x31, 0xBB, 0x4C, 0xDF, 0x59, 0x58, 0x84, 0xC4, 0x0B, 0x7D, 0xA4, 0x22, 0x8B, 0x8F, 0xCF,
        0x2A, 0xF6, 0x4D, 0x02, 0xAD, 0x05, 0x1D, 0x88, 0x7F, 0x5D, 0x2C, 0xF1, 0x0A, 0xE0, 0xD8,
        0xC4, 0x56, 0x41, 0xF6, 0x57, 0x6F, 0x22, 0x4B, 0xBF, 0xF3, 0xD6, 0xCD, 0x65, 0xAA, 0x1C,
        0xC9, 0xC4, 0xFB, 0x56, 0x0D, 0x1F, 0x76, 0x85, 0x81, 0xF1, 0xA2, 0x5E, 0x31, 0x9D, 0x0D,
        0xCD, 0xF4, 0x37, 0x4B, 0xD2, 0x39, 0x39, 0x1A, 0xF5, 0xA7, 0x3A, 0x51, 0x79, 0xAB, 0x4F,
        0xD0, 0xB9, 0x55, 0xF9, 0xDE, 0x93, 0xB1, 0x87, 0x33, 0xE7, 0x13, 0xEE, 0xB2, 0x9C, 0x48,
    ];
    #[test]
    pub fn test2() {
        use p384::elliptic_curve::PublicKey;
        use p384::NistP384;
        use spki::DecodePublicKey;

        let pk = PublicKey::<NistP384>::from_public_key_der(&PK2).unwrap();
        // println!("PK: {}", to_hex_array(&PK));
        println!("Decoded: {pk:#?}");
    }
}
