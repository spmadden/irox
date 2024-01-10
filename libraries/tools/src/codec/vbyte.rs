// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

///
///
/// ```text
///  7       0
/// |--------|
///  01111111
///
/// ```
pub fn encode_7bits(val: u8) -> [u8; 1] {
    [val & 0x7F]
}
///
///
/// ```text
///  7       0
/// |--------|
///  10000000
///
/// ```
pub fn encode_8bits(val: u8) -> [u8; 2] {
    let upper = (val & 0x80) >> 7;
    [0x80 | upper, val & 0x7F]
}

///
///
/// ```text
/// 15        7       0
/// |--------|--------|
///  00111111 10000000
///
/// ```
pub fn encode_14bits(val: u16) -> [u8; 2] {
    let b = (val & 0x7F) as u8;
    let val = val >> 7;
    let a = ((val & 0x7F) | 0x80) as u8;
    [a, b]
}

///
///
/// ```text
/// 15        7       0
/// |--------|--------|
///  22111111 10000000
///
/// ```
pub fn encode_16bits(val: u16) -> [u8; 3] {
    let c = (val & 0x7F) as u8;
    let val = val >> 7;
    let b = ((val & 0x7F) | 0x80) as u8;
    let val = val >> 7;
    let a = ((val & 0x7F) | 0x80) as u8;
    [a, b, c]
}

///
///
/// ```text
/// 24       15        7       0
/// |--------|--------|--------|
///  33322222 22111111 10000000
///
/// ```
pub fn encode_21bits(val: u16) -> [u8; 3] {
    let c = (val & 0x7F) as u8;
    let val = val >> 7;
    let b = ((val & 0x7F) | 0x80) as u8;
    let val = val >> 7;
    let a = ((val & 0x7F) | 0x80) as u8;
    [a, b, c]
}
