// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

extern crate alloc;
use crate::fitting::LinearRegression;
use crate::sampling::Sample;
use crate::streaming::Summary;
use alloc::collections::BTreeMap;
use core::fmt::Debug;
use core::ops::Deref;
use core::ops::{Add, Div, Mul, Sub};
use irox_time::epoch::Timestamp;
use irox_time::Duration;
use irox_tools::debug_assert_eq_eps;
use irox_tools::f64::FloatExt;

/// A convolution kernel generator.
pub trait KernelGenerator {
    ///
    /// The total number of points required for this kernel.  Cannot be zero - usually an odd value.
    fn required_number_of_points(&self) -> usize;

    ///
    /// The minimum (negative) and maximum (positive) index into the [`get_kernel_value`] function.  This defaults to [(1-N)/2, (N-1)/2].
    fn absolute_value_offset(&self) -> usize {
        (self.required_number_of_points() - 1) / 2
    }
    ///
    /// Calculates the value for the convolution kernel at the specified offset.  The range of valid
    /// offsets is [-offset_size, 0, offset_size]
    fn get_kernel_value(&self, offset: f64) -> f64;

    ///
    /// Returns the expected value of the kernel.  Most kernels will be 1.0 though
    /// some (like first-derivative kernels) will be at zero.
    fn expected_weighted_sum(&self) -> f64 {
        1.0f64
    }
}
pub struct SavitszkyGolaySmoother23 {
    m: usize,
}
impl SavitszkyGolaySmoother23 {
    pub const fn new(m: usize) -> Self {
        Self { m }
    }
    pub const fn absolute_value_offset(&self) -> usize {
        (self.m - 1) / 2
    }
    pub const fn get_kernel_value(&self, offset: f64) -> f64 {
        let m = self.absolute_value_offset() as f64;
        let msq = m * m;
        let m2 = 2. * m;
        let m3 = 3. * m;
        let a = 3. * (3. * msq + m3 - 1. - 5. * offset * offset);
        let b = (m2 + 3.) * (m2 + 1.) * (m2 - 1.);
        a / b
    }
}
impl KernelGenerator for SavitszkyGolaySmoother23 {
    fn required_number_of_points(&self) -> usize {
        self.m
    }

    fn get_kernel_value(&self, offset: f64) -> f64 {
        SavitszkyGolaySmoother23::get_kernel_value(self, offset)
    }
}
pub struct SavitszkyGolaySmoother24Builder;
impl KernelBuilder for SavitszkyGolaySmoother24Builder {
    type Output = SavitszkyGolaySmoother23;

    fn generate_kernel(&self, num_samples: usize) -> Option<Self::Output> {
        (num_samples >= self.minimum_samples()).then(|| SavitszkyGolaySmoother23::new(num_samples))
    }

    fn minimum_samples(&self) -> usize {
        3
    }
}
macro_rules! make_fn {
    ($name:ident,$strukt:ident) => {
        #[allow(clippy::indexing_slicing)]
        const fn $name<const N: usize>() -> [f64; N] {
            let m = ((N - 1) / 2) as i32;
            let sv = $strukt::new(N);
            let mut off = -m;
            let mut out = [0.; N];
            let mut idx = 0;
            while idx < N {
                out[idx] = sv.get_kernel_value(off as f64);
                idx += 1;
                off += 1;
            }
            out
        }
    };
}
make_fn!(make_savitskygolay_23, SavitszkyGolaySmoother23);
pub const SAVITZKY_GOLAY_SMOOTH_23_5: [f64; 5] = make_savitskygolay_23::<5>();
pub const SAVITZKY_GOLAY_SMOOTH_23_7: [f64; 7] = make_savitskygolay_23::<7>();
pub const SAVITZKY_GOLAY_SMOOTH_23_9: [f64; 9] = make_savitskygolay_23::<9>();

pub struct SavitszkyGolaySmoother45 {
    m: usize,
    denom: f64,
    b: f64,
}
impl SavitszkyGolaySmoother45 {
    pub const fn new(m: usize) -> Self {
        let mf = ((m - 1) / 2) as f64;
        let twom = mf * 2.;
        let mf2 = mf * mf;
        let mf3 = mf * mf2;
        let mf4 = mf * mf3;
        let denom = (twom + 5.) * (twom + 3.) * (twom + 1.) * (twom - 1.) * (twom - 3.);
        let b = 15. * mf4 + 30. * mf3 - 35. * mf2 - 50. * mf + 12.;
        Self { m, denom, b }
    }
    pub const fn absolute_value_offset(&self) -> usize {
        (self.m - 1) / 2
    }
    pub const fn get_kernel_value(&self, offset: f64) -> f64 {
        let m = self.absolute_value_offset() as f64;
        let s2 = offset * offset;
        let s4 = s2 * s2;
        let twom = 2. * m;
        let m2 = m * m;
        let a = 15. / 4.;
        let c = 35. * (2. * m2 + twom - 3.) * s2;
        a * ((self.b - c + 63. * s4) / self.denom)
    }
}
impl KernelGenerator for SavitszkyGolaySmoother45 {
    fn required_number_of_points(&self) -> usize {
        self.m
    }
    fn get_kernel_value(&self, offset: f64) -> f64 {
        SavitszkyGolaySmoother45::get_kernel_value(self, offset)
    }
}
make_fn!(make_savitskygolay_45, SavitszkyGolaySmoother45);
pub const SAVITZKY_GOLAY_SMOOTH_45_5: [f64; 5] = make_savitskygolay_45::<5>();
pub const SAVITZKY_GOLAY_SMOOTH_45_7: [f64; 7] = make_savitskygolay_45::<7>();
pub const SAVITZKY_GOLAY_SMOOTH_45_9: [f64; 9] = make_savitskygolay_45::<9>();

pub struct SavitskyGolay1DerivOrder2 {
    m: usize,
    denom: f64,
}
impl SavitskyGolay1DerivOrder2 {
    pub const fn new(m: usize) -> Self {
        let mf = ((m - 1) / 2) as f64;
        let denom = (2. * mf + 1.) * (mf + 1.) * mf;
        Self { m, denom }
    }
    pub const fn absolute_value_offset(&self) -> usize {
        (self.m - 1) / 2
    }
    pub const fn get_kernel_value(&self, offset: f64) -> f64 {
        (3. * offset) / self.denom
    }
}
impl KernelGenerator for SavitskyGolay1DerivOrder2 {
    fn required_number_of_points(&self) -> usize {
        self.m
    }
    fn get_kernel_value(&self, offset: f64) -> f64 {
        SavitskyGolay1DerivOrder2::get_kernel_value(self, offset)
    }

    fn expected_weighted_sum(&self) -> f64 {
        0.0
    }
}
make_fn!(make_savitskygolay_1d2, SavitskyGolay1DerivOrder2);
pub const SAVITZKY_GOLAY_1D_2_5: [f64; 5] = make_savitskygolay_1d2::<5>();
pub const SAVITZKY_GOLAY_1D_2_7: [f64; 7] = make_savitskygolay_1d2::<7>();
pub const SAVITZKY_GOLAY_1D_2_9: [f64; 9] = make_savitskygolay_1d2::<9>();

pub struct SavitzkyGolay1DerivOrder2Builder;
impl KernelBuilder for SavitzkyGolay1DerivOrder2Builder {
    type Output = SavitskyGolay1DerivOrder2;

    fn generate_kernel(&self, num_samples: usize) -> Option<Self::Output> {
        (num_samples >= self.minimum_samples()).then(|| SavitskyGolay1DerivOrder2::new(num_samples))
    }

    fn minimum_samples(&self) -> usize {
        3
    }
}

