// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::time::Duration;

use irox_stats::decay::HalfLife;
use irox_stats::filter::StreamingFilter;
use irox_stats::gaussian::StandardDistribution;
use irox_stats::Distribution;

pub struct Run {
    run_duration: Duration,
    half_life: HalfLife,
    normal_peak_at: Duration,
}

impl Run {
    pub fn new(run_duration: Duration, half_life: HalfLife, normal_peak_at: Duration) -> Run {
        Run {
            normal_peak_at,
            run_duration,
            half_life,
        }
    }

    pub fn run_data(&self, initial_quantity: f64, start_offset: Duration) -> Vec<f64> {
        let kernel = self.generate_normal(initial_quantity, &start_offset);
        let mut filter = StreamingFilter::new(kernel);
        let mut data: Vec<f64> = (0..=self.run_duration.as_secs())
            .filter_map(|t| {
                let factor = self.half_life.decay_factor_at(&Duration::from_secs(t));
                filter.add_and_convolve(factor)
            })
            .collect();
        data.extend(filter.drain());

        data
    }

    pub fn generate_normal(&self, initial_quantity: f64, start_offset: &Duration) -> Vec<f64> {
        let stddev = self.normal_peak_at.as_secs_f64() / 6.;
        let mean = start_offset.as_secs_f64() + self.normal_peak_at.as_secs_f64() / 2.;
        let duration = start_offset.as_secs() + self.normal_peak_at.as_secs();
        let std = StandardDistribution::new(mean, stddev);
        (0..=duration)
            .map(|f| std.pdf(f as f64 - 0.5) * initial_quantity)
            .collect()
    }
}
