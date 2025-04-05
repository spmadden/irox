// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{BitsWrapper, Error, MutBits};
use irox_openpgp::packets::{OpenPGPMessage, OpenPGPPackeStream, OpenPGPPacketData};
use irox_tools::hash::SHA1;
use irox_tools::hex;
use irox_tools::hex::HexDump;
use std::io::Write;

static _PUBKEY_A: &str = "-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEZ33ekRYJKwYBBAHaRw8BAQdAZOxGlC92+TxBy2cZHzcR65FWFvJj+bU05CZv
RkTfd120LFNlYW4gUC4gTWFkZGVuIChZSzVOTykgPHNlYW5Ac2Vhbm1hZGRlbi5u
ZXQ+iPsEExYKAKMCGwMFCQlmAYAFCwkIBwICIgIGFQoJCAsCBBYCAwECHgcCF4AW
IQS+drHUARw2ry6CpPN60L2jYSP5cwUCZ+yWomEYaHR0cHM6Ly9rZXlzZXJ2ZXIu
dWJ1bnR1LmNvbS9wa3MvbG9va3VwP29wPWdldCZzZWFyY2g9MHhiZTc2YjFkNDAx
MWMzNmFmMmU4MmE0ZjM3YWQwYmRhMzYxMjNmOTczAAoJEHrQvaNhI/lzWlUBANmc
d5x49tZdSzmEZ+1vsk4hKp3ggsc/bHbTH3v0yqr5AQDJ0a1CzlWaB3Sk3GQuld+F
cq3e3KWAqU2WDf1r1bIeAoh5BBAWCgAhFiEEm/Wdu/h7bqAD0bAIPHM0izmSu7kF
Amd95HIDBQJ4AAoJEDxzNIs5kru5AxkA/ivLWx8ZEJHSv5NhuaoCGVbOzOyCwFLm
R4R8nDV/b119AP9ICmMHAMdLrGyIiOEXwEHGI831MmjnF2cFDToAQ6T7D7gzBGd9
3pEWCSsGAQQB2kcPAQEHQHk54ZL3M7x+K18DEN4XRMZP6wDd9B5SB5s0AZ8Y0Pzp
iH4EGBYKACYWIQS+drHUARw2ry6CpPN60L2jYSP5cwUCZ33ekQIbIAUJCWYBgAAK
CRB60L2jYSP5czMMAQC6dt9NY/B4S5TCM7Df0+h6V2WuufuIAuoPj2pNokYYRwEA
5X3AJlOG5PGtg9GGanU5q7lRiis/mB0w/sVDnnObyw24OARnfd6REgorBgEEAZdV
AQUBAQdAPGb7DdiC2VEXXNdMXkbwXsriOe2fQ/XfDfaV/cx+XUEDAQgHiH4EGBYK
ACYWIQS+drHUARw2ry6CpPN60L2jYSP5cwUCZ33ekQIbDAUJCWYBgAAKCRB60L2j
YSP5cyQCAPkBVQFqnMHxUmEoKxhaeQ/pJWIRLxIjAYNQwhbWVtiQkgEA9lCR8OMt
g4shcSb4NcU324+Jsv4dJNznS6hP8AgWCgM=
=cRQ8
-----END PGP PUBLIC KEY BLOCK-----";

static PUBKEY_B: &[u8] = &hex!(
    "983304677dde9116092b06010401da470f0101074064ec46942f76f93c41cb67"
    "191f3711eb915616f263f9b534e4266f4644df775db42c5365616e20502e204d"
    "616464656e2028594b354e4f29203c7365616e407365616e6d616464656e2e6e"
    "65743e88fb0413160a00a3021b03050909660180050b0908070202220206150a"
    "09080b020416020301021e07021780162104be76b1d4011c36af2e82a4f37ad0"
    "bda36123f973050267ec96a2611868747470733a2f2f6b65797365727665722e"
    "7562756e74752e636f6d2f706b732f6c6f6f6b75703f6f703d67657426736561"
    "7263683d30786265373662316434303131633336616632653832613466333761"
    "6430626461333631323366393733000a09107ad0bda36123f9735a550100d99c"
    "779c78f6d65d4b398467ed6fb24e212a9de082c73f6c76d31f7bf4caaaf90100"
    "c9d1ad42ce559a0774a4dc642e95df8572addedca580a94d960dfd6bd5b21e02"
    "88790410160a00211621049bf59dbbf87b6ea003d1b0083c73348b3992bbb905"
    "02677de47203050278000a09103c73348b3992bbb9031900fe2bcb5b1f191091"
    "d2bf9361b9aa021956ceccec82c052e647847c9c357f6f5d7d00ff480a630700"
    "c74bac6c8888e117c041c623cdf53268e71767050d3a0043a4fb0fb83304677d"
    "de9116092b06010401da470f010107407939e192f733bc7e2b5f0310de1744c6"
    "4feb00ddf41e52079b34019f18d0fce9887e0418160a0026162104be76b1d401"
    "1c36af2e82a4f37ad0bda36123f9730502677dde91021b20050909660180000a"
    "09107ad0bda36123f973330c0100ba76df4d63f0784b94c233b0dfd3e87a5765"
    "aeb9fb8802ea0f8f6a4da24618470100e57dc0265386e4f1ad83d1866a7539ab"
    "b9518a2b3f981d30fec5439e739bcb0db83804677dde91120a2b060104019755"
    "0105010107403c66fb0dd882d951175cd74c5e46f05ecae239ed9f43f5df0df6"
    "95fdcc7e5d4103010807887e0418160a0026162104be76b1d4011c36af2e82a4"
    "f37ad0bda36123f9730502677dde91021b0c050909660180000a09107ad0bda3"
    "6123f973240200f90155016a9cc1f15261282b185a790fe92562112f12230183"
    "50c216d656d890920100f65091f0e32d838b217126f835c537db8f89b2fe1d24"
    "dce74ba84ff008160a03");

#[test]
pub fn test_read() -> Result<(), Error> {
    let mut key = PUBKEY_B;
    let mut out = std::io::stdout();
    let out = &mut BitsWrapper::Borrowed(&mut out);
    let mut stream = OpenPGPPackeStream::new(BitsWrapper::Borrowed(&mut key));
    while let Some((ty, len, pkt)) = stream.read_next_packet()? {
        writeln!(out, "TYPE: {ty}, LEN: {:02X}", len)?;
        pkt.hexdump_to(out)?;
        if ty == 14 || ty == 6 {
            let mut hash = SHA1::new();
            hash.write_u8(0x99)?;
            hash.write_be_u16(len as u16)?;
            let fp = hash.hash(&pkt);
            fp.as_slice().hexdump_to(out)?;
        }
    }
    Ok(())
}

#[test]
pub fn test_packets() -> Result<(), Error> {
    let mut key = PUBKEY_B;
    let msg = OpenPGPMessage::build_from(&mut key)?;
    for pkt in msg.packets {
        println!("{pkt:?}");
        match pkt.data {
            OpenPGPPacketData::PublicKey(pk) => {
                println!("  PUBKEY: {pk:#?}");
            }
            OpenPGPPacketData::PublicSubkey(psk) => {
                println!("  PUBSUBKEY: {psk:#?}");
            }
            OpenPGPPacketData::Signature(sig) => {
                println!("  SIG: {sig:#?}");
            }
            _ => {}
        }
    }
    Ok(())
}
