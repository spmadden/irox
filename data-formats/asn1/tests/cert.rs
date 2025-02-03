// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use std::io::BufReader;
use irox_asn1::{Asn1Object, DecodeDER};
use irox_bits::{BitsError, BitsWrapper};

#[test]
pub fn testcert() -> Result<(), BitsError> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open("testdata/msivroot.cer").unwrap();
    let file = BufReader::new(file);
    let mut file = BitsWrapper::Owned(file);
    
    
    let res = Asn1Object::decode_der(&mut file)?;
    println!("{res:#?}");
    Ok(())
}