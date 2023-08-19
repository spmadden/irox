// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::time::Duration;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HalfLife(Duration);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MeanLifetime(Duration);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DecayConstant(Duration);

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
        let ln2 = std::f64::consts::LN_2;
        let var = value.0.as_secs_f64();
        HalfLife(Duration::from_secs_f64(ln2 / var))
    }
}

impl From<MeanLifetime> for HalfLife {
    fn from(value: MeanLifetime) -> Self {
        let ln2 = std::f64::consts::LN_2;
        let var = value.0.as_secs_f64();
        HalfLife(Duration::from_secs_f64(var * ln2))
    }
}
