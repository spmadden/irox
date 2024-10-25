// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Streaming Statistics
//!

use core::ops::{Add, Div, Mul, Sub};
use irox_tools::f64::FloatExt;

///
/// A statistic type that can be incrementally calculated after each sample is added.  Does not
/// necessarily have to have the full set of data available to it, and as such, should be fast.
pub trait StreamingStatistic {
    type Type;

    /// adds a sample to this streaming statistic, returning a result of the operation.
    fn add_sample(&mut self, sample: Self::Type) -> Self::Type;
    /// Returns a copy of the last sample added
    fn get_last_sample(&self) -> Self::Type;
    /// Returns the last value returned by 'add_sample'
    fn get_last_result(&self) -> Self::Type;
    /// Returns the total number of samples pushed in
    fn get_num_samples(&self) -> u64;
}

///
/// Standard rolling average/mean of all the samples pushed into it.
#[derive(Default, Debug, Clone, Copy)]
pub struct Mean<Type> {
    sample_count: u64,
    last_sample: Type,
    last_mean: Type,
}

impl<T> Mean<T>
where
    T: Copy,
{
    pub fn get_mean(&self) -> T {
        self.last_mean
    }
}

///
/// Standard rolling average/mean of all the f32 samples pushed into it.
pub type MeanF32 = Mean<f32>;
///
/// Standard rolling average/mean of all the f64 samples pushed into it.
pub type MeanF64 = Mean<f64>;

impl<Type> StreamingStatistic for Mean<Type>
where
    Type: Sub<Type, Output = Type> + Copy + Div<f64, Output = Type> + Add<Type, Output = Type>,
{
    type Type = Type;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        self.sample_count += 1;
        let mean = self.last_mean + (sample - self.last_mean) / self.sample_count as f64;
        self.last_mean = mean;
        mean
    }

    fn get_last_sample(&self) -> Self::Type {
        self.last_sample
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_mean
    }

    fn get_num_samples(&self) -> u64 {
        self.sample_count
    }
}

/// Streaming maximum function
#[derive(Default, Debug, Clone, Copy)]
pub struct Max<T> {
    max_val: T,
    last_sample: T,
    num_samples: u64,
}
impl<T: Copy> Max<T> {
    pub fn get_max_val(&self) -> T {
        self.max_val
    }
}

impl<T> StreamingStatistic for Max<T>
where
    T: PartialOrd + Copy,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        self.num_samples += 1;
        if sample.ge(&self.max_val) {
            self.max_val = sample;
        }
        self.max_val
    }

    fn get_last_sample(&self) -> Self::Type {
        self.last_sample
    }

    fn get_last_result(&self) -> Self::Type {
        self.max_val
    }

    fn get_num_samples(&self) -> u64 {
        self.num_samples
    }
}

/// Streaming minimum function
#[derive(Default, Debug, Clone, Copy)]
pub struct Min<T> {
    min_val: T,
    last_sample: T,
    num_samples: u64,
}

impl<T: Copy> Min<T> {
    pub fn get_min_val(&self) -> T {
        self.min_val
    }
}

impl<T> StreamingStatistic for Min<T>
where
    T: PartialOrd + Copy,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        self.num_samples += 1;
        if sample.le(&self.min_val) {
            self.min_val = sample;
        }
        self.min_val
    }

    fn get_last_sample(&self) -> Self::Type {
        self.last_sample
    }

    fn get_last_result(&self) -> Self::Type {
        self.min_val
    }

    fn get_num_samples(&self) -> u64 {
        self.num_samples
    }
}

///
/// Unweighted sum of squares / [Total Sum of Squares](https://en.wikipedia.org/wiki/Total_sum_of_squares) / SST
/// using Chan, Golub, LeVeque's algorithm in TR222 1.3b
#[derive(Default, Debug, Clone, Copy)]
pub struct UnweightedSumOfSquares<T> {
    means: Mean<T>,
    last_ssq: T,
}

impl<T> UnweightedSumOfSquares<T>
where
    T: Copy,
{
    pub fn get_mean(&self) -> T {
        self.means.last_mean
    }
    pub fn get_unweighted_sum_of_squares(&self) -> T {
        self.last_ssq
    }
}

impl<Type> StreamingStatistic for UnweightedSumOfSquares<Type>
where
    Type: Sub<Type, Output = Type>
        + Copy
        + Div<f64, Output = Type>
        + Add<Type, Output = Type>
        + Mul<f64, Output = Type>
        + Mul<Type, Output = Type>,
{
    type Type = Type;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        let count = self.means.get_num_samples() as f64;
        let var = sample - self.means.get_last_result();
        let ussq = self.last_ssq + var * (var / (count + 1.0)) * count;
        self.last_ssq = ussq;
        let _ = self.means.add_sample(sample);
        ussq
    }

    fn get_last_sample(&self) -> Self::Type {
        self.means.get_last_sample()
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_ssq
    }

    fn get_num_samples(&self) -> u64 {
        self.means.get_num_samples()
    }
}

