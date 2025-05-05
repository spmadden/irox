// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::armor::Dearmor;
use crate::keybox::{Keybox, MultiKeybox};
use crate::packets::{OpenPGPMessage, OpenPGPPacketData};
use crate::types::HashAlgorithm;
use irox_bits::{BitsErrorKind, Error};
use irox_tools::hash::Hasher;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::Path;

pub struct SignatureValidator<'a> {
    pub keybox: MultiKeybox<'a>,
}
impl<'a> SignatureValidator<'a> {
    pub fn new_empty() -> Self {
        SignatureValidator {
            keybox: MultiKeybox::Owned(Keybox::default()),
        }
    }
    pub fn new_keybox(keybox: MultiKeybox<'a>) -> Self {
        Self { keybox }
    }
    pub fn verify_detached_armored_signature<S: AsRef<Path>, D: AsRef<Path>>(
        &'a mut self,
        sigfile: S,
        datafile: D,
    ) -> Result<(), Error> {
        let file = OpenOptions::new().read(true).create(false).open(sigfile)?;
        let mut file = BufReader::new(file);
        let mut file = file.dearmor();
        let sig = OpenPGPMessage::build_from(&mut file)?;
        if let Some(r) = self.keybox.map_mut(|v| v.add_to_keybox(&sig)) {
            r?;
        }

        let Some(sig) = sig.packets.iter().find_map(|p| {
            if let OpenPGPPacketData::Signature(sig) = &p.data {
                Some(sig)
            } else {
                None
            }
        }) else {
            return Err(Error::new(
                BitsErrorKind::NotFound,
                "No signature packet found",
            ));
        };
        let mut hasher: Hasher = sig.get_hash_alg().try_into()?;
        hasher.hash_file(datafile)?;

        sig.validate_signature(&self.keybox, hasher)
    }

    pub fn verify_detached_signature<S: AsRef<Path>, D: AsRef<Path>>(
        &mut self,
        sigfile: S,
        datafile: D,
    ) -> Result<(), Error> {
        let file = OpenOptions::new().read(true).create(false).open(sigfile)?;
        let mut file = BufReader::new(file);
        let sig = OpenPGPMessage::build_from(&mut file)?;
        if let Some(r) = self.keybox.map_mut(|v| v.add_to_keybox(&sig)) {
            r?;
        }

        let Some(sig) = sig.packets.iter().find_map(|p| {
            if let OpenPGPPacketData::Signature(sig) = &p.data {
                Some(sig)
            } else {
                None
            }
        }) else {
            return Err(Error::new(
                BitsErrorKind::NotFound,
                "No signature packet found",
            ));
        };
        let mut hasher: Hasher = sig.get_hash_alg().try_into()?;
        hasher.hash_file(datafile)?;

        sig.validate_signature(&self.keybox, hasher)
    }
}

pub fn s2k(alg: HashAlgorithm, iter: usize, salt: &[u8], data: &[u8]) -> Result<Box<[u8]>, Error> {
    let mut h: Hasher = alg.try_into()?;
    let mut rem = iter;
    while rem > 0 {
        let l = rem.min(salt.len());
        let Some(s) = salt.get(0..l) else {
            break;
        };
        h.write(s);
        rem -= l;
        let l = rem.min(data.len());
        let Some(d) = data.get(0..l) else {
            break;
        };
        h.write(d);
        rem -= l;
    }
    Ok(h.finish())
}

#[cfg(test)]
mod tests {
    use crate::types::HashAlgorithm;
    use crate::validator::s2k;
    use irox_bits::Error;
    use irox_tools::hash::SHA256;
    use irox_tools::{assert_eq_hex_slice, hex};

    #[test]
    pub fn test_s2k_1() {
        let s = hex!("3031323334353637313233343536");
        let c = 0x000186A0;
        let mut rem = c;
        let mut h = SHA256::new();
        while rem > 0 {
            let l = rem.min(s.len());
            h.write(&s[0..l]);
            rem -= l;
        }
        let h = h.finish();
        assert_eq_hex_slice!(
            h,
            hex!("773784A602B6C81E3F092F4D7D00E17CC822D88F7360FCF2D2EF2D9D901F44B6")
        );
    }

    #[test]
    pub fn test_s2k_2() -> Result<(), Error> {
        let s = b"01234567";
        let d = b"123456";
        let c = 0x000186A0;
        let h = s2k(HashAlgorithm::SHA256, c, s, d)?;
        assert_eq_hex_slice!(
            h,
            hex!("773784A602B6C81E3F092F4D7D00E17CC822D88F7360FCF2D2EF2D9D901F44B6")
        );
        Ok(())
    }
    #[test]
    pub fn test_s2k_3() -> Result<(), Error> {
        let s = hex!("4142434445464748");
        let d = b"12345678";
        let c = 0x000186A0;
        let h = s2k(HashAlgorithm::SHA256, c, &s, d)?;
        assert_eq_hex_slice!(
            h,
            hex!("2675D6164A0D4827D1D00C7EEA620D015C00030A1CAB38B4D0DD600B27DC9630")
        );
        Ok(())
    }
}
