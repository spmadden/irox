// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Chacha20, Poly1305};
use core::ops::DerefMut;
use irox_bits::{Bits, BitsWrapper, Error, MutBits};

pub struct ChaCha20Poly1305Encrypt<'a, OUT: MutBits> {
    keystream: Chacha20,
    macdata: Poly1305,
    output: BitsWrapper<'a, OUT>,
    written: u64,
    aadlen: u64,
    cipherlen: u64,
}
impl<'a, OUT: MutBits> ChaCha20Poly1305Encrypt<'a, OUT> {
    pub fn new<AAD: Bits>(
        output: BitsWrapper<'a, OUT>,
        key: &[u8; 32],
        nonce: &[u8; 12],
        mut aad: BitsWrapper<'_, AAD>,
    ) -> Self {
        let mut keystream = Chacha20::new(*key, *nonce);
        keystream.set_counter(1);
        let otk = Poly1305::key_gen(key, nonce);
        let mut macdata = Poly1305::new(&otk);
        let aadlen = macdata
            .write_all_into_self_from(aad.deref_mut())
            .unwrap_or_default();
        let written = aadlen;
        let mut out = Self {
            keystream,
            output,
            macdata,
            written,
            aadlen,
            cipherlen: 0,
        };
        let _ = out.write_pad(aadlen);
        out
    }
    pub fn write(&mut self, mut data: &[u8]) -> Result<(), Error> {
        let mut buf = [0u8; 16];
        loop {
            let proc = self.keystream.process(data, &mut buf);
            if proc == 0 {
                break;
            }
            data = data.split_at(proc).1;
            for v in buf.iter().take(proc) {
                self.macdata.write_u8(*v)?;
                self.output.write_u8(*v)?;
                self.cipherlen += 1;
                self.written += 1;
            }
        }
        Ok(())
    }
    fn write_pad(&mut self, len: u64) -> Result<(), Error> {
        let towrite = 0x10 - (len & 0xF);
        for _ in 0..towrite {
            self.macdata.write_u8(0)?;
        }
        self.written += towrite;
        Ok(())
    }
    pub fn finish(mut self) -> Result<(), Error> {
        self.write_pad(self.cipherlen)?;
        self.macdata.write_le_u64(self.aadlen)?;
        self.macdata.write_le_u64(self.cipherlen)?;
        let tag = self.macdata.finish();
        self.output.write_all_bytes(&tag)?;
        Ok(())
    }
}
impl<OUT: MutBits> MutBits for ChaCha20Poly1305Encrypt<'_, OUT> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.write(&[val])
    }
}

#[cfg(test)]
mod tests {
    use crate::ChaCha20Poly1305Encrypt;
    use irox_bits::{BitsWrapper, Error};
    use irox_tools::{assert_eq_hex_slice, hex};

    static SUNSCREEN: &[u8] = b"\
Ladies and Gentlemen of the class of '99: \
If I could offer you only one tip for \
the future, sunscreen would be it.";

    #[test]
    pub fn test_aead() -> Result<(), Error> {
        let aad = hex!("50515253c0c1c2c3c4c5c6c7");
        let key = hex!("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f");
        let nonce = hex!("070000004041424344454647");
        let mut out = Vec::new();
        let obuf = BitsWrapper::Borrowed(&mut out);
        let aadbuf = BitsWrapper::Owned(aad.as_slice());
        let mut enc = ChaCha20Poly1305Encrypt::new(obuf, &key, &nonce, aadbuf);
        enc.write(SUNSCREEN)?;
        enc.finish()?;

        let enc = hex!(
            "d31a8d34648e60db7b86afbc53ef7ec2"
            "a4aded51296e08fea9e2b5a736ee62d6"
            "3dbea45e8ca9671282fafb69da92728b"
            "1a71de0a9e060b2905d6a5b67ecd3b36"
            "92ddbd7f2d778b8c9803aee328091b58"
            "fab324e4fad675945585808b4831d7bc"
            "3ff4def08e4b7a9de576d26586cec64b"
            "6116"
        );
        let mut exp = Vec::new();
        exp.extend_from_slice(&enc);
        exp.extend_from_slice(&hex!("1ae10b594f09e26a7e902ecbd0600691"));
        assert_eq_hex_slice!(exp.as_slice(), out.as_slice());

        Ok(())
    }
}
