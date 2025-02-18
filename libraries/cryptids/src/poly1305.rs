// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::indexing_slicing)]

use core::ops::{Add, AddAssign};
use core::ops::{BitAnd, BitAndAssign, Index, IndexMut};
use irox_bits::{Bits, BitsError, FromLEBytes, MutBits};
use irox_tools::iterators::Zipping;

const _P1305: U136 = U136 {
    v: [
        0xFA, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0x03,
    ],
};

const MASK: U136 = U136 {
    v: [
        0xFF, 0xFF, 0xFF, 0x0F, 0xFC, 0xFF, 0xFF, 0x0F, 0xFC, 0xFF, 0xFF, 0x0F, 0xFC, 0xFF, 0xFF,
        0x0F, 0xFF,
    ],
};
const EMPTY: U136 = U136 { v: [0; 17] };
const fn one() -> U136 {
    let mut out = EMPTY;
    out.v[0] = 1;
    out
}
const ONE: U136 = one();

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
struct U136 {
    v: [u8; 17],
}
impl U136 {
    const fn _from_array(v: &[u8; 17]) -> Self {
        Self { v: *v }
    }

    fn read_from_bits<T: Bits>(input: &mut T) -> Result<Self, BitsError> {
        let mut out = Self::default();
        for v in &mut out.v {
            if let Some(a) = input.next_u8()? {
                *v = a;
            } else {
                *v = 1;
                break;
            }
        }
        Ok(out)
    }
}
impl From<u128> for U136 {
    fn from(value: u128) -> Self {
        let mut out = U136::default();
        let _ = out.v.as_mut_slice().write_le_u128(value);
        out
    }
}
impl From<[u32; 17]> for U136 {
    fn from(h: [u32; 17]) -> Self {
        let mut out = U136::default();
        let mut working = 0u32;
        for (o, a) in out.v.iter_mut().zip(h.iter()) {
            working += *a;
            *o = working as u8;
            working >>= 8;
        }
        todo!();
    }
}
impl Add for U136 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        (&self).add(other)
    }
}

impl Add<U136> for &U136 {
    type Output = U136;

    fn add(self, other: U136) -> Self::Output {
        let mut working = 0u32;
        let mut out = [0u8; 17];
        for (o, [a, b]) in out
            .iter_mut()
            .zip(Zipping::from([self.v.iter(), other.v.iter()]))
        {
            working += a as u32;
            working += b as u32;
            *o = working as u8;
            working >>= 8;
        }

        Self::Output { v: out }
    }
}

impl AddAssign for U136 {
    fn add_assign(&mut self, other: Self) {
        let mut working = 0u32;
        for (out, a) in self.v.iter_mut().zip(other.v.iter()) {
            working += *out as u32;
            working += *a as u32;
            *out = working as u8;
            working >>= 8;
        }
    }
}
impl BitAnd for U136 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        let mut out = [0u8; 17];
        for (o, [a, b]) in out
            .iter_mut()
            .zip(Zipping::from([self.v.iter(), other.v.iter()]))
        {
            *o = a & b;
        }
        Self::Output { v: out }
    }
}
impl BitAndAssign for U136 {
    fn bitand_assign(&mut self, other: Self) {
        for (out, a) in self.v.iter_mut().zip(other.v.iter()) {
            *out &= *a;
        }
    }
}
impl Index<usize> for U136 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}
impl IndexMut<usize> for U136 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

pub struct Poly1305;

impl Poly1305 {
    pub fn hash<T: Bits>(msg: &mut T, key: [u8; 32]) -> [u8; 16] {
        let [r, _s] = <[u128; 2]>::from_le_bytes(key);
        let mut r = U136::from(r);
        r.bitand_assign(MASK);

        let mut h = EMPTY;

        loop {
            let c = U136::read_from_bits(msg).unwrap_or_default();
            if c == EMPTY || c == ONE {
                break;
            }
            h.add_assign(c);
            for i in 0..17 {
                let mut x = [0u32; 17];
                for j in 0..17 {
                    let mult = if j <= i {
                        r[i - j] as u32
                    } else {
                        r[i + 17 - j] as u32 * 320
                    };
                    x[i] += mult * h[j] as u32;
                }
            }
        }

        todo!()
    }
}
