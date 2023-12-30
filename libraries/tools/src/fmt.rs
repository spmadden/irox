// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Formatting structs and traits
//!

use core::fmt::{Display, Formatter};
extern crate alloc;
#[allow(unused_imports)]
use crate::f64::FloatExt;
use alloc::string::String;

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        extern crate alloc;
        use alloc::string::String;
        use core::fmt::Write;

        let mut val = String::new();
        val.write_fmt(format_args!($($arg)*)).expect("a formatting trait implementation returned an error");
        val
    }};
}

///
/// This struct allows you to print a specific number of digits before the decimal point,
/// and after the decimal point.
///
/// This exists because the base format trait allows you to specify a width and a precision.
/// However, in a fractional number, the width applies to the WHOLE number, including the fractional
/// component, and doesn't zero-pad effectively.
///
/// * The first parameter is the number of zero-padded digits before the decimal point.
/// * The second parameter is the number of zero-padded digits after the decimal point.
///
/// # Example:
/// ```
/// use irox_tools::fmt::DecimalFormatF64;
/// assert_eq!("00.1235", format!("{}", DecimalFormatF64(2,4,0.1234567)));
/// ```
pub struct DecimalFormatF64(pub usize, pub usize, pub f64);

impl Display for DecimalFormatF64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut base = self.2.trunc();
        let width = self.0;
        let prec = self.1;
        let powi = 10_u64.pow(self.1 as u32) as f64;
        let mut val = (self.2.fract().abs() * powi).round();
        if val >= powi {
            base += 1.;
            val -= powi;
        }
        let val = val as u64;
        write!(f, "{base:0width$}.{val:0prec$}")
    }
}

///
/// This struct allows you to print a specific number of digits before the decimal point,
/// and after the decimal point.
///
/// This exists because the base format trait allows you to specify a width and a precision.
/// However, in a fractional number, the width applies to the WHOLE number, including the fractional
/// component, and doesn't zero-pad effectively.
///
/// * The first parameter is the number of zero-padded digits before the decimal point.
/// * The second parameter is the number of zero-padded digits after the decimal point.
///
/// # Example:
/// ```
/// use irox_tools::fmt::DecimalFormatF32;
/// assert_eq!("00.1235", format!("{}", DecimalFormatF32(2,4,0.1234567)));
/// ```
pub struct DecimalFormatF32(pub usize, pub usize, pub f32);

impl Display for DecimalFormatF32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut base = self.2.trunc();
        let width = self.0;
        let prec = self.1;
        let powi = 10_u64.pow(self.1 as u32) as f32;
        let mut val = (self.2.fract().abs() * powi).round();
        if val >= powi {
            base += 1.;
            val -= powi;
        }
        let val = val as u64;
        write!(f, "{base:0width$}.{val:0prec$}")
    }
}

///
/// This struct allows you to print a specific number of digits before the decimal point,
/// and after the decimal point.
///
/// This exists because the base format trait allows you to specify a width and a precision.
/// However, in a fractional number, the width applies to the WHOLE number, including the fractional
/// component, and doesn't zero-pad effectively.
///
/// * The first parameter is the number of zero-padded digits before the decimal point.
/// * The second parameter is the number of zero-padded digits after the decimal point.
///
/// # Example:
/// ```
/// use irox_tools::fmt::DecimalFormat;
/// let fmt = DecimalFormat::new(2,4);
///
/// assert_eq!("00.1235", fmt.format_f64(0.1234567));
/// ```
pub struct DecimalFormat {
    width: usize,
    precision: usize,
}

impl DecimalFormat {
    pub fn new(width: usize, precision: usize) -> Self {
        Self { width, precision }
    }

    ///
    /// Formats the specified [`f64`] using this formatter.
    ///
    /// # Example:
    /// ```
    /// use irox_tools::fmt::DecimalFormat;
    /// let fmt = DecimalFormat::new(2,4);
    ///
    /// assert_eq!("00.1235", fmt.format_f64(0.1234567));
    /// ```
    pub fn format_f64(&self, val: f64) -> String {
        format!("{}", DecimalFormatF64(self.width, self.precision, val))
    }

    ///
    /// Formats the specified [`f32`] using this formatter.
    ///
    /// # Example:
    /// ```
    /// use irox_tools::fmt::DecimalFormat;
    /// let fmt = DecimalFormat::new(2,4);
    ///
    /// assert_eq!("00.1235", fmt.format_f32(0.1234567));
    /// ```
    pub fn format_f32(&self, val: f32) -> String {
        format!("{}", DecimalFormatF32(self.width, self.precision, val))
    }
}

#[cfg(test)]
mod tests {
    use crate::fmt::DecimalFormatF64;

    #[test]
    pub fn test() {
        let val = 0.1234567;
        assert_eq!("00.1235", format!("{}", DecimalFormatF64(2, 4, val)));

        assert_eq!("0.123", format!("{}", DecimalFormatF64(0, 3, val)));
        assert_eq!("0.123", format!("{}", DecimalFormatF64(1, 3, val)));
        assert_eq!("00.123", format!("{}", DecimalFormatF64(2, 3, val)));
        assert_eq!("00.12346", format!("{}", DecimalFormatF64(2, 5, val)));
        assert_eq!("00.123457", format!("{}", DecimalFormatF64(2, 6, val)));
        assert_eq!("00.1234567", format!("{}", DecimalFormatF64(2, 7, val)));
        assert_eq!("00.12345670", format!("{}", DecimalFormatF64(2, 8, val)));
        assert_eq!("00.123456700", format!("{}", DecimalFormatF64(2, 9, val)));
        assert_eq!(
            "000.1234567000",
            format!("{}", DecimalFormatF64(3, 10, val))
        );
    }

    #[test]
    pub fn test2() {
        assert_eq!("1.0", format!("{}", DecimalFormatF64(1, 0, 0.98)));
        assert_eq!("1.0", format!("{}", DecimalFormatF64(1, 1, 0.98)));
        assert_eq!("0.98", format!("{}", DecimalFormatF64(1, 2, 0.98)));
        assert_eq!("0.980", format!("{}", DecimalFormatF64(1, 3, 0.98)));
        assert_eq!("0.950", format!("{}", DecimalFormatF64(1, 3, 0.95)));
        assert_eq!("0.940", format!("{}", DecimalFormatF64(1, 3, 0.94)));
        assert_eq!("0.94", format!("{}", DecimalFormatF64(1, 2, 0.94)));
        assert_eq!("0.9", format!("{}", DecimalFormatF64(1, 1, 0.94)));
        assert_eq!("1.0", format!("{}", DecimalFormatF64(1, 0, 0.94)));

        assert_eq!("0.999", format!("{}", DecimalFormatF64(1, 3, 0.999)));
        assert_eq!("0.9990", format!("{}", DecimalFormatF64(1, 4, 0.999)));
        assert_eq!("1.00", format!("{}", DecimalFormatF64(1, 2, 0.999)));

        assert_eq!("-21.30", format!("{}", DecimalFormatF64(2, 2, -21.3)));
        assert_eq!("-21.3", format!("{}", DecimalFormatF64(2, 1, -21.3)));
        assert_eq!("-21.0", format!("{}", DecimalFormatF64(2, 0, -21.3)));
    }
}
