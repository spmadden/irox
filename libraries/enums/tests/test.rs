// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_enums::{EnumIterItem, EnumName, EnumTryFromStr};

#[derive(Debug, Eq, PartialEq, EnumName, EnumIterItem, EnumTryFromStr)]
pub enum TestEnum {
    First,
    Second,
    Third,
    Fourth,
}

#[test]
pub fn test_name() {
    assert_eq!("First", TestEnum::First.name());
    assert_eq!("Second", TestEnum::Second.name());
    assert_eq!("Third", TestEnum::Third.name());
    assert_eq!("Fourth", TestEnum::Fourth.name());
}
#[test]
pub fn test_iter_names() {
    let mut names = TestEnum::iter_names();
    assert_eq!("First", names.next().unwrap());
    assert_eq!("Second", names.next().unwrap());
    assert_eq!("Third", names.next().unwrap());
    assert_eq!("Fourth", names.next().unwrap());
    assert_eq!(None, names.next());
}

#[test]
pub fn test_iter_items() {
    let mut items = TestEnum::iter_items();
    assert_eq!(items.next().unwrap(), TestEnum::First);
    assert_eq!(items.next().unwrap(), TestEnum::Second);
    assert_eq!(items.next().unwrap(), TestEnum::Third);
    assert_eq!(items.next().unwrap(), TestEnum::Fourth);
}

#[test]
pub fn try_fromstr() {
    assert_eq!(TestEnum::try_from("First").unwrap(), TestEnum::First);
    assert_eq!(TestEnum::try_from("Second").unwrap(), TestEnum::Second);
    assert_eq!(TestEnum::try_from("Third").unwrap(), TestEnum::Third);
    assert_eq!(TestEnum::try_from("Fourth").unwrap(), TestEnum::Fourth);
    assert_eq!(TestEnum::try_from("garbage"), Err(()))
}

#[derive(Debug, EnumName)]
pub enum TestEnum2 {
    First(u8, u8),
    Second,
}

#[test]
pub fn test_complex() {
    assert_eq!("First", TestEnum2::First(0, 0).name());
    assert_eq!("Second", TestEnum2::Second.name());

    let mut names = TestEnum2::iter_names();
    assert_eq!("First", names.next().unwrap());
    assert_eq!("Second", names.next().unwrap());
}
