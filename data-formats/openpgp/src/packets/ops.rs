// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::packets::{PubkeyAlgorithm, SignatureSubtype};
use crate::types::HashAlgorithm;
use core::fmt::Debug;
use core::fmt::Formatter;
use irox_bits::{Bits, Error};
use irox_tools::hex::to_hex_str_upper;

#[derive(Clone, Eq, PartialEq)]
pub struct OnePassSignature {
    pub vsn: u8,
    pub sigtype: SignatureSubtype,
    pub hash_alg: HashAlgorithm,
    pub pk_alg: PubkeyAlgorithm,
    pub key_id: [u8; 8],
}
impl Debug for OnePassSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OnePassSignature")
            .field("vsn", &self.vsn)
            .field("type", &self.sigtype)
            .field("hash", &self.hash_alg)
            .field("pk_type", &self.pk_alg)
            .field("key_id", &to_hex_str_upper(&self.key_id))
            .finish()
    }
}
impl TryFrom<&[u8]> for OnePassSignature {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let vsn = value.read_u8()?;
        let sigtype: SignatureSubtype = value.read_u8()?.try_into()?;
        let hash_alg: HashAlgorithm = value.read_u8()?.try_into()?;
        let pk_alg: PubkeyAlgorithm = value.read_u8()?.try_into()?;
        let key_id = value.read_exact::<8>()?;
        Ok(OnePassSignature {
            vsn,
            sigtype,
            hash_alg,
            pk_alg,
            key_id,
        })
    }
}