pub struct SavitzkyGolay1DerivOrder34 {
    m: usize,
    denom: f64,
    a: f64,
    b: f64,
}
impl SavitzkyGolay1DerivOrder34 {
    pub const fn new(m: usize) -> Self {
        let mf = ((m - 1) / 2) as f64;

        let mut denom: f64 = 2. * mf + 3.;
        denom *= 2. * mf + 1.;
        denom *= 2. * mf - 1.;
        denom *= mf + 2.;
        denom *= mf + 1.;
        denom *= mf;
        denom *= mf - 1.;

        let mf2 = mf * mf;
        let mf3 = mf2 * mf;
        let mf4 = mf3 * mf;

        let a = 3. * mf4 + 6. * mf3 - 3. * mf + 1.;
        let b = 3. * mf2 + 3. * mf - 1.;

        Self { m, denom, a, b }
    }

    pub const fn absolute_value_offset(&self) -> usize {
        (self.m - 1) / 2
    }
    pub const fn get_kernel_value(&self, offset: f64) -> f64 {
        let a = 5. * self.a * offset;
        let o2 = offset * offset;
        let o3 = o2 * offset;
        let b = 7. * self.b * o3;

        let top = 5. * (a - b);

        top / self.denom
    }
}
impl KernelGenerator for SavitzkyGolay1DerivOrder34 {
    fn required_number_of_points(&self) -> usize {
        self.m
    }

    fn get_kernel_value(&self, offset: f64) -> f64 {
        SavitzkyGolay1DerivOrder34::get_kernel_value(self, offset)
    }

    fn expected_weighted_sum(&self) -> f64 {
        0.0
    }
}
pub struct SavitzkyGolay1DerivOrder34Builder;
impl KernelBuilder for SavitzkyGolay1DerivOrder34Builder {
    type Output = SavitzkyGolay1DerivOrder34;

    fn generate_kernel(&self, num_samples: usize) -> Option<Self::Output> {
        (num_samples >= self.minimum_samples())
            .then(|| SavitzkyGolay1DerivOrder34::new(num_samples))
    }

    fn minimum_samples(&self) -> usize {
        3
    }
}
make_fn!(make_savitskygolay_1d34, SavitzkyGolay1DerivOrder34);
pub const SAVITZKY_GOLAY_1D_3_5: [f64; 5] = make_savitskygolay_1d34::<5>();
pub const SAVITZKY_GOLAY_1D_3_7: [f64; 7] = make_savitskygolay_1d34::<7>();
pub const SAVITZKY_GOLAY_1D_3_9: [f64; 9] = make_savitskygolay_1d34::<9>();

///
/// This struct is a rolling time window for the provided data.  It will automatically "throw out"
/// data that falls outside (older) than the most recent data provided.  It does NOT do any
/// downsampling or processing of the data.   
pub struct TimeWindow<T> {
    values: BTreeMap<Timestamp<T>, f64>,
    window_duration: Duration,
}
impl<T: Copy> TimeWindow<T> {
    pub fn new(window_duration: Duration) -> Self {
        Self {
            window_duration,
            values: BTreeMap::<Timestamp<T>, f64>::new(),
        }
    }
    pub fn insert(&mut self, timestamp: Timestamp<T>, value: f64) {
        self.values.insert(timestamp, value);
        let Some(last) = self.values.last_key_value() else {
            return;
        };
        let window_start = last.0 - self.window_duration;
        self.values = self.values.split_off(&window_start);
    }
    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn add_sample(&mut self, samp: Sample<T>) {
        self.insert(samp.time, samp.value);
    }

    /// returns the first (earliest, oldest) sample provided
    #[must_use]
    pub fn first_key_value(&self) -> Option<(&Timestamp<T>, &f64)> {
        self.values.first_key_value()
    }
    /// returns the last (latest, newest) sample provided
    #[must_use]
    pub fn last_key_value(&self) -> Option<(&Timestamp<T>, &f64)> {
        self.values.last_key_value()
    }

    /// number of samples currently stored.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Copies the set of the data out
    #[must_use]
    pub fn data(&self) -> Vec<f64> {
        self.values.values().copied().collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Timestamp<T>, &f64)> + Clone {
        self.values.iter()
    }

