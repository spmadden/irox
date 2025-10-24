// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Streaming Statistics
//!

use core::fmt::{Debug, Display, Formatter};
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
    last_sample: Option<Type>,
    last_mean: Option<Type>,
}

impl<T> Mean<T>
where
    T: Copy,
{
    pub fn get_mean(&self) -> Option<T> {
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
    Type: Sub<Type, Output = Type>
        + Copy
        + Div<f64, Output = Type>
        + Add<Type, Output = Type>
        + Default,
{
    type Type = Type;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        self.sample_count += 1;
        self.last_sample = Some(sample);
        let last = self.last_mean.get_or_insert(sample);
        let mean = *last + (sample - *last) / self.sample_count as f64;
        *last = mean;
        mean
    }

    fn get_last_sample(&self) -> Self::Type {
        self.last_sample.unwrap_or_default()
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_mean.unwrap_or_default()
    }

    fn get_num_samples(&self) -> u64 {
        self.sample_count
    }
}

/// Streaming maximum function
#[derive(Default, Debug, Clone, Copy)]
pub struct Max<T> {
    max_val: Option<T>,
    last_sample: Option<T>,
    num_samples: u64,
}
impl<T: Copy> Max<T> {
    pub fn get_max_val(&self) -> Option<T> {
        self.max_val
    }
}

impl<T: Default> StreamingStatistic for Max<T>
where
    T: PartialOrd + Copy,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        self.num_samples += 1;
        let mut max = self.max_val.get_or_insert(sample);
        if sample.ge(max) {
            max = self.max_val.insert(sample);
        }
        *max
    }

    fn get_last_sample(&self) -> Self::Type {
        self.last_sample.unwrap_or_default()
    }

    fn get_last_result(&self) -> Self::Type {
        self.max_val.unwrap_or_default()
    }

    fn get_num_samples(&self) -> u64 {
        self.num_samples
    }
}

/// Streaming minimum function
#[derive(Default, Debug, Clone, Copy)]
pub struct Min<T> {
    min_val: Option<T>,
    last_sample: Option<T>,
    num_samples: u64,
}

impl<T: Copy> Min<T> {
    pub fn get_min_val(&self) -> Option<T> {
        self.min_val
    }
}

impl<T: Default> StreamingStatistic for Min<T>
where
    T: PartialOrd + Copy,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        self.num_samples += 1;
        let mut min = self.min_val.get_or_insert(sample);
        if sample.le(min) {
            min = self.min_val.insert(sample);
        }
        *min
    }

    fn get_last_sample(&self) -> Self::Type {
        self.last_sample.unwrap_or_default()
    }

    fn get_last_result(&self) -> Self::Type {
        self.min_val.unwrap_or_default()
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
    last_ssq: Option<T>,
}

impl<T> UnweightedSumOfSquares<T>
where
    T: Copy + Default,
{
    pub fn get_mean(&self) -> T {
        self.means.last_mean.unwrap_or_default()
    }
    pub fn get_unweighted_sum_of_squares(&self) -> T {
        self.last_ssq.unwrap_or_default()
    }
}

impl<Type> StreamingStatistic for UnweightedSumOfSquares<Type>
where
    Type: Sub<Type, Output = Type>
        + Copy
        + Default
        + Div<f64, Output = Type>
        + Add<Type, Output = Type>
        + Mul<f64, Output = Type>
        + Mul<Type, Output = Type>,
{
    type Type = Type;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        let count = self.means.get_num_samples() as f64;
        let var = sample - self.means.get_last_result();
        let last = self.last_ssq.unwrap_or_default();
        let ussq = last + var * (var / (count + 1.0)) * count;
        self.last_ssq = Some(ussq);
        let _ = self.means.add_sample(sample);
        ussq
    }

    fn get_last_sample(&self) -> Self::Type {
        self.means.get_last_sample()
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_ssq.unwrap_or_default()
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
    last_result: Option<T>,
}

impl<T> BiasedVariance<T>
where
    T: Copy + Default,
{
    pub fn get_mean(&self) -> T {
        self.inner.get_mean()
    }
    pub fn get_unweighted_sum_of_squares(&self) -> Option<T> {
        self.inner.last_ssq
    }
    pub fn get_biased_variance(&self) -> Option<T> {
        self.last_result
    }
}

impl<T> StreamingStatistic for BiasedVariance<T>
where
    T: Sub<T, Output = T>
        + Copy
        + Default
        + Div<f64, Output = T>
        + Add<T, Output = T>
        + Mul<f64, Output = T>
        + Mul<T, Output = T>,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        let cnt = self.get_num_samples() + 1; // not incremented until add_sample
        let variance = self.inner.add_sample(sample) / cnt as f64;
        self.last_result = Some(variance);
        variance
    }

    fn get_last_sample(&self) -> Self::Type {
        self.inner.get_last_sample()
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_result.unwrap_or_default()
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
    last_result: Option<T>,
}

impl<T> UnbiasedVariance<T>
where
    T: Copy + Default,
{
    pub fn get_mean(&self) -> T {
        self.inner.get_mean()
    }
    pub fn get_unweighted_sum_of_squares(&self) -> Option<T> {
        self.inner.last_ssq
    }
    pub fn get_unbiased_variance(&self) -> Option<T> {
        self.last_result
    }
}

impl<T> StreamingStatistic for UnbiasedVariance<T>
where
    T: Sub<T, Output = T>
        + Copy
        + Default
        + Div<f64, Output = T>
        + Add<T, Output = T>
        + Mul<f64, Output = T>
        + Mul<T, Output = T>,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        let cnt = self.get_num_samples();
        let variance = self.inner.add_sample(sample) / cnt as f64;
        self.last_result = Some(variance);
        variance
    }

    fn get_last_sample(&self) -> Self::Type {
        self.inner.get_last_sample()
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_result.unwrap_or_default()
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
    last_result: Option<T>,
}

