// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use core::ops::{Add, AddAssign};
use irox_bits::FromLEBytes;

const _P1305: Uint1305 = Uint1305 {
    a: 0x03,
    b: 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFAu128,
};

#[derive(Default, Debug, Copy, Clone)]
struct Uint1305 {
    a: u128,
    b: u128,
}
impl From<u128> for Uint1305 {
    fn from(value: u128) -> Self {
        Self { a: 0, b: value }
    }
}
impl Add for Uint1305 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let (b, ov) = self.b.overflowing_add(other.b);
        let ov = if ov { 1 } else { 0 };
        let a = self.a + other.a + ov;
        Self { a, b }
    }
}

impl Add<Uint1305> for &mut Uint1305 {
    type Output = Uint1305;

    fn add(self, other: Uint1305) -> Self::Output {
        let (b, ov) = self.b.overflowing_add(other.b);
        let ov = if ov { 1 } else { 0 };
        let a = self.a + other.a + ov;
        Uint1305 { a, b }
    }
}

impl AddAssign for Uint1305 {
    fn add_assign(&mut self, other: Self) {
        let (b, ov) = self.b.overflowing_add(other.b);
        self.b = b;
        let ov = if ov { 1 } else { 0 };
        self.a += other.a + ov;
    }
}

pub struct Poly1305;

impl Poly1305 {
    pub fn hash(_msg: &[u8], key: [u8; 32]) -> [u8; 16] {
        let [_r, _s] = <[u128; 2]>::from_le_bytes(key);

        todo!()
    }
}
