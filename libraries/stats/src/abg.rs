// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::sampling::Sample;
use irox_time::epoch::Timestamp;

#[derive(Debug, Copy, Clone)]
pub struct ABStep<T: Copy> {
    pub time: Timestamp<T>,
    pub predicted_value: f64,
    pub estimated_value: f64,
    pub estimated_velocity: f64,
    pub residual_error: f64,
}

pub struct AlphaBetaFilter<T: Copy> {
    alpha_coefficient: f64,
    beta_coefficient: f64,

    last_step: Option<ABStep<T>>,
}

impl<T: Copy> AlphaBetaFilter<T> {
    pub fn add_sample(&mut self, sample: Sample<T>) -> Option<ABStep<T>> {
        self.insert(sample.time, sample.value)
    }
    pub fn insert(&mut self, time: Timestamp<T>, value: f64) -> Option<ABStep<T>> {
        let Some(last_step) = &self.last_step else {
            self.last_step = Some(ABStep {
                time,
                predicted_value: value,
                estimated_value: value,
                estimated_velocity: 0.0,
                residual_error: 0.0,
            });
            return None;
        };
        let dt = (time - last_step.time).as_seconds_f64();

        // predict step, predict the current (measured) value
        let predicted_value = last_step.estimated_value + last_step.estimated_velocity * dt;

        // update step, calculate residual error of the prediction
        let residual_error = value - predicted_value;

        // update step, update estimates for next iteration
        let estimated_value = predicted_value + self.alpha_coefficient * residual_error;
        let estimated_velocity =
            last_step.estimated_velocity + self.beta_coefficient * (residual_error / dt);

        let step = ABStep {
            time,
            predicted_value,
            estimated_velocity,
            estimated_value,
            residual_error,
        };
        self.last_step = Some(step);
        self.last_step
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ABGStep<T: Copy> {
    pub time: Timestamp<T>,
    pub predicted_value: f64,
    pub predicted_velocity: f64,

    pub estimated_value: f64,
    pub estimated_velocity: f64,
    pub estimated_acceleration: f64,

    pub residual_error: f64,
}

pub struct AlphaBetaGammaFilter<T: Copy> {
    alpha_coefficient: f64,
    beta_coefficient: f64,
    gamma_coefficient: f64,

    last_step: Option<ABGStep<T>>,
}

impl<T: Copy> AlphaBetaGammaFilter<T> {
    pub fn add_sample(&mut self, sample: Sample<T>) -> Option<ABGStep<T>> {
        self.insert(sample.time, sample.value)
    }
    pub fn insert(&mut self, time: Timestamp<T>, value: f64) -> Option<ABGStep<T>> {
        let Some(last_step) = &self.last_step else {
            self.last_step = Some(ABGStep {
                time,
                predicted_value: value,
                predicted_velocity: 0.0,
                estimated_value: value,
                estimated_velocity: 0.0,
                estimated_acceleration: 0.0,
                residual_error: 0.0,
            });
            return None;
        };
        let dt = (time - last_step.time).as_seconds_f64();
        let dt2 = (dt * dt) / 2.;

        // predict step, predict the current (measured) value and velocity
        let predicted_velocity =
            last_step.estimated_velocity + last_step.estimated_acceleration * dt;
        let predicted_value = last_step.estimated_value
            + last_step.estimated_velocity * dt
            + last_step.estimated_acceleration * dt2;

        // update step, calculate residual error of the prediction
        let residual_error = value - predicted_value;

        // update step, update estimates for next iteration
        let estimated_value = predicted_value + self.alpha_coefficient * residual_error;
        let estimated_velocity =
            last_step.estimated_velocity + self.beta_coefficient * (residual_error / dt);
        let estimated_acceleration =
            last_step.estimated_acceleration + self.gamma_coefficient * (residual_error / dt2);

        let step = ABGStep {
            time,
            predicted_value,
            predicted_velocity,
            estimated_velocity,
            estimated_value,
            residual_error,
            estimated_acceleration,
        };
        self.last_step = Some(step);
        self.last_step
    }
}
