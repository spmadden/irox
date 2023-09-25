// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::f64::consts::TAU;

use super::{Distribution, DistributionParams};

pub struct GaussianDistribution {
    mean_mu: f64,
    standard_deviation_sigma: f64,
    variance: f64,
}

impl GaussianDistribution {
    pub fn new(mean: f64, standard_deviation: f64) -> GaussianDistribution {
        GaussianDistribution {
            mean_mu: mean,
            standard_deviation_sigma: standard_deviation,
            variance: standard_deviation.powi(2),
        }
    }
}
pub type StandardDistribution = GaussianDistribution;

impl Default for GaussianDistribution {
    fn default() -> Self {
        GaussianDistribution {
            mean_mu: 0.0,
            standard_deviation_sigma: 1.0,
            variance: 1.0,
        }
    }
}

impl Distribution for GaussianDistribution {
    fn pdf(&self, x: f64) -> f64 {
        let a = (x - self.mean_mu) / self.standard_deviation_sigma;
        let exp = -0.5 * a.powi(2);
        exp.exp() / (self.standard_deviation_sigma * TAU.sqrt())
    }

    fn get_params(&self) -> Vec<DistributionParams> {
        vec![
            DistributionParams::Mean(self.mean_mu),
            DistributionParams::Variance(self.variance),
            DistributionParams::StandardDeviation(self.standard_deviation_sigma),
        ]
    }
}
