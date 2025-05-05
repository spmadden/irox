// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::Error;
use irox_openpgp::validator::SignatureValidator;

#[test]
pub fn test_ozzy_armor_detatched() -> Result<(), Error> {
    let sigfile = "tests/ozzy.txt.asc";
    let datafile = "tests/ozzy.txt";
    SignatureValidator::new_empty().verify_detached_armored_signature(sigfile, datafile)
}

#[test]
pub fn test_ozzy_detatched() -> Result<(), Error> {
    let sigfile = "tests/ozzy.txt.sig";
    let datafile = "tests/ozzy.txt";
    SignatureValidator::new_empty().verify_detached_signature(sigfile, datafile)
}