    #[must_use]
    pub fn map_data<V, F: Fn((&Timestamp<T>, &f64)) -> V>(&self, fun: F) -> Vec<V> {
        let mut out = Vec::with_capacity(self.len());
        for v in &self.values {
            out.push(fun(v));
        }
        out
    }
}

///
/// How to choose the output timestamp of the window'ed function.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum WindowBinStrategy {
    /// The lower value of the window (min)
    Lower,
    /// The center value of the window `(max-min)/2`
    #[default]
    Center,
    /// The upper value of the window (max)
    Upper,
}
pub trait KernelBuilder {
    type Output: KernelGenerator;
    fn generate_kernel(&self, num_samples: usize) -> Option<Self::Output>;
    fn minimum_samples(&self) -> usize;
}
///
/// Time-series data downsampling based on a convolution kernel.  Stores up to 2x the time window
/// for nyquist sampling reasons.  Once it has a full time window duration, will run the kernel and
/// provide the result of the convolution operation.
pub struct TimedWindowFilter<T, K: KernelGenerator> {
    values: TimeWindow<T>,
    window_duration: Duration,
    bin_strategy: WindowBinStrategy,
    kernel_generator: Box<dyn KernelBuilder<Output = K>>,
}
impl<T: Copy, K: KernelGenerator> TimedWindowFilter<T, K> {
    pub fn new(
        window_duration: Duration,
        bin_strategy: WindowBinStrategy,
        kernel_generator: Box<dyn KernelBuilder<Output = K>>,
    ) -> Self {
        Self {
            window_duration,
            bin_strategy,
            kernel_generator,
            values: TimeWindow::new(window_duration * 2.0),
        }
    }
    pub fn add_sample(&mut self, sample: Sample<T>) -> Option<Sample<T>> {
        self.insert(sample.time, sample.value)
    }
    ///
    /// Push a new sample into the filter.  If there's sufficient data to run the downsampling,
    /// will run and return the result.
    pub fn insert(&mut self, time: Timestamp<T>, value: f64) -> Option<Sample<T>> {
        self.values.insert(time, value);

        if self.kernel_generator.minimum_samples() > self.values.len() {
            // not enough samples to meet the requirements of the kernel.
            return None;
        }
        let earliest = self.values.first_key_value()?;
        let latest = self.values.last_key_value()?;
        let stored_range = latest.0 - earliest.0;
        if stored_range <= self.window_duration {
            // collect more datas.
            return None;
        }
        let numvals = self.values.len();
        if numvals & 0x01 == 0x00 {
            // even # of samps, wait one.
            return None;
        }
        let last = *latest.0;
        let window_start = last - self.window_duration;

        let filter = self.kernel_generator.generate_kernel(numvals)?;

        let center_time = window_start + self.window_duration / 2.;
        // convolve!
        let mut out = 0f64;
        let mut tally = 0f64;
        for (idx, (_time, val)) in self.values.iter().enumerate() {
            let idx = idx as i32 - filter.absolute_value_offset() as i32;
            // let idx = (*time - center_time) / self.window_duration * numvals as f64;
            let kernel = filter.get_kernel_value(idx as f64);
            out += kernel * val;
            tally += kernel;
        }
        let scale = 1.0 - (filter.expected_weighted_sum() - tally);
        out /= scale;
        debug_assert_eq_eps!(filter.expected_weighted_sum(), tally, 1e-15);
        let out_time = match self.bin_strategy {
            WindowBinStrategy::Lower => window_start,
            WindowBinStrategy::Center => center_time,
            WindowBinStrategy::Upper => last,
        };
        self.values.clear();
        Some(Sample::new(out, out_time))
    }
}
///
/// An individual timed bin.  Has a start time, a width, and the min/mean/max summary of the data
/// within that bin.
#[derive()]
pub struct WindowBin<V, I, R> {
    pub width: R,
    pub start: I,
    pub summary: Summary<V>,
}
impl<V: Default, I, R> WindowBin<V, I, R> {
    pub fn new(width: R, start: I) -> Self {
        Self {
            width,
            start,
            summary: Summary::default(),
        }
    }
}
impl<
        T: Sub<T, Output = T>
            + PartialOrd
            + Copy
            + Default
            + Div<f64, Output = T>
            + Add<T, Output = T>
            + Mul<f64, Output = T>
            + Mul<T, Output = T>
            + FloatExt<Type = T>,
        I,
        R,
    > WindowBin<T, I, R>
{
    pub fn insert(&mut self, value: T) {
        self.summary.add_sample(value);
    }
}
impl<V, I, R> Deref for WindowBin<V, I, R> {
    type Target = Summary<V>;
    fn deref(&self) -> &Self::Target {
        &self.summary
    }
}
///
/// Time series data binning.  Initialize it with a bin width and it will downsample/re-bin
/// your data providing each bin as a [`WindowBin`]
pub struct BinStatistics<V, I, R> {
    pub bin_width: R,
    pub bins: BTreeMap<i64, WindowBin<V, I, R>>,
    pub anchor: Option<I>,
}

