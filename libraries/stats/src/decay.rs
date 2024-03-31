// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Exponential Half-lifes and Decays

use core::f64::consts::LN_2;
use core::time::Duration;
#[allow(unused_imports)]
use irox_tools::f64::FloatExt;

///
/// The half-life is defined as the duration required for a particular quantity to drop to 50% of the
/// original value.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HalfLife(Duration);

///
/// Average length of time that a particular item remains in the set.  (Tau)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MeanLifetime(Duration);

///
/// Exponential Decay Rate Constant (1/Tau)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DecayConstant(Duration);

impl From<Duration> for HalfLife {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}
impl From<Duration> for DecayConstant {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}
impl From<Duration> for MeanLifetime {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

impl From<MeanLifetime> for DecayConstant {
    fn from(value: MeanLifetime) -> Self {
        let var = 1.0 / value.0.as_secs_f64();
        DecayConstant(Duration::from_secs_f64(var))
    }
}

impl From<DecayConstant> for MeanLifetime {
    fn from(value: DecayConstant) -> Self {
        let var = 1.0 / value.0.as_secs_f64();
        MeanLifetime(Duration::from_secs_f64(var))
    }
}

impl From<DecayConstant> for HalfLife {
    fn from(value: DecayConstant) -> Self {
        let var = value.0.as_secs_f64();
        HalfLife(Duration::from_secs_f64(LN_2 / var))
    }
}

impl From<MeanLifetime> for HalfLife {
    fn from(value: MeanLifetime) -> Self {
        let var = value.0.as_secs_f64();
        HalfLife(Duration::from_secs_f64(var * LN_2))
    }
}

impl MeanLifetime {
    pub fn decay_factor_at(&self, time: &Duration) -> f64 {
        (-time.as_secs_f64() / self.0.as_secs_f64()).exp()
    }
}

impl HalfLife {
    pub fn decay_factor_at(&self, time: &Duration) -> f64 {
        2.0_f64.powf(-time.as_secs_f64() / self.0.as_secs_f64())
    }
}
