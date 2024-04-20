// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! A collection of digital signals processing and statistics functions

#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;

pub use gaussian as standard;

pub mod decay;
pub mod filter;
pub mod gaussian;
pub mod streaming;
pub mod tdigest;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DistributionParams {
    Mean(f64),
    StandardDeviation(f64),
    Variance(f64),
    Other(&'static str, f64),
}

/// This trait represents a statistical distribution
pub trait Distribution {
    /// computes the probability distribution function for a particular value
    fn pdf(&self, value: f64) -> f64;

    /// Returns a set of the known parameters of this distribution.
    fn get_params(&self) -> Vec<DistributionParams>;
}
