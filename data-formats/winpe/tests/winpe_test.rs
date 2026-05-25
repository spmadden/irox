// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use irox_tools::hash::Hasher;
use irox_tools::hex;
use irox_winpe::authenticode::{
    read_certificate_data, verify_authenticode_signatures, ValidSignature,
};
use irox_winpe::{load_authenticode_hash, PEFile};
use std::fs::File;

#[test]
pub fn test_hello() {
    let mut f = File::open("tests/hello-world.exe").unwrap();

    let pe = PEFile::parse_from(&mut f).unwrap();
    println!("{pe:#?}");

    read_certificate_data(&mut f, &pe).unwrap();
}

#[test]
pub fn test_authenticode1() {
    let mut hasher = Hasher::SHA256(Default::default());

    let mut f = File::open("tests/hello-world.exe").unwrap();

    let pe = PEFile::parse_from(&mut f).unwrap();
    load_authenticode_hash(&mut f, &pe, &mut hasher).unwrap();

    let hash = hasher.finish();
    let h = irox_tools::hex::to_hex_str_upper(&hash);
    println!("{h}");

    let exp = hex!("F343CF121CCC948171ADBC05A248CB6C7CCE88874DF7A600C85934393617D898");
    let hash = hash.as_ref();
    let exp = exp.as_ref();
    assert_eq!(hash, exp);
}

#[test]
pub fn test_authenticode2() {
    let sigs = verify_authenticode_signatures("tests/hello-world.exe").unwrap();

    assert_eq!(
        vec![ValidSignature {
            signer_sha1_fingerprint: hex!("C976A0187D9778277AA0118F8873EFD9920DC389"),
            signer_sha256_fingerprint: hex!(
                "57141449D2A328A802638C3F981D5BA9C98A6645E417AE8E8D9FF1D8249D722C"
            ),
        }],
        sigs
    );
}