///
/// Returns the [Biased Variance](https://en.wikipedia.org/wiki/Variance#Biased_sample_variance),
/// which is the Sum of the Squares (SST) scaled by 1/N (the current count)
#[derive(Default, Debug, Copy, Clone)]
pub struct BiasedVariance<T> {
    inner: UnweightedSumOfSquares<T>,
    last_result: T,
}

impl<T> BiasedVariance<T>
where
    T: Copy,
{
    pub fn get_mean(&self) -> T {
        self.inner.get_mean()
    }
    pub fn get_unweighted_sum_of_squares(&self) -> T {
        self.inner.last_ssq
    }
    pub fn get_biased_variance(&self) -> T {
        self.last_result
    }
}

impl<T> StreamingStatistic for BiasedVariance<T>
where
    T: Sub<T, Output = T>
        + Copy
        + Div<f64, Output = T>
        + Add<T, Output = T>
        + Mul<f64, Output = T>
        + Mul<T, Output = T>,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        let cnt = self.get_num_samples() + 1; // not incremented until add_sample
        let variance = self.inner.add_sample(sample) / cnt as f64;
        self.last_result = variance;
        variance
    }

    fn get_last_sample(&self) -> Self::Type {
        self.inner.get_last_sample()
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_result
    }

    fn get_num_samples(&self) -> u64 {
        self.inner.get_num_samples()
    }
}

///
/// Returns the [Unbiased Variance](https://en.wikipedia.org/wiki/Variance#Unbiased_sample_variance),
/// which is the Sum of the Squares (SST) scaled by 1/(N-1) (the last count)
#[derive(Default, Debug, Copy, Clone)]
pub struct UnbiasedVariance<T> {
    inner: UnweightedSumOfSquares<T>,
    last_result: T,
}

impl<T> UnbiasedVariance<T>
where
    T: Copy,
{
    pub fn get_mean(&self) -> T {
        self.inner.get_mean()
    }
    pub fn get_unweighted_sum_of_squares(&self) -> T {
        self.inner.last_ssq
    }
    pub fn get_unbiased_variance(&self) -> T {
        self.last_result
    }
}

impl<T> StreamingStatistic for UnbiasedVariance<T>
where
    T: Sub<T, Output = T>
        + Copy
        + Div<f64, Output = T>
        + Add<T, Output = T>
        + Mul<f64, Output = T>
        + Mul<T, Output = T>,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        let cnt = self.get_num_samples();
        let variance = self.inner.add_sample(sample) / cnt as f64;
        self.last_result = variance;
        variance
    }

    fn get_last_sample(&self) -> Self::Type {
        self.inner.get_last_sample()
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_result
    }

    fn get_num_samples(&self) -> u64 {
        self.inner.get_num_samples()
    }
}

///
/// Returns the Unbiased Standard Deviation, which is the square root of the unbiased Variance,
/// also known as the 'Sample Standard Deviation'
#[derive(Default, Debug, Copy, Clone)]
pub struct UnbiasedStandardDeviation<T> {
    inner: UnbiasedVariance<T>,
    last_result: T,
}

impl<T> UnbiasedStandardDeviation<T>
where
    T: Copy,
{
    pub fn get_mean(&self) -> T {
        self.inner.get_mean()
    }
    pub fn get_unweighted_sum_of_squares(&self) -> T {
        self.inner.get_unweighted_sum_of_squares()
    }
    pub fn get_unbiased_variance(&self) -> T {
        self.inner.last_result
    }
    pub fn get_unbiased_stdev(&self) -> T {
        self.last_result
    }
}

impl<T> StreamingStatistic for UnbiasedStandardDeviation<T>
where
    T: Sub<T, Output = T>
        + Copy
        + Div<f64, Output = T>
        + Add<T, Output = T>
        + Mul<f64, Output = T>
        + Mul<T, Output = T>
        + FloatExt<Type = T>,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        let val = self.inner.add_sample(sample).sqrt();
        self.last_result = val;
        val
    }

    fn get_last_sample(&self) -> Self::Type {
        self.inner.get_last_sample()
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_result
    }

    fn get_num_samples(&self) -> u64 {
        self.inner.get_num_samples()
    }
}

///
/// Returns the Biased Standard Deviation, which is the square root of the biased Variance,
/// also known as the 'Population Standard Deviation'
#[derive(Default, Debug, Copy, Clone)]
pub struct BiasedStandardDeviation<T> {
    inner: BiasedVariance<T>,
    last_result: T,
}

