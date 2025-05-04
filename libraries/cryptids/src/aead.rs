// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Chacha20, Poly1305};
use core::ops::DerefMut;
use irox_bits::{Bits, BitsWrapper, Error, ErrorKind, MutBits};

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
    pub fn finish(mut self) -> Result<[u8; 16], Error> {
        self.write_pad(self.cipherlen)?;
        self.macdata.write_le_u64(self.aadlen)?;
        self.macdata.write_le_u64(self.cipherlen)?;
        let tag = self.macdata.finish();
        Ok(tag)
    }
}
impl<OUT: MutBits> MutBits for ChaCha20Poly1305Encrypt<'_, OUT> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.write(&[val])
    }
}

pub struct ChaCha20Poly1305Decrypt<'a, IN: Bits> {
    keystream: Chacha20,
    macdata: Poly1305,
    input: BitsWrapper<'a, IN>,
    read: u64,
    aadlen: u64,
    cipherlen: u64,
}
impl<'a, IN: Bits> ChaCha20Poly1305Decrypt<'a, IN> {
    pub fn new<AAD: Bits>(
        input: BitsWrapper<'a, IN>,
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
            input,
            macdata,
            read: written,
            aadlen,
            cipherlen: 0,
        };
        let _ = out.write_pad(aadlen);
        out
    }
    fn write_pad(&mut self, len: u64) -> Result<(), Error> {
        let towrite = 0x10 - (len & 0xF);
        for _ in 0..towrite {
            self.macdata.write_u8(0)?;
        }
        self.read += towrite;
        Ok(())
    }
    pub fn finish(mut self, tag: &[u8; 16]) -> Result<(), Error> {
        self.write_pad(self.cipherlen)?;
        self.macdata.write_le_u64(self.aadlen)?;
        self.macdata.write_le_u64(self.cipherlen)?;
        let calc = self.macdata.finish();
        if tag != &calc {
            return Err(ErrorKind::InvalidData.into());
        }
        Ok(())
    }
}
impl<IN: Bits> Bits for ChaCha20Poly1305Decrypt<'_, IN> {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        Ok(if let Some(v) = self.input.next_u8()? {
            self.macdata.write_u8(v)?;
            self.cipherlen += 1;
            self.read += 1;
            let v = v ^ self.keystream.read_u8()?;
            Some(v)
        } else {
            None
        })
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use crate::{ChaCha20Poly1305Decrypt, ChaCha20Poly1305Encrypt};
    use irox_bits::{Bits, BitsWrapper, Error};
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
        let mut enc = ChaCha20Poly1305Encrypt::new(
            BitsWrapper::Borrowed(&mut out),
            &key,
            &nonce,
            BitsWrapper::Owned(aad.as_slice()),
        );
        enc.write(SUNSCREEN)?;
        let tag = enc.finish()?;

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
        let exp_tag = hex!("1ae10b594f09e26a7e902ecbd0600691");
        assert_eq_hex_slice!(enc.as_slice(), out.as_slice());
        assert_eq_hex_slice!(exp_tag.as_slice(), tag.as_slice());

        let mut dec = ChaCha20Poly1305Decrypt::new(
            BitsWrapper::Owned(enc.as_slice()),
            &key,
            &nonce,
            BitsWrapper::Owned(aad.as_slice()),
        );
        for (idx, v) in SUNSCREEN.iter().enumerate() {
            assert_eq!(*v, dec.read_u8()?, "idx: {idx}");
        }
        assert_eq!(Ok(None), dec.next_u8());
        assert_eq!(Ok(()), dec.finish(&exp_tag));
        Ok(())
    }

    static IETF: &[u8] = b"\
Internet-Drafts are draft documents valid for a maximum of six months and may \
be updated, replaced, or obsoleted by other documents at any time. It is \
inappropriate to use Internet-Drafts as reference material or to cite them other \
than as /\xe2\x80\x9cwork in progress./\xe2\x80\x9d";

    #[test]
    pub fn test_aead2() -> Result<(), Error> {
        let key = hex!("1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0");
        let nonce = hex!("000000000102030405060708");
        let aad = hex!("f33388860000000000004e91");
        let tag = hex!("eead9d67890cbb22392336fea1851f38");
        let cipher = hex!(
            "64a0861575861af460f062c79be643bd"
            "5e805cfd345cf389f108670ac76c8cb2"
            "4c6cfc18755d43eea09ee94e382d26b0"
            "bdb7b73c321b0100d4f03b7f355894cf"
            "332f830e710b97ce98c8a84abd0b9481"
            "14ad176e008d33bd60f982b1ff37c855"
            "9797a06ef4f0ef61c186324e2b350638"
            "3606907b6a7c02b0f9f6157b53c867e4"
            "b9166c767b804d46a59b5216cde7a4e9"
            "9040c5a40433225ee282a1b0a06c523e"
            "af4534d7f83fa1155b0047718cbc546a"
            "0d072b04b3564eea1b422273f548271a"
            "0bb2316053fa76991955ebd63159434e"
            "cebb4e466dae5a1073a6727627097a10"
            "49e617d91d361094fa68f0ff77987130"
            "305beaba2eda04df997b714d6c6f2c29"
            "a6ad5cb4022b02709b"
        );

        let mut dec = ChaCha20Poly1305Decrypt::new(
            BitsWrapper::Owned(cipher.as_slice()),
            &key,
            &nonce,
            BitsWrapper::Owned(aad.as_slice()),
        );
        for (idx, v) in IETF.iter().enumerate() {
            assert_eq!(Ok(*v), dec.read_u8(), "idx: {idx}");
        }
        assert_eq!(Ok(None), dec.next_u8());
        let calc = dec.finish(&tag);
        assert_eq!(Ok(()), calc);

        let mut buf = Vec::new();
        let mut enc = ChaCha20Poly1305Encrypt::new(
            BitsWrapper::Borrowed(&mut buf),
            &key,
            &nonce,
            BitsWrapper::Owned(aad.as_slice()),
        );
        enc.write(IETF)?;
        let calc = enc.finish()?;

        assert_eq_hex_slice!(buf.as_slice(), cipher.as_slice());
        assert_eq_hex_slice!(&tag, &calc);
        Ok(())
    }
}
