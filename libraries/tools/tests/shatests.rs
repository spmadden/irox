// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::Error;
use irox_tools::hash::{SHA1, SHA224, SHA256, SHA384, SHA512};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Test {
    msg: Vec<u8>,
    digest: Vec<u8>,
}

fn parse_rsp_file(path: &str) -> Result<Vec<Test>, Error> {
    let file = std::fs::OpenOptions::new().read(true).open(path)?;
    let file = BufReader::new(file);

    let mut tests = Vec::<Test>::new();

    let mut len_bytes: Option<u32> = None;
    let mut msg: Option<Vec<u8>> = None;

    for line in file.lines() {
        let line = line?;
        let line = line.trim();

        let Some((key, val)) = line.split_once(" = ") else {
            continue;
        };

        if key.starts_with("Len") {
            let Ok(v) = u32::from_str(val) else {
                continue;
            };
            len_bytes = Some(v);
        } else if key.starts_with("Msg") {
            let mut buf = Vec::<u8>::new();
            let res = irox_tools::hex::from_hex_into(val, &mut buf)?;
            if res > 0 {
                msg = Some(buf);
            }
        } else if key.starts_with("MD") {
            let mut buf = Vec::<u8>::new();
            let res = irox_tools::hex::from_hex_into(val, &mut buf)?;
            if res > 0 {
                let Some(len) = len_bytes.take() else {
                    continue;
                };
                let Some(mut msg) = msg.take() else {
                    continue;
                };
                let len = len / 8;
                msg.truncate(len as usize);
                tests.push(Test { msg, digest: buf })
            }
        }
    }
    Ok(tests)
}

macro_rules! impl_test {
    ($name:ident, $filepath:literal, $testcount:literal, $test:ty) => {
        #[test]
        pub fn $name() -> Result<(), Error> {
            let tests = parse_rsp_file($filepath)?;

            assert_eq!($testcount, tests.len());
            for (idx, test) in tests.iter().enumerate() {
                let res = <$test>::new().hash(test.msg.as_slice());
                assert_eq!(test.digest.as_slice(), res);
                println!("{} {idx}: passed", stringify!($name));
            }

            Ok(())
        }
    };
}

impl_test!(
    test_sha1_short,
    "./doc/shabytetestvectors/SHA1ShortMsg.rsp",
    65,
    SHA1
);
impl_test!(
    test_sha1_long,
    "./doc/shabytetestvectors/SHA1LongMsg.rsp",
    64,
    SHA1
);

impl_test!(
    test_sha224_short,
    "./doc/shabytetestvectors/SHA224ShortMsg.rsp",
    65,
    SHA224
);
impl_test!(
    test_sha224_long,
    "./doc/shabytetestvectors/SHA224LongMsg.rsp",
    64,
    SHA224
);
impl_test!(
    test_sha256_short,
    "./doc/shabytetestvectors/SHA256ShortMsg.rsp",
    65,
    SHA256
);
impl_test!(
    test_sha256_long,
    "./doc/shabytetestvectors/SHA256LongMsg.rsp",
    64,
    SHA256
);

impl_test!(
    test_sha384_short,
    "./doc/shabytetestvectors/SHA384ShortMsg.rsp",
    129,
    SHA384
);
impl_test!(
    test_sha384_long,
    "./doc/shabytetestvectors/SHA384LongMsg.rsp",
    128,
    SHA384
);
impl_test!(
    test_sha512_short,
    "./doc/shabytetestvectors/SHA512ShortMsg.rsp",
    129,
    SHA512
);
impl_test!(
    test_sha512_long,
    "./doc/shabytetestvectors/SHA512LongMsg.rsp",
    128,
    SHA512
);
