// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

//!
//! Pseudo-Random Number Generators (PRNGs)
//!

use std::io::Error;
use std::ops::BitXorAssign;
use std::time::UNIX_EPOCH;

use crate::bits::Bits;

/// Default starting state/seed if the system clock fails
const DEFAULT_STATE: u64 = 0x4d595df4d0f33173u64;

/// incremental multiplier for each state
const MULTIPLIER: u64 = 6364136223846793005u64;

/// incremental incrementer for each state
const INCREMENT: u64 = 1442695040888963407u64;

///
/// Basic Random Number Generator based on the `PCG-XSH-RR`
pub struct Random {
    state: u64,
}

impl Random {
    ///
    /// Creates a random seeded with this number.
    pub fn new_seed(seed: u64) -> Random {
        Random {
            state: seed.wrapping_mul(2).wrapping_add(1),
        }
    }

    ///
    /// Gets the next random [`u32`] for this random sequence
    pub fn next_u32(&mut self) -> u32 {
        // standard PCG-XSH-RR
        let count = (self.state >> 59) as u32;
        let mut x = self.state;
        self.state = x.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        x.bitxor_assign(x >> 18);
        let x = (x >> 27) as u32;
        x.rotate_right(count)
    }

    ///
    /// Gets the next random [`u8`] for this random sequence
    pub fn next_u8(&mut self) -> u8 {
        self.next_u32() as u8
    }

    ///
    /// Gets the next random [`u16`] for this random sequence
    pub fn next_u16(&mut self) -> u16 {
        self.next_u32() as u16
    }

    ///
    /// Gets the next random [`u64`] for this random sequence
    pub fn next_u64(&mut self) -> u64 {
        let a: u64 = self.next_u32() as u64;
        let b: u64 = self.next_u32() as u64;
        a << 32 | b
    }

    ///
    /// Gets the next random [`u128`] for this random sequence
    pub fn next_u128(&mut self) -> u128 {
        let a: u128 = self.next_u64() as u128;
        let b: u128 = self.next_u64() as u128;
        a << 64 | b
    }

    ///
    /// Gets the next random [`f32`] for this random sequence
    pub fn next_f32(&mut self) -> f32 {
        f32::from_bits(self.next_u32())
    }

    ///
    /// Gets the next random [`f64`] for this random sequence
    pub fn next_f64(&mut self) -> f64 {
        f64::from_bits(self.next_u64())
    }
}

impl Bits for Random {
    fn read_u8(&mut self) -> Result<u8, Error> {
        Ok(self.next_u8())
    }

    fn read_be_u16(&mut self) -> Result<u16, Error> {
        Ok(self.next_u16())
    }

    fn read_be_u32(&mut self) -> Result<u32, Error> {
        Ok(self.next_u32())
    }

    fn read_be_u64(&mut self) -> Result<u64, Error> {
        Ok(self.next_u64())
    }

    fn read_be_u128(&mut self) -> Result<u128, Error> {
        Ok(self.next_u128())
    }

    fn read_f32(&mut self) -> Result<f32, Error> {
        Ok(self.next_f32())
    }

    fn read_f64(&mut self) -> Result<f64, Error> {
        Ok(self.next_f64())
    }

    fn read_be_i16(&mut self) -> Result<i16, Error> {
        Ok(self.next_u16() as i16)
    }

    fn read_be_i32(&mut self) -> Result<i32, Error> {
        Ok(self.next_u32() as i32)
    }

    fn read_be_i64(&mut self) -> Result<i64, Error> {
        Ok(self.next_u64() as i64)
    }

    fn advance(&mut self, len: usize) -> Result<usize, Error> {
        let whole_u32s = len / 4;
        let rem = len - whole_u32s * 4;
        for _i in 0..whole_u32s {
            self.next_u32();
        }
        for _i in 0..rem {
            self.next_u8();
        }
        Ok(len)
    }
}

impl Default for Random {
    fn default() -> Self {
        let seed = match std::time::SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(e) => e.as_nanos() as u64,
            Err(_) => DEFAULT_STATE,
        };
        Random::new_seed(seed)
    }
}
