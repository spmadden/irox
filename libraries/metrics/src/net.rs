// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::Error;
use irox_bits::{Bits, BitsWrapper, MutBits};
use irox_cryptids::x25519::{Curve25519PublicKey, SecretKey, SharedCurve25519Secret};
use irox_cryptids::{fill_random_bits, ChaCha20Poly1305Decrypt, ChaCha20Poly1305Encrypt};
use irox_tools::buf::ZeroedBuffer;
use irox_tools::random::PRNG;

pub struct RoadWarrior {
    public: Curve25519PublicKey,
    ext_key: Curve25519PublicKey,
    shared_secret: SharedCurve25519Secret,
    buf: Vec<u8>,
    rand: irox_tools::random::PcgXshRR,
}
impl RoadWarrior {
    pub fn new(ext_key: Curve25519PublicKey) -> Result<RoadWarrior, Error> {
        let secret: SecretKey = SecretKey::generate_random()?;
        let public = secret.generate_curve25519_pubkey()?;

        let shared_secret = secret.generate_curve25519_shared_secret(&ext_key)?;
        let buf = Vec::with_capacity(4096);
        Ok(Self {
            shared_secret,
            public,
            ext_key,
            buf,
            rand: irox_tools::random::PcgXshRR::default(),
        })
    }
    #[allow(clippy::indexing_slicing)]
    pub fn seal(&mut self, plaintext: &[u8]) -> Result<&[u8], Error> {
        self.buf.clear();
        self.buf.extend_from_slice(self.public.as_ref());
        let nonce = self.nonce();
        self.buf.extend_from_slice(&nonce);

        self.buf.resize(32 + 12 + plaintext.len(), 0);

        let mut ciphbuf = &mut self.buf[44..(plaintext.len() + 44)];
        let out = BitsWrapper::Borrowed(&mut ciphbuf);
        let mut aad = self.public.as_ref();
        let aad = BitsWrapper::Borrowed(&mut aad);
        let mut crypto =
            ChaCha20Poly1305Encrypt::new(out, self.shared_secret.as_ref(), &nonce, aad);
        crypto.write_all_bytes(plaintext)?;
        let res = crypto.finish()?;
        self.buf.extend_from_slice(&res);
        Ok(self.buf.as_slice())
    }
    fn nonce(&mut self) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        let v = nonce.as_mut();
        self.rand.fill(v);
        nonce
    }
    pub fn verify(&self, msg: &[u8], sig: &[u8; 64]) -> Result<bool, Error> {
        self.ext_key
            .as_signing_key()
            .verify_signed_message(msg, sig)?;
        Ok(true)
    }
}

pub struct HomeBase {
    secret: SecretKey,
}
impl HomeBase {
    pub fn new(secret: SecretKey) -> Result<HomeBase, Error> {
        Ok(HomeBase { secret })
    }
    #[allow(clippy::indexing_slicing)]
    pub fn unseal(&self, mut data: &[u8]) -> Result<Vec<u8>, Error> {
        let pubkey: [u8; 32] = Bits::read_exact(&mut data)?;
        let nonce: [u8; 12] = Bits::read_exact(&mut data)?;

        let end = data.len() - 16;
        let mut encbuf = &data[..end];
        let tag: [u8; 16] = Bits::read_exact(&mut &data[end..])?;
        let pk = Curve25519PublicKey::try_from(pubkey)?;
        let ss = self.secret.generate_curve25519_shared_secret(&pk)?;
        let mut dec = ChaCha20Poly1305Decrypt::new(
            BitsWrapper::Borrowed(&mut encbuf),
            ss.as_ref(),
            &nonce,
            BitsWrapper::Borrowed(&mut pubkey.as_ref()),
        );

        let mut out = Vec::new();
        dec.read_all_into(&mut out)?;
        dec.finish(&tag)?;
        Ok(out)
    }

    // pub fn sign(&self, data: &[u8]) -> Result<[u8; 64], Error> {
    //     let sk = ed25519_dalek::SigningKey::from_bytes(self.secret.as_bytes());
    //     let sig = sk.sign(data).to_bytes();
    //     Ok(sig)
    // }
}

pub fn random_data() -> Result<Vec<u8>, Error> {
    // let length = (1200 - 48)/8;
    // let length = 1200/8;
    let length = 600 / 8;
    let mut data = Vec::new_zeroed(length);
    fill_random_bits(&mut data.as_mut_slice())?;
    Ok(data)
}

pub fn random_keys() -> Result<(Curve25519PublicKey, SecretKey), Error> {
    let sec = SecretKey::generate_random()?;
    let pk = sec.generate_curve25519_pubkey()?;
    Ok((pk, sec))
}

#[cfg(test)]
mod tests {
    use crate::{random_data, random_keys, Error, HomeBase, RoadWarrior};
    use irox_tools::assert_eq_hex_slice;
    use irox_tools::hex::HexDump;

    #[test]
    pub fn test() -> Result<(), Error> {
        let (pubkey, privkey) = random_keys()?;

        let mut e = RoadWarrior::new(pubkey)?;
        let data = random_data()?;
        (&mut data.as_slice()).hexdump();

        let ciphertext = e.seal(&data)?;
        ciphertext.iter().as_slice().hexdump();

        let hb = HomeBase::new(privkey)?;
        let decrypted = hb.unseal(ciphertext)?;
        decrypted.as_slice().hexdump();
        assert_eq_hex_slice!(decrypted, data);
        Ok(())
    }

    // const PK2: [u8; 120] = [
    //     0x30, 0x76, 0x30, 0x10, 0x06, 0x07, 0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x02, 0x01, 0x06, 0x05,
    //     0x2B, 0x81, 0x04, 0x00, 0x22, 0x03, 0x62, 0x00, 0x04, 0x28, 0x6C, 0x16, 0x89, 0x92, 0x27,
    //     0x31, 0xBB, 0x4C, 0xDF, 0x59, 0x58, 0x84, 0xC4, 0x0B, 0x7D, 0xA4, 0x22, 0x8B, 0x8F, 0xCF,
    //     0x2A, 0xF6, 0x4D, 0x02, 0xAD, 0x05, 0x1D, 0x88, 0x7F, 0x5D, 0x2C, 0xF1, 0x0A, 0xE0, 0xD8,
    //     0xC4, 0x56, 0x41, 0xF6, 0x57, 0x6F, 0x22, 0x4B, 0xBF, 0xF3, 0xD6, 0xCD, 0x65, 0xAA, 0x1C,
    //     0xC9, 0xC4, 0xFB, 0x56, 0x0D, 0x1F, 0x76, 0x85, 0x81, 0xF1, 0xA2, 0x5E, 0x31, 0x9D, 0x0D,
    //     0xCD, 0xF4, 0x37, 0x4B, 0xD2, 0x39, 0x39, 0x1A, 0xF5, 0xA7, 0x3A, 0x51, 0x79, 0xAB, 0x4F,
    //     0xD0, 0xB9, 0x55, 0xF9, 0xDE, 0x93, 0xB1, 0x87, 0x33, 0xE7, 0x13, 0xEE, 0xB2, 0x9C, 0x48,
    // ];
    // #[test]
    // pub fn test2() {
    //     use p384::elliptic_curve::PublicKey;
    //     use p384::NistP384;
    //     use spki::DecodePublicKey;
    //
    //     let pk = PublicKey::<NistP384>::from_public_key_der(&PK2).unwrap();
    //     // println!("PK: {}", to_hex_array(&PK));
    //     println!("Decoded: {pk:#?}");
    // }
}
