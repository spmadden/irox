// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::Error;
use irox_tools::assert_eq_hex_slice;
use irox_tools::hash::{HMACSHA1, HMACSHA224, HMACSHA256, HMACSHA384, HMACSHA512};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum HMAC {
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
}
impl HMAC {
    pub fn mac(&self, key: &[u8], msg: &[u8]) -> Box<[u8]> {
        match self {
            HMAC::SHA1 => Box::from(HMACSHA1::new(key).hash(msg)),
            HMAC::SHA224 => Box::from(HMACSHA224::new(key).hash(msg)),
            HMAC::SHA256 => Box::from(HMACSHA256::new(key).hash(msg)),
            HMAC::SHA384 => Box::from(HMACSHA384::new(key).hash(msg)),
            HMAC::SHA512 => Box::from(HMACSHA512::new(key).hash(msg)),
        }
    }
}
struct Test {
    key: Vec<u8>,
    msg: Vec<u8>,
    mac: Vec<u8>,
    maclen: u32,
    hmac: HMAC,
}

fn parse_rsp_file(path: &str) -> Result<Vec<Test>, Error> {
    let file = std::fs::OpenOptions::new().read(true).open(path)?;
    let file = BufReader::new(file);

    let mut tests = Vec::<Test>::new();

    let mut maclen: Option<u32> = None;
    let mut msg: Option<Vec<u8>> = None;
    let mut key_buf: Option<Vec<u8>> = None;
    let mut hmac: Option<HMAC> = None;

    for line in file.lines() {
        let line = line?;
        let line = line.trim();

        if line.starts_with("[L=") {
            hmac = Some(match line {
                "[L=20]" => HMAC::SHA1,
                "[L=28]" => HMAC::SHA224,
                "[L=32]" => HMAC::SHA256,
                "[L=48]" => HMAC::SHA384,
                "[L=64]" => HMAC::SHA512,
                _ => {
                    continue;
                }
            })
        }

        let Some((key, val)) = line.split_once(" = ") else {
            continue;
        };
        if key.starts_with("Tlen") {
            let Ok(v) = u32::from_str(val) else {
                continue;
            };
            maclen = Some(v);
        } else if key.starts_with("Msg") {
            let mut buf = Vec::<u8>::new();
            let res = irox_tools::hex::from_hex_into(val, &mut buf)?;
            if res > 0 {
                msg = Some(buf);
            }
        } else if key.starts_with("Key") {
            let mut buf = Vec::<u8>::new();
            let res = irox_tools::hex::from_hex_into(val, &mut buf)?;
            if res > 0 {
                key_buf = Some(buf);
            }
        } else if key.starts_with("Mac") {
            let mut buf = Vec::<u8>::new();
            let res = irox_tools::hex::from_hex_into(val, &mut buf)?;
            if res > 0 {
                let Some(maclen) = maclen.take() else {
                    continue;
                };
                let Some(msg) = msg.take() else {
                    continue;
                };
                let Some(key_buf) = key_buf.take() else {
                    continue;
                };
                let Some(hmac) = hmac else {
                    continue;
                };

                tests.push(Test {
                    msg,
                    key: key_buf,
                    mac: buf,
                    maclen,
                    hmac,
                });
            }
        }
    }
    Ok(tests)
}

#[test]
pub fn nist_hmac_testvectors() -> Result<(), Error> {
    let filepath = "./doc/hmactestvectors/HMAC.rsp";
    let tests = parse_rsp_file(filepath)?;
    assert_eq!(1575, tests.len());

    for (idx, test) in tests.iter().enumerate() {
        let res = test.hmac.mac(&test.key, &test.msg);
        assert_eq!(test.maclen as usize, test.mac.len());
        let res = res.split_at(test.maclen as usize).0;
        assert_eq_hex_slice!(test.mac.as_slice(), res, format!("test index {idx}"));
        println!("HMAC-{:?} {idx}: passed", test.hmac);
    }

    Ok(())
}