impl<T: Copy> BinStatistics<f64, Timestamp<T>, Duration> {
    pub fn new(bin_width: Duration) -> Self {
        Self {
            bin_width,
            bins: Default::default(),
            anchor: None,
        }
    }
    fn bindex(&mut self, timestamp: Timestamp<T>) -> i64 {
        let anchor = *self.anchor.get_or_insert(timestamp);
        ((timestamp - anchor) / self.bin_width).round() as i64
    }
    ///
    /// Process and insert a sample into it's bin.  Returns a reference to the bin in which
    /// it was inserted with the latest data.
    pub fn insert(
        &mut self,
        timestamp: Timestamp<T>,
        value: f64,
    ) -> &WindowBin<f64, Timestamp<T>, Duration> {
        let bin_index = self.bindex(timestamp);
        let bin = self.bins.entry(bin_index).or_insert_with(|| {
            let anchor = *self.anchor.get_or_insert(timestamp);
            let start = anchor + bin_index as f64 * self.bin_width;
            WindowBin::new(self.bin_width, start)
        });
        bin.insert(value);
        bin
    }
    ///
    /// Garbage collection - remove all data older than the specified timestamp's bin.
    pub fn remove_data_before(&mut self, timestamp: Timestamp<T>) {
        let bin_index = self.bindex(timestamp) - 1;
        self.bins = self.bins.split_off(&bin_index);
    }
    pub fn len(&self) -> usize {
        self.bins.len()
    }
    pub fn is_empty(&self) -> bool {
        self.bins.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&i64, &WindowBin<f64, Timestamp<T>, Duration>)> {
        self.bins.iter()
    }
}

pub struct TimedLinearSlopeFilter<T> {
    values: TimeWindow<T>,
    window_duration: Duration,
    bin_strategy: WindowBinStrategy,
}
impl<T: Copy> TimedLinearSlopeFilter<T> {
    pub fn new(window_duration: Duration, bin_strategy: WindowBinStrategy) -> Self {
        Self {
            window_duration,
            bin_strategy,
            values: TimeWindow::new(window_duration),
        }
    }
    pub fn add_sample(&mut self, sample: Sample<T>) -> Option<Sample<T>> {
        self.insert(sample.time, sample.value)
    }
    ///
    /// Push a new sample into the filter.  If there's sufficient data to run the downsampling,
    /// will run and return the result.
    pub fn insert(&mut self, time: Timestamp<T>, value: f64) -> Option<Sample<T>> {
        self.values.insert(time, value);

        let earliest = self.values.first_key_value()?;
        let latest = self.values.last_key_value()?;
        let last = *latest.0;
        let stored_range = latest.0 - earliest.0;
        let window_start = last - self.window_duration;
        let center_time = window_start + self.window_duration / 2.;
        if stored_range < (self.window_duration * 0.95) {
            // collect more datas.
            return None;
        }

        let reg = LinearRegression::from_data(
            self.values.iter(),
            |(t, _v)| t.get_offset().value(),
            |(_t, v)| **v,
        )?;
        let out = reg.slope;
        let out_time = match self.bin_strategy {
            WindowBinStrategy::Lower => window_start,
            WindowBinStrategy::Center => center_time,
            WindowBinStrategy::Upper => last,
        };
        self.values.clear();
        Some(Sample::new(out, out_time))
    }
}

