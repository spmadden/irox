// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]

pub use gaussian as standard;

pub mod decay;
pub mod gaussian;

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
