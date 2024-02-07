// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_tools::bits::Bits;
use irox_tools::bits::Error;

#[test]
pub fn bits_test1() -> Result<(), Error> {
    let mut buf = [
        0x0, 0x01_u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
    ]
    .as_slice();
    assert_eq!(0x0, buf.read_u8()?);
    assert_eq!(0x1, buf.read_u8()?);
    assert_eq!(0x2, buf.read_u8()?);
    assert_eq!(0x3, buf.read_u8()?);
    assert_eq!(0x4, buf.read_u8()?);
    assert_eq!(0x5, buf.read_u8()?);
    assert_eq!(0x6, buf.read_u8()?);
    assert_eq!(0x7, buf.read_u8()?);
    assert_eq!(0x8, buf.read_u8()?);
    assert_eq!(0x9, buf.read_u8()?);
    assert_eq!(0xA, buf.read_u8()?);
    assert_eq!(0xB, buf.read_u8()?);
    assert_eq!(0xC, buf.read_u8()?);
    assert_eq!(0xD, buf.read_u8()?);
    assert_eq!(0xE, buf.read_u8()?);
    assert_eq!(0xF, buf.read_u8()?);
    Ok(())
}

#[test]
pub fn bits_test2() -> Result<(), Error> {
    let mut buf = [
        0x0, 0x01_u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
    ]
    .as_slice();
    assert_eq!(0x0001, buf.read_be_u16()?);
    assert_eq!(0x0203, buf.read_be_u16()?);
    assert_eq!(0x0405, buf.read_be_u16()?);
    assert_eq!(0x0607, buf.read_be_u16()?);
    assert_eq!(0x0809, buf.read_be_u16()?);
    assert_eq!(0x0A0B, buf.read_be_u16()?);
    assert_eq!(0x0C0D, buf.read_be_u16()?);
    assert_eq!(0x0E0F, buf.read_be_u16()?);
    Ok(())
}

#[test]
pub fn bits_test3() -> Result<(), Error> {
    let mut buf = [
        0x0, 0x01_u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
    ]
    .as_slice();
    assert_eq!(0x00010203, buf.read_be_u32()?);
    assert_eq!(0x04050607, buf.read_be_u32()?);
    assert_eq!(0x08090A0B, buf.read_be_u32()?);
    assert_eq!(0x0C0D0E0F, buf.read_be_u32()?);
    Ok(())
}

#[test]
pub fn bits_test4() -> Result<(), Error> {
    let mut buf = [
        0x0, 0x01_u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
    ]
    .as_slice();
    assert_eq!(0x0001020304050607, buf.read_be_u64()?);
    assert_eq!(0x08090A0B0C0D0E0F, buf.read_be_u64()?);
    Ok(())
}

#[test]
pub fn bits_test5() -> Result<(), Error> {
    let mut buf = [
        0x0, 0x01_u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
    ]
    .as_slice();
    assert_eq!(0x0102030405060708090A0B0C0D0E0F, buf.read_be_u128()?);
    Ok(())
}
