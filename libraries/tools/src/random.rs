// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

//!
//! Pseudo-Random Number Generators (PRNGs)
//!

use core::ops::{BitXor, BitXorAssign};
use irox_bits::MutBits;

/// Default starting state/seed if the system clock fails
const DEFAULT_STATE: u64 = 0x4d595df4d0f33173u64;

/// incremental multiplier for each state
const MULTIPLIER: u64 = 6364136223846793005u64;

/// incremental incrementer for each state
const INCREMENT: u64 = 1442695040888963407u64;

const MULTIPLIER_128: u128 = 0x2360ED051FC65DA44385DF649FCCF645u128;
const INCREMENT_128: u128 = 0x5851F42D4C957F2D14057B7EF767814Fu128;

pub type Random = PcgXshRR;
///
/// Basic Random Number Generator based on the `PCG-XSH-RR`.  64 bit state, 32 bit output.
pub struct PcgXshRR {
    state: u64,
}

impl PcgXshRR {
    ///
    /// Creates a random seeded with this number.
    pub fn new_seed(seed: u64) -> Self {
        Self {
            state: seed.wrapping_mul(2).wrapping_add(1),
        }
    }
}

impl PRNG for PcgXshRR {
    ///
    /// Gets the next random [`u32`] for this random sequence
    fn next_u32(&mut self) -> u32 {
        // standard PCG-XSH-RR
        let count = (self.state >> 59) as u32;
        let mut x = self.state;
        self.state = x.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        x.bitxor_assign(x >> 18);
        let x = (x >> 27) as u32;
        x.rotate_right(count)
    }
}

///
/// `PCG-XSH-RS`, 32-bit output, 64-bit state - slightly better speed than `PCG-XSH-RR`, but with worse statistical
/// properties.
pub struct PcgXshRs {
    state: u64,
}

impl PcgXshRs {
    ///
    /// Creates a random seeded with this number.
    pub fn new_seed(seed: u64) -> Self {
        Self {
            state: seed.wrapping_mul(2).wrapping_add(1),
        }
    }
}

impl PRNG for PcgXshRs {
    fn next_u32(&mut self) -> u32 {
        let state = self.state;
        self.state = state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        let shift = 22 + (state >> 61);

        (state.bitxor(state >> 22) >> shift) as u32
    }
}

///
/// `PCG-RXS-M-XS-64`, 64-bit output, 64-bit state - Insecure, but 2nd fastest after `PCG-XSL-RR-RR`
pub struct PcgRxsMXs64 {
    state: u64,
}

impl PcgRxsMXs64 {
    ///
    /// Creates a random seeded with this number.
    pub fn new_seed(seed: u64) -> Self {
        Self {
            state: seed.wrapping_mul(2).wrapping_add(1),
        }
    }
}

impl PRNG for PcgRxsMXs64 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let state = self.state;
        self.state = state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        let word = ((state >> ((state >> 59).wrapping_add(5))) ^ state)
            .wrapping_mul(12605985483714917081u64);
        (word >> 43) ^ word
    }
}

///
/// `PCG-XSL-RR-RR`, 128-bit state, 128-bit output.  Fastest PRNG in the west.  Most insecure of them all.
pub struct PcgXslRrRr {
    state: u128,
}
impl PcgXslRrRr {
    ///
    /// Creates a random seeded with this number.
    pub fn new_seed(seed: u128) -> Self {
        Self {
            state: seed.wrapping_mul(2).wrapping_add(1),
        }
    }
}

impl PRNG for PcgXslRrRr {
    fn next_u32(&mut self) -> u32 {
        self.next_u128() as u32
    }

    fn next_u128(&mut self) -> u128 {
        let state = self.state;
        self.state = state
            .wrapping_mul(MULTIPLIER_128)
            .wrapping_add(INCREMENT_128);
        let rot1 = (state >> 122) as u32;
        let high = (state >> 64) as u64;
        let newlow = (high ^ state as u64).rotate_right(rot1);
        let newhigh = high.rotate_right((newlow & 0x3F) as u32);
        (newhigh as u128) << 64 | newlow as u128
    }
}

pub struct PcgMcgXslRr {
    state: u128,
}
impl PcgMcgXslRr {
    pub fn new_seed(seed: u128) -> Self {
        Self {
            state: seed.wrapping_mul(2).wrapping_add(1),
        }
    }
}
impl PRNG for PcgMcgXslRr {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let state = self.state.wrapping_mul(MULTIPLIER_128);
        self.state = state;
        let rot = (state >> 122) as u32;
        ((state >> 64) as u64)
            .bitxor(state as u64)
            .rotate_right(rot)
    }
}