impl<T> UnbiasedStandardDeviation<T>
where
    T: Copy + Default,
{
    pub fn get_mean(&self) -> T {
        self.inner.get_mean()
    }
    pub fn get_unweighted_sum_of_squares(&self) -> Option<T> {
        self.inner.get_unweighted_sum_of_squares()
    }
    pub fn get_unbiased_variance(&self) -> Option<T> {
        self.inner.last_result
    }
    pub fn get_unbiased_stdev(&self) -> Option<T> {
        self.last_result
    }
}

impl<T> StreamingStatistic for UnbiasedStandardDeviation<T>
where
    T: Sub<T, Output = T>
        + Copy
        + Default
        + Div<f64, Output = T>
        + Add<T, Output = T>
        + Mul<f64, Output = T>
        + Mul<T, Output = T>
        + FloatExt<Type = T>,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        let val = self.inner.add_sample(sample).sqrt();
        self.last_result = Some(val);
        val
    }

    fn get_last_sample(&self) -> Self::Type {
        self.inner.get_last_sample()
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_result.unwrap_or_default()
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
    last_result: Option<T>,
}

impl<T> BiasedStandardDeviation<T>
where
    T: Copy + Default,
{
    pub fn get_mean(&self) -> T {
        self.inner.get_mean()
    }
    pub fn get_unweighted_sum_of_squares(&self) -> Option<T> {
        self.inner.get_unweighted_sum_of_squares()
    }
    pub fn get_biased_variance(&self) -> Option<T> {
        self.inner.get_biased_variance()
    }
    pub fn get_biased_stdev(&self) -> Option<T> {
        self.last_result
    }
}

impl<T> StreamingStatistic for BiasedStandardDeviation<T>
where
    T: Sub<T, Output = T>
        + Copy
        + Default
        + Div<f64, Output = T>
        + Add<T, Output = T>
        + Mul<f64, Output = T>
        + Mul<T, Output = T>
        + FloatExt<Type = T>,
{
    type Type = T;

    fn add_sample(&mut self, sample: Self::Type) -> Self::Type {
        let val = self.inner.add_sample(sample).sqrt();
        self.last_result = Some(val);
        val
    }

    fn get_last_sample(&self) -> Self::Type {
        self.inner.get_last_sample()
    }

    fn get_last_result(&self) -> Self::Type {
        self.last_result.unwrap_or_default()
    }

    fn get_num_samples(&self) -> u64 {
        self.inner.get_num_samples()
    }
}

#[derive(Copy, Clone)]
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
impl<T: Debug + Copy + Default> Debug for Summary<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "avg: {:?} +/- {:?} (1std) [{:?}-{:?}]",
            self.mean.get_mean(),
            self.stdev.get_unbiased_stdev(),
            self.min.get_min_val(),
            self.max.get_max_val()
        ))
    }
}
impl<T> Summary<T>
where
    T: Sub<T, Output = T>
        + PartialOrd
        + Copy
        + Default
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
    #[must_use]
    pub fn mean(&self) -> Option<T> {
        self.mean.get_mean()
    }
    #[must_use]
    pub fn min(&self) -> Option<T> {
        self.min.get_min_val()
    }
    #[must_use]
    pub fn max(&self) -> Option<T> {
        self.max.get_max_val()
    }
    #[must_use]
    pub fn stddev(&self) -> Option<T> {
        self.stdev.get_unbiased_stdev()
    }
    #[must_use]
    pub fn num_samples(&self) -> u64 {
        self.mean.get_num_samples()
    }
}
impl<T> Display for Summary<T>
where
    T: Default + Display + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        core::write!(
            f,
            "min({}) max({}) mean({}) stddev({})",
            self.min.min_val.unwrap_or_default(),
            self.max.max_val.unwrap_or_default(),
            self.mean.last_mean.unwrap_or_default(),
            self.stdev.last_result.unwrap_or_default()
        )
    }
}

pub trait SummarizingIterator<'a, T: 'a>: Iterator<Item = &'a T> + Sized {
    fn summarize(self) -> Summary<T>
    where
        Self: Sized + Iterator<Item = &'a T>,
        T: Sub<T, Output = T>
            + PartialOrd
            + Copy
            + Default
            + Div<f64, Output = T>
            + Add<T, Output = T>
            + Mul<f64, Output = T>
            + Mul<T, Output = T>
            + FloatExt<Type = T>,
    {
        let mut summary = Summary::default();
        for v in self {
            summary.add_sample(*v);
        }
        summary
    }
}
impl<'a, T: Sized, B: 'a> SummarizingIterator<'a, B> for T where T: Iterator<Item = &'a B> {}

#[cfg(feature = "std")]
pub struct OneSecondWindows {
    epoch: irox_time::epoch::Epoch,
    windows: alloc::collections::BTreeMap<irox_time::Time64, Summary<f64>>,
}
#[cfg(feature = "std")]
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
            samp_stddev.get_unbiased_variance().unwrap_or_default(),
            1e-15
        );
        assert_eq_eps!(
            2.1380899352993947,
            samp_stddev.get_unbiased_stdev().unwrap_or_default(),
            1e-15
        );

        assert_eq!(5.0, pop_stddev.get_mean());
        assert_eq_eps!(
            4.,
            pop_stddev.get_biased_variance().unwrap_or_default(),
            1e-15
        );
        assert_eq_eps!(2., pop_stddev.get_biased_stdev().unwrap_or_default(), 1e-15);
    }
}
