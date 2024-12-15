// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

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
pub const SAVINSKY_GOLAY_SMOOTH_23_5: [f64; 5] = make_savitskygolay_23::<5>();
pub const SAVINSKY_GOLAY_SMOOTH_23_7: [f64; 7] = make_savitskygolay_23::<7>();
pub const SAVINSKY_GOLAY_SMOOTH_23_9: [f64; 9] = make_savitskygolay_23::<9>();

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
pub const SAVINSKY_GOLAY_SMOOTH_45_5: [f64; 5] = make_savitskygolay_45::<5>();
pub const SAVINSKY_GOLAY_SMOOTH_45_7: [f64; 7] = make_savitskygolay_45::<7>();
pub const SAVINSKY_GOLAY_SMOOTH_45_9: [f64; 9] = make_savitskygolay_45::<9>();

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
}
make_fn!(make_savitskygolay_1d2, SavitskyGolay1DerivOrder2);
pub const SAVINSKY_GOLAY_1D_2_5: [f64; 5] = make_savitskygolay_1d2::<5>();
pub const SAVINSKY_GOLAY_1D_2_7: [f64; 7] = make_savitskygolay_1d2::<7>();
pub const SAVINSKY_GOLAY_1D_2_9: [f64; 9] = make_savitskygolay_1d2::<9>();

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
        assert_eq_eps_slice!(values, SAVINSKY_GOLAY_SMOOTH_23_9, f64::EPSILON);
        assert_eq_eps!(
            1.0,
            SAVINSKY_GOLAY_SMOOTH_23_9.iter().sum::<f64>(),
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
        assert_eq_eps_slice!(values, SAVINSKY_GOLAY_SMOOTH_45_9, f64::EPSILON);
        assert_eq_eps!(
            1.0,
            SAVINSKY_GOLAY_SMOOTH_45_9.iter().sum::<f64>(),
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
        assert_eq_eps_slice!(values, SAVINSKY_GOLAY_1D_2_9, f64::EPSILON);
        assert_eq_eps!(0.0, SAVINSKY_GOLAY_1D_2_9.iter().sum::<f64>(), f64::EPSILON);
    }
}
