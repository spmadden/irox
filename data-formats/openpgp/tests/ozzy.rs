// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{Bits, Error};
use irox_openpgp::keybox::Fingerprint;
use irox_openpgp::packets::{SignatureSubtype, SignatureTarget, SignatureValidationResult};
use irox_openpgp::types::{Hash, HashAlgorithm};
use irox_openpgp::validator::SignatureValidator;
use irox_tools::hex;

#[test]
pub fn test_ozzy_armor_detatched() -> Result<(), Error> {
    let sigfile = "tests/ozzy.txt.asc";
    let datafile = "tests/ozzy.txt";
    let res =
        SignatureValidator::new_empty().verify_detached_armored_signature(sigfile, datafile)?;

    assert_eq!(res, SignatureValidationResult {
        sigtype: SignatureSubtype::Binary,
        target: SignatureTarget::Data(Hash {
            hash: hex!("aa98f4372fdc38da3e74a39beb4fc38633d87d8d6add097464ad65a9fd684200b8143e620db31bf2d7cfd489fe8c2b43fbf242cded7dd18ba39e3014178a9731").into(),
            algorithm: HashAlgorithm::SHA512,
        }),
        signer: Some(Fingerprint(hex!("90DC661D06B61505B9235173E5C0CEC91E2CE49D").into())),
        result: Ok(()),
    });
    Ok(())
}

#[test]
pub fn test_ozzy_detatched() -> Result<(), Error> {
    let sigfile = "tests/ozzy.txt.sig";
    let datafile = "tests/ozzy.txt";
    let res = SignatureValidator::new_empty().verify_detached_signature(sigfile, datafile)?;
    assert_eq!(res, SignatureValidationResult {
        sigtype: SignatureSubtype::Binary,
        target: SignatureTarget::Data(Hash {
            hash: hex!("aa98f4372fdc38da3e74a39beb4fc38633d87d8d6add097464ad65a9fd684200b8143e620db31bf2d7cfd489fe8c2b43fbf242cded7dd18ba39e3014178a9731").into(),
            algorithm: HashAlgorithm::SHA512,
        }),
        signer: Some(Fingerprint(hex!("90DC661D06B61505B9235173E5C0CEC91E2CE49D").into())),
        result: Ok(()),
    });
    Ok(())
}

#[test]
pub fn test_ozzy_armored_attached() -> Result<(), Error> {
    let sigfile = "tests/ozzy.txt.casc";
    let res = SignatureValidator::new_empty().verify_attached_armored_signature(sigfile)?;
    let mut expecteddata = Vec::new();
    let mut slice = include_bytes!("ozzy.txt").as_slice();
    while let Ok(Some(line)) = slice.read_line_str() {
        expecteddata.extend_from_slice(line.trim_end().as_bytes());
        expecteddata.extend_from_slice(b"\r\n");
    }
    expecteddata.truncate(expecteddata.len() - 2);

    println!("{res:?}");
    assert_eq!(res, SignatureValidationResult {
        sigtype: SignatureSubtype::Text,
        target: SignatureTarget::EmbeddedData(Hash {
            hash: hex!("C2BED8DE9C6404913F3FB474C2C6CE2D0B83B7039562F92A30D3DF2B47BCDCB9E9804A691475AB321C7AA69C719E29C14E5B5BFAE3673DE064CC5834E70F8292").into(),
            algorithm: HashAlgorithm::SHA512,
        }, expecteddata),
        signer: Some(Fingerprint(hex!("90DC661D06B61505B9235173E5C0CEC91E2CE49D").into())),
        result: Ok(()),
    });
    Ok(())
}
