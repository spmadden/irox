<<<<<<< HEAD
<<<<<<< HEAD
// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_fixedmath::FixedU64;

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
    println!("{c}");
    assert_eq!(3. / 32., Into::<f64>::into(c))
}
=======
=======
>>>>>>> 4f4b7a9 (snap)
// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_fixedmath::FixedU64;
use irox_tools::random::Random;

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
    println!("{c}");
    assert_eq!(3. / 32., Into::<f64>::into(c))
}

#[test]
pub fn long_add() {
    let mut rand = Random::default();

<<<<<<< HEAD
}
>>>>>>> 1e2e250 (fixed-math snap)
=======
}
>>>>>>> 4f4b7a9 (snap)
