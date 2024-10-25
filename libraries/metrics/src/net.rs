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
}