#[cfg(test)]
mod tests {
    use crate::windows::*;
    use irox_tools::{assert_eq_eps, assert_eq_eps_slice};

    #[test]
    pub fn test_savitz23() {
        let sv = SavitszkyGolaySmoother23::new(9);
        assert_eq!(9, sv.required_number_of_points());
        assert_eq!(4, sv.absolute_value_offset());

        let values = [
            -21. / 231.,
            14. / 231.,
            39. / 231.,
            54. / 231.,
            59. / 231.,
            54. / 231.,
            39. / 231.,
            14. / 231.,
            -21. / 231.,
        ];

        for (idx, v) in (-4..4).enumerate() {
            assert_eq_eps!(values[idx], sv.get_kernel_value(v as f64), f64::EPSILON);
        }
        assert_eq_eps_slice!(values, SAVITZKY_GOLAY_SMOOTH_23_9, f64::EPSILON);
        assert_eq_eps!(
            1.0,
            SAVITZKY_GOLAY_SMOOTH_23_9.iter().sum::<f64>(),
            f64::EPSILON
        );
    }

    #[test]
    pub fn test_savitz45() {
        let sv = SavitszkyGolaySmoother45::new(9);
        assert_eq!(9, sv.required_number_of_points());
        assert_eq!(4, sv.absolute_value_offset());

        let values = [
            15. / 429.,
            -55. / 429.,
            30. / 429.,
            135. / 429.,
            179. / 429.,
            135. / 429.,
            30. / 429.,
            -55. / 429.,
            15. / 429.,
        ];

        for (idx, v) in (-4..4).enumerate() {
            assert_eq_eps!(values[idx], sv.get_kernel_value(v as f64), 1e-15);
        }
        assert_eq_eps_slice!(values, SAVITZKY_GOLAY_SMOOTH_45_9, f64::EPSILON);
        assert_eq_eps!(
            1.0,
            SAVITZKY_GOLAY_SMOOTH_45_9.iter().sum::<f64>(),
            f64::EPSILON
        );
    }

    #[test]
    pub fn test_savitz_1d2() {
        let sv = SavitskyGolay1DerivOrder2::new(9);
        assert_eq!(9, sv.required_number_of_points());
        assert_eq!(4, sv.absolute_value_offset());

        let values = [
            -4. / 60.,
            -3. / 60.,
            -2. / 60.,
            -1. / 60.,
            0.,
            1. / 60.,
            2. / 60.,
            3. / 60.,
            4. / 60.,
        ];

        for (idx, v) in (-4..4).enumerate() {
            assert_eq_eps!(values[idx], sv.get_kernel_value(v as f64), 1e-15);
        }
        assert_eq_eps_slice!(values, SAVITZKY_GOLAY_1D_2_9, f64::EPSILON);
        assert_eq_eps!(0.0, SAVITZKY_GOLAY_1D_2_9.iter().sum::<f64>(), f64::EPSILON);
    }

    #[test]
    pub fn test_savitz_1d34() {
        let sv = SavitzkyGolay1DerivOrder34::new(9);
        assert_eq!(9, sv.required_number_of_points());
        assert_eq!(4, sv.absolute_value_offset());

        let values = [
            86. / 1188.,
            -142. / 1188.,
            -193. / 1188.,
            -126. / 1188.,
            0.,
            126. / 1188.,
            193. / 1188.,
            142. / 1188.,
            -86. / 1188.,
        ];

        for (idx, v) in (-4..=4).enumerate() {
            assert_eq_eps!(values[idx], sv.get_kernel_value(v as f64), 1e-15);
        }
        assert_eq_eps_slice!(values, SAVITZKY_GOLAY_1D_3_9, f64::EPSILON);
        assert_eq_eps!(0.0, SAVITZKY_GOLAY_1D_3_9.iter().sum::<f64>(), f64::EPSILON);
    }
}
