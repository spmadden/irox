// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_fixedmath::FloatExt;
use irox_fixedmath::{FixedU128, FixedU64};
use irox_tools::assert_eq_eps;

#[test]
pub fn add() {
    let a: FixedU64 = 1.into();
    let b: FixedU64 = 32.into();

    let c = a + b;
    assert_eq!(33, Into::<u64>::into(c))
}

#[test]
pub fn sub() {
    let a: FixedU64 = 1.into();
    let b: FixedU64 = 32.into();

    let c = b - a;
    assert_eq!(31, Into::<u64>::into(c))
}

#[test]
pub fn mul() {
    let a: FixedU64 = 3.into();
    let b: FixedU64 = 32.into();

    let c = a * b;
    assert_eq!(96, Into::<u64>::into(c))
}

#[test]
pub fn div() {
    let a: FixedU64 = 3.into();
    let b: FixedU64 = 32.into();

    let c = a / b;
    assert_eq!(3. / 32., Into::<f64>::into(c));
}

#[test]
pub fn long_add() {
    // let mut rand = Random::default();
}
#[test]
pub fn test_resolution() {
    assert_eq_eps!(1. / u32::MAX as f64, FixedU64::RESOLUTION.as_f64(), 1e-16);
    assert_eq_eps!(1. / u64::MAX as f64, FixedU128::RESOLUTION.as_f64(), 1e-16);
    let res = FixedU128::RESOLUTION.as_f64();
    let minf64 = 1. / u64::MAX as f64;
    let diff = res - minf64;
    println!("{diff} = {res} - {minf64}");

    assert_eq_eps!(
        0.5f64,
        FixedU64::ONE_HALF.as_f64(),
        FixedU64::RESOLUTION.as_f64()
    );
}
#[test]
pub fn test_e() {
    assert_eq_eps!(
        core::f64::consts::E,
        FixedU64::E.as_f64(),
        FixedU64::RESOLUTION.as_f64()
    );
    assert_eq_eps!(
        core::f64::consts::E,
        FixedU128::E.as_f64(),
        FixedU128::RESOLUTION.as_f64()
    );
}
#[test]
pub fn test_pi() {
    assert_eq_eps!(
        core::f64::consts::PI,
        FixedU64::PI.as_f64(),
        FixedU64::RESOLUTION.as_f64()
    );
}
#[test]
pub fn test_ln() {
    let z = FixedU64::E;
    let a = z - 1u8;
    assert_eq_eps!(
        core::f64::consts::E - 1.0,
        a.as_f64(),
        FixedU64::RESOLUTION.as_f64()
    );
    let b = z + 1u8;
    assert_eq_eps!(
        core::f64::consts::E + 1.0,
        b.as_f64(),
        FixedU64::RESOLUTION.as_f64()
    );
    let d = a / b;
    assert_eq!(FixedU64::from_parts(0, 1984778077), d);
    let e = d * 2u16;
    assert_eq!(FixedU64::from_parts(0, 3969556154), e);
    assert_eq_eps!(
        0.924234314520019f64,
        e.as_f64(),
        FixedU64::RESOLUTION.as_f64()
    );

    use crate::FloatExt;
    assert_eq_eps!(
        FixedU64::default(),
        FixedU64::from_parts(1, 0).ln(),
        FixedU64::from(1e-16)
    );
    // assert_eq_eps!(
    //     FixedU128::default(),
    //     FixedU128::from_parts(1, 0).ln(),
    //     FixedU128::from(1e-16)
    // );
    // this LN impl is only accurate to 7 parts :<
    assert_eq!(FixedU64::from_parts(0, u32::MAX - 6), FixedU64::E.ln());
    // println!("{:X} {:X} {}", FixedU128::E.whole(), FixedU128::E.fract(), FloatExt::fract(FixedU128::E));
    // assert_eq!(FixedU128::from_parts(1, 0), FixedU128::E.ln());
    assert_eq_eps!(
        4.605170185988092,
        FixedU64::from_parts(100, 0).ln().as_f64(),
        1e-7
    );
    // assert_eq_eps!(
    //     4.605170185988092,
    //     FixedU128::from_parts(100, 0).ln().as_f64(),
    //     1e-15
    // );
    assert_eq_eps!(
        11.090339630053647,
        FixedU64::from_parts(u16::MAX as u32, 0).ln().as_f64(),
        1e-4
    );
}

#[test]
pub fn test_exp() {
    assert_eq_eps!(FixedU64::from_parts(1, 0), FixedU64::default().exp(), 1e-16);
    assert_eq_eps!(FixedU64::E, FixedU64::from(1).exp(), 1e-15);
    assert_eq_eps!(
        FixedU64::from_parts(7, 1670983221),
        FixedU64::from(2).exp(),
        1e-14
    );
    assert_eq_eps!(FixedU64::from_parts(15, 662551282), FixedU64::E.exp(), 1e-8);
}

#[test]
pub fn test_sqrt() {
    assert_eq_eps!(
        FixedU64::from_parts(2, 0),
        FixedU64::from_parts(4, 0).sqrt(),
        1e-8
    );
}