pub trait PRNG {
    ///
    /// Gets the next random [`u32`] for this random sequence
    fn next_u32(&mut self) -> u32;
    ///
    /// Gets the next random [`u8`] for this random sequence
    fn next_u8(&mut self) -> u8 {
        self.next_u32() as u8
    }
    ///
    /// Gets the next random [`u16`] for this random sequence
    fn next_u16(&mut self) -> u16 {
        self.next_u32() as u16
    }
    ///
    /// Gets the next random [`u64`] for this random sequence
    fn next_u64(&mut self) -> u64 {
        let a: u64 = self.next_u32() as u64;
        let b: u64 = self.next_u32() as u64;
        a << 32 | b
    }
    ///
    /// Gets the next random [`u128`] for this random sequence
    fn next_u128(&mut self) -> u128 {
        let a: u128 = self.next_u64() as u128;
        let b: u128 = self.next_u64() as u128;
        a << 64 | b
    }
    ///
    /// Gets the next random [`f32`] for this random sequence
    fn next_f32(&mut self) -> f32 {
        f32::from_bits(self.next_u32())
    }
    ///
    /// Gets the next random [`f64`] for this random sequence
    fn next_f64(&mut self) -> f64 {
        f64::from_bits(self.next_u64())
    }

    fn fill(&mut self, mut data: &mut [u8]) {
        while !data.is_empty() {
            let val = self.next_u64().to_ne_bytes();
            for v in val {
                if data.write_u8(v).is_err() {
                    return;
                }
            }
        }
    }

    ///
    /// Gets a random number in the range `[0..=1]`
    fn next_unit_f64(&mut self) -> f64 {
        self.next_u64() as f64 / u64::MAX as f64
    }
    ///
    /// Gets a random number in the range `[min..=max]`
    fn next_in_range(&mut self, min: f64, max: f64) -> f64 {
        let off = (max - min) * self.next_unit_f64();
        min + off
    }
    ///
    /// Gets a random number in the range `center +/- (range/2)`
    fn next_in_distribution(&mut self, center: f64, range: f64) -> f64 {
        let off = (self.next_unit_f64() - 0.5) * range;
        center + off
    }
}

#[cfg(feature = "std")]
impl Default for Random {
    fn default() -> Self {
        let seed = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(e) => e.as_nanos() as u64,
            Err(_) => DEFAULT_STATE,
        };
        Random::new_seed(seed)
    }
}
#[cfg(not(feature = "std"))]
impl Default for Random {
    fn default() -> Self {
        Random::new_seed(DEFAULT_STATE)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    #![allow(clippy::all)]
    use crate::random::{PcgRxsMXs64, PcgXslRrRr, PRNG};

    // #[test]
    // #[ignore]
    pub fn speedtest_128() -> f64 {
        let mut rand = PcgXslRrRr::new_seed(0);
        let start = std::time::Instant::now();
        let todo = 100_000_000;
        std::hint::black_box({
            let mut _v = 0;
            for _i in 0..todo {
                _v = rand.next_u128();
                _v = rand.next_u128();
                _v = rand.next_u128();
                _v = rand.next_u128();
                _v = rand.next_u128();
                _v = rand.next_u128();
                _v = rand.next_u128();
                _v = rand.next_u128();
            }
        });
        let elapsed = start.elapsed().as_secs_f64();
        let did = todo as f64 * 128. / 1e6;
        println!("Did {} MB/s", did / elapsed);
        did
    }
    #[allow(unused)]
    pub fn speedtest_64() -> f64 {
        let mut rand = PcgRxsMXs64::new_seed(0);
        let start = std::time::Instant::now();
        let todo = 1_000_000;
        std::hint::black_box({
            let mut _v = 0;
            for _i in 0..todo {
                _v = rand.next_u64();
                _v = rand.next_u64();
                _v = rand.next_u64();
                _v = rand.next_u64();
                _v = rand.next_u64();
                _v = rand.next_u64();
                _v = rand.next_u64();
                _v = rand.next_u64();
            }
        });
        let elapsed = start.elapsed().as_secs_f64();
        let did = todo as f64 * 64. / 1e6;
        println!("Did {} MB/s", did / elapsed);
        did
    }

    #[test]
    #[ignore]
    pub fn multi_speedtest() {
        let core_ids = core_affinity::get_core_ids().unwrap_or_default();
        let start = std::time::Instant::now();
        let mut handles = core_ids
            .into_iter()
            .map(|id| {
                std::thread::spawn(move || {
                    let _val = core_affinity::set_for_current(id);
                    speedtest_128()
                })
            })
            .collect::<Vec<_>>();

        let did: f64 = handles
            .drain(..)
            .map(|v| v.join().unwrap_or_default())
            .sum();
        let elapsed = start.elapsed().as_secs_f64();
        println!("Did {} MB/s", did / elapsed);
    }
}
