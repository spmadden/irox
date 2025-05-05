// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::Error;
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
