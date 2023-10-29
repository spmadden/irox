// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use irox_networking::address::IPv6Address;

#[test]
pub fn ipv6_display_case1() {
    // "::"
    let addr = IPv6Address::new(&[0, 0, 0, 0, 0, 0, 0, 0]);

    assert_eq!("::", format!("{addr}"));
    assert_eq!("0000:0000:0000:0000:0000:0000:0000:0000", format!("{addr:#}"));
}

#[test]
pub fn ipv6_display_case2() {
    // "::1"
    let addr = IPv6Address::new(&[0, 0, 0, 0, 0, 0, 0, 1]);

    assert_eq!("0000:0000:0000:0000:0000:0000:0000:0001", format!("{addr:#}"));
    assert_eq!("::1", format!("{addr}"));
}

#[test]
pub fn ipv6_display_case3() {
    // "2001:db8:0:1:1:1:1:1"

    let addr = IPv6Address::new(&[0x2001,0xdb8,0,1,1,1,1,1]);
    assert_eq!("2001:db8:0:1:1:1:1:1", format!("{addr}"));
    assert_eq!("2001:0db8:0000:0001:0001:0001:0001:0001", format!("{addr:#}"));
}

#[test]
pub fn ipv6_display_case4() {
    // "2001:db8:0:0:1:0:0:1"

    let addr = IPv6Address::new(&[0x2001,0xdb8,0,0,1,0,0,1]);
    assert_eq!("2001:db8::1:0:0:1", format!("{addr}"));
    assert_eq!("2001:0db8:0000:0000:0001:0000:0000:0001", format!("{addr:#}"));
}

#[test]
pub fn ipv6_display_case5() {
    // "2001:db8::1"

    let addr = IPv6Address::new(&[0x2001,0xdb8,0,0,0,0,0,1]);
    assert_eq!("2001:db8::1", format!("{addr}"));
    assert_eq!("2001:0db8:0000:0000:0000:0000:0000:0001", format!("{addr:#}"));
}