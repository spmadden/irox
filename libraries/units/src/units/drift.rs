// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::units::duration::Duration;
use crate::units::UnitStruct;
use core::fmt::Display;
use core::marker::PhantomData;
use core::ops::Mul;

/// Represents a drift rate or rate of change
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct DriftRate<T: UnitStruct<U> + Copy + Mul<f64, Output = T>, U> {
    drift_amount: T,
    per_duration: Duration,
    _phan: PhantomData<U>,
}
impl<T: UnitStruct<U> + Copy + Mul<f64, Output = T> + Display, U> Display for DriftRate<T, U>
where
    T: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.per_duration.value().to_bits() != 1.0f64.to_bits() {
            write!(f, "{}/{}", self.drift_amount, self.per_duration)
        } else {
            write!(
                f,
                "{}/{}",
                self.drift_amount,
                self.per_duration.units().abbreviation()
            )
        }
    }
}

impl<T: UnitStruct<U> + Copy + Mul<f64, Output = T>, U> DriftRate<T, U> {
    /// Creates a new drift value
    pub const fn new(drift_amount: T, per_duration: Duration) -> Self {
        Self {
            drift_amount,
            per_duration,
            _phan: PhantomData,
        }
    }
    /// Amount of the drift
    pub const fn drift_amount(&self) -> T {
        self.drift_amount
    }
    /// Drift amount per this unit duration
    pub const fn per_duration(&self) -> Duration {
        self.per_duration
    }
    /// Parts-per-thousand
    #[must_use]
    pub const fn new_ppk(drift_amount: T) -> Self {
        Self::new(drift_amount, PPK)
    }
    /// Parts-per-million
    #[must_use]
    pub const fn new_ppm(drift_amount: T) -> Self {
        Self::new(drift_amount, PPM)
    }
    /// Parts-per-billion
    #[must_use]
    pub const fn new_ppb(drift_amount: T) -> Self {
        Self::new(drift_amount, PPB)
    }
    /// Parts-per-trillion
    #[must_use]
    pub const fn new_ppt(drift_amount: T) -> Self {
        Self::new(drift_amount, PPT)
    }

    /// Parts-per-thousand
    #[must_use]
    pub fn as_ppk(&self) -> Self {
        let factor = self.per_duration.as_millis_f64();
        let adj = self.drift_amount * factor;
        Self::new(adj, PPK)
    }
    /// Parts-per-million
    #[must_use]
    pub fn as_ppm(&self) -> Self {
        let factor = self.per_duration.as_micros_f64();
        let adj = self.drift_amount * factor;
        Self::new(adj, PPM)
    }
    /// Parts-per-billion
    #[must_use]
    pub fn as_ppb(&self) -> Self {
        let factor = self.per_duration.as_nanos_f64();
        let adj = self.drift_amount * factor;
        Self::new(adj, PPB)
    }
    /// Parts-per-trillion
    #[must_use]
    pub fn as_ppt(&self) -> Self {
        let factor = self.per_duration.as_picos_f64();
        let adj = self.drift_amount * factor;
        Self::new(adj, PPT)
    }
}

/// Parts-per-thousand
pub const PPK: Duration = Duration::from_millis(1);
/// Parts-per-million
pub const PPM: Duration = Duration::from_micros(1);
/// Parts-per-billion
pub const PPB: Duration = Duration::from_nanos(1);
/// Parts-per-trillion
pub const PPT: Duration = Duration::from_picos(1);

#[cfg(test)]
mod tests {
    use crate::units::drift::{DriftRate, PPB, PPK, PPM};
    use crate::units::freq::Frequency;
    use core::fmt::Write;
    use irox_tools::buf::StrBuf;

    #[test]
    pub fn test_to_ppk() {
        // 1ppm = 0.001ppk
        let val = Frequency::new_hz(1);
        let chk = DriftRate::new(val, PPM).as_ppk();

        let amt = chk.drift_amount();
        assert_eq!(amt, Frequency::new_hz_f64(0.001));
        let dur = chk.per_duration();
        assert_eq!(dur, PPK);

        let mut buf = StrBuf::<80>::new();
        write!(buf, "{chk}").unwrap();
        assert_eq!("0.001 Hz/ms", buf.as_str().unwrap());
    }
    #[test]
    pub fn test_ppm_to_ppb() {
        // 1ppm = 1000ppb
        let val = Frequency::new_hz(1);
        let chk = DriftRate::new_ppm(val).as_ppb();

        let amt = chk.drift_amount();
        assert_eq!(amt, Frequency::new_hz_f64(1000.));
        let dur = chk.per_duration();
        assert_eq!(dur, PPB);

        let mut buf = StrBuf::<80>::new();
        write!(buf, "{chk}").unwrap();
        assert_eq!("1000.000 Hz/ns", buf.as_str().unwrap());
    }
}
