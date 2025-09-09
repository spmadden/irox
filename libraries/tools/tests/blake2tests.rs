// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::Error;
use irox_tools::assert_eq_hex_slice;
use irox_tools::hash::{BLAKE2b512, BLAKE2s256};
use std::io::{BufRead, BufReader};

struct Test {
    msg: Vec<u8>,
    key: Vec<u8>,
    hash: Vec<u8>,
}

fn parse_kat_file(path: &str) -> Result<Vec<Test>, Error> {
    let file = std::fs::OpenOptions::new().read(true).open(path)?;
    let file = BufReader::new(file);

    let mut tests = Vec::<Test>::new();

    let mut keyb: Option<Vec<u8>> = None;
    let mut msg: Option<Vec<u8>> = None;

    for line in file.lines() {
        let line = line?;
        let line = line.trim();

        let Some((key, val)) = line.split_once(":") else {
            continue;
        };
        let val = val.trim();
        if key.starts_with("in") {
            let mut buf = Vec::<u8>::new();
            let _res = irox_tools::hex::from_hex_into(val, &mut buf)?;
            msg = Some(buf);
        } else if key.starts_with("key") {
            let mut buf = Vec::<u8>::new();
            let res = irox_tools::hex::from_hex_into(val, &mut buf)?;
            if res > 0 {
                keyb = Some(buf);
            }
        } else if key.starts_with("hash") {
            let mut buf = Vec::<u8>::new();
            let res = irox_tools::hex::from_hex_into(val, &mut buf)?;
            if res > 0 {
                let Some(key) = keyb.take() else {
                    continue;
                };
                let Some(msg) = msg.take() else {
                    continue;
                };
                tests.push(Test {
                    msg,
                    key,
                    hash: buf,
                })
            }
        }
    }
    Ok(tests)
}
macro_rules! impl_test {
    ($name:ident, $filepath:literal, $testcount:literal, $test:ty) => {
        #[test]
        pub fn $name() -> Result<(), Error> {
            let tests = parse_kat_file($filepath)?;

            assert_eq!($testcount, tests.len());
            for (idx, test) in tests.iter().enumerate() {
                let res = <$test>::new(test.key.as_slice()).hash(test.msg.as_slice());
                assert_eq_hex_slice!(test.hash.as_slice(), res, format!("{idx}: failed"));
                println!("{} {idx}: passed", stringify!($name));
            }

            Ok(())
        }
    };
}

impl_test!(
    test_blake2b,
    "./doc/blake2testvectors/blake2b-kat.txt",
    256,
    BLAKE2b512
);
impl_test!(
    test_blake2s,
    "./doc/blake2testvectors/blake2s-kat.txt",
    256,
    BLAKE2s256
);