impl<T> BiasedStandardDeviation<T>
where
    T: Copy,
{
    pub fn get_mean(&self) -> T {
        self.inner.get_mean()
    }
    pub fn get_unweighted_sum_of_squares(&self) -> T {
        self.inner.get_unweighted_sum_of_squares()
    }
    pub fn get_biased_variance(&self) -> T {
        self.inner.get_biased_variance()
    }
    pub fn get_biased_stdev(&self) -> T {
        self.last_result
    }
}

impl<T> StreamingStatistic for BiasedStandardDeviation<T>
where
    T: Sub<T, Output = T>
        + Copy
        + Div<f64, Output = T>
        + Add<T, Output = T>
        + Mul<f64, Output = T>
        + Mul<T, Output = T>
        + FloatExt<Type = T>,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        let val = self.inner.add_sample(sample).sqrt();
        self.last_result = val;
        val
    }

    fn get_last_sample(&self) -> Self::Type {
        self.inner.get_last_sample()
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_result
    }

    fn get_num_samples(&self) -> u64 {
        self.inner.get_num_samples()
    }
}

pub struct Summary<T> {
    mean: Mean<T>,
    min: Min<T>,
    max: Max<T>,
    stdev: UnbiasedStandardDeviation<T>,
}
impl<T: Default> Default for Summary<T> {
    fn default() -> Self {
        Summary {
            mean: Mean::default(),
            min: Min::default(),
            max: Max::default(),
            stdev: UnbiasedStandardDeviation::default(),
        }
    }
}
impl<T> Summary<T>
where
    T: Sub<T, Output = T>
        + PartialOrd
        + Copy
        + Div<f64, Output = T>
        + Add<T, Output = T>
        + Mul<f64, Output = T>
        + Mul<T, Output = T>
        + FloatExt<Type = T>,
{
    pub fn add_sample(&mut self, value: T) {
        self.min.add_sample(value);
        self.max.add_sample(value);
        self.stdev.add_sample(value);
        self.mean.add_sample(value);
    }
    pub fn mean(&self) -> T {
        self.mean.get_mean()
    }
    pub fn min(&self) -> T {
        self.min.get_min_val()
    }
    pub fn max(&self) -> T {
        self.max.get_max_val()
    }
    pub fn stdev(&self) -> T {
        self.stdev.get_unbiased_stdev()
    }
    pub fn num_samples(&self) -> u64 {
        self.mean.get_num_samples()
    }
}
#[cfg(feature = "time,std")]
pub struct OneSecondWindows {
    epoch: irox_time::epoch::Epoch,
    windows: alloc::collections::BTreeMap<irox_time::Time64, Summary<f64>>,
}
#[cfg(feature = "time,std")]
impl OneSecondWindows {
    pub fn new(epoch: irox_time::epoch::Epoch) -> Self {
        Self {
            epoch,
            windows: alloc::collections::BTreeMap::new(),
        }
    }
    pub fn add_sample(&mut self, time: irox_time::Time64, value: f64) {
        let seconds = time.as_epoch(self.epoch).as_only_seconds();
        self.windows.entry(seconds).or_default().add_sample(value);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&irox_time::Time64, &Summary<f64>)> {
        self.windows.iter()
    }
}

#[cfg(test)]
mod test {
    use crate::streaming::*;
    use irox_tools::assert_eq_eps;
    use irox_units::units::duration::Duration;

    #[test]
    pub fn test() {
        let mut mean = MeanF64::default();
        let v = mean.add_sample(0.0);
        assert_eq!(v, 0.0);
        let v = mean.add_sample(1.0);
        assert_eq!(v, 0.5);

        let mut mean = Mean::<Duration>::default();
        let v = mean.add_sample(Duration::from_seconds(1));
        assert_eq!(v, Duration::from_seconds_f64(1.0));
        let v = mean.add_sample(Duration::from_seconds(2));
        assert_eq!(v, Duration::from_seconds_f64(1.5));

        let mut samp_stddev = UnbiasedStandardDeviation::default();
        let mut pop_stddev = BiasedStandardDeviation::default();
        for val in [2, 4, 4, 4, 5, 5, 7, 9] {
            let _ = samp_stddev.add_sample(val as f64);
            let _ = pop_stddev.add_sample(val as f64);
        }
        assert_eq!(5.0, samp_stddev.get_mean());
        assert_eq_eps!(
            4.571428571428571,
            samp_stddev.get_unbiased_variance(),
            1e-15
        );
        assert_eq_eps!(2.1380899352993947, samp_stddev.get_unbiased_stdev(), 1e-15);

        assert_eq!(5.0, pop_stddev.get_mean());
        assert_eq_eps!(4., pop_stddev.get_biased_variance(), 1e-15);
        assert_eq_eps!(2., pop_stddev.get_biased_stdev(), 1e-15);
    }
}
