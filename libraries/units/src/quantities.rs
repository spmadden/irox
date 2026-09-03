// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use core::ops::{Deref, DerefMut};
use irox_tools::{cfg_feature_alloc, ToF64};

cfg_feature_alloc! {
    extern crate alloc;
    use alloc::format;
}
#[allow(unused_imports)]
use irox_tools::f64::FloatExt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Units {
    Gram,
    Meter,
    SquareMeter,
    CubicMeter,
    MeterPerSecond,
    MeterPerSecondPerSecond,
    Second,
    Mole,
    Ampere,
    Kelvin,
    Candela,
    Newton,
    Joule,
    Katal,
    Coulomb,
    Celsius,
    Lux,
    Lumen,
    Farad,
    Weber,
    Watt,
    Pascal,
    Gray,
    Becquerel,
    Henry,
    Volt,
    Ohm,
    Steradian,
    Radian,
    Siemens,
    Tesla,
    Hertz,
    Sievert,
    Other {
        name: &'static str,
        symbol: &'static str,
    },
}

impl Units {
    pub fn name(&self) -> &'static str {
        match self {
            Units::Gram => "Gram",
            Units::Meter => "Meter",
            Units::SquareMeter => "SquareMeter",
            Units::CubicMeter => "CubicMeter",
            Units::MeterPerSecond => "MeterPerSecond",
            Units::MeterPerSecondPerSecond => "MeterPerSecondPerSecond",
            Units::Second => "Second",
            Units::Mole => "Mole",
            Units::Ampere => "Ampere",
            Units::Kelvin => "Kelvin",
            Units::Candela => "Candela",
            Units::Newton => "Newton",
            Units::Joule => "Joule",
            Units::Katal => "Katal",
            Units::Coulomb => "Coulomb",
            Units::Celsius => "Celsius",
            Units::Lux => "Lux",
            Units::Lumen => "Lumen",
            Units::Farad => "Farad",
            Units::Weber => "Weber",
            Units::Watt => "Watt",
            Units::Pascal => "Pascal",
            Units::Gray => "Gray",
            Units::Becquerel => "Becquerel",
            Units::Henry => "Henry",
            Units::Volt => "Volt",
            Units::Ohm => "Ohm",
            Units::Steradian => "Steradian",
            Units::Radian => "Radian",
            Units::Siemens => "Siemens",
            Units::Tesla => "Tesla",
            Units::Hertz => "Hertz",
            Units::Sievert => "Sievert",
            Units::Other { name, symbol: _ } => name,
        }
    }
    pub fn symbol(&self) -> &'static str {
        match self {
            Units::Gram => "g",
            Units::Meter => "m",
            Units::SquareMeter => "m\u{00B2}",
            Units::CubicMeter => "m\u{00B3}",
            Units::MeterPerSecond => "m/s",
            Units::MeterPerSecondPerSecond => "m/s\u{00B2}",
            Units::Second => "s",
            Units::Mole => "mol",
            Units::Ampere => "A",
            Units::Kelvin => "K",
            Units::Candela => "cd",
            Units::Newton => "N",
            Units::Joule => "J",
            Units::Katal => "kat",
            Units::Coulomb => "C",
            Units::Celsius => "\u{00B0}C",
            Units::Lux => "lx",
            Units::Lumen => "lm",
            Units::Farad => "F",
            Units::Weber => "Wb",
            Units::Watt => "W",
            Units::Pascal => "Pa",
            Units::Gray => "Gy",
            Units::Becquerel => "Bq",
            Units::Henry => "H",
            Units::Volt => "V",
            Units::Ohm => "\u{03A9}",
            Units::Steradian => "sr",
            Units::Radian => "rad",
            Units::Siemens => "S",
            Units::Tesla => "T",
            Units::Hertz => "Hz",
            Units::Sievert => "Sv",
            Units::Other { name: _, symbol } => symbol,
        }
    }

    cfg_feature_alloc! {
        pub fn format<T: ToF64>(&self, v: &T) -> alloc::string::String {
            let value = v.to_f64();
            if let Some(prefix) = crate::prefixes::PrefixSet::Common.best_prefix_for(&value) {
                let scale = value / prefix.scale_factor();
                format!("{scale:.3}{}{}", prefix.symbol(), self.symbol())
            } else {
                format!("{:.3}{}", value, self.symbol() )
            }
        }
    }

    pub fn display<T: irox_tools::ToF64, W: core::fmt::Write>(
        &self,
        v: &T,
        f: &mut W,
    ) -> core::fmt::Result {
        let value = v.to_f64();
        if let Some(prefix) = crate::prefixes::PrefixSet::Common.best_prefix_for(&value) {
            let scale = value / prefix.scale_factor();
            write!(f, "{scale:.3}{}{}", prefix.symbol(), self.symbol())
        } else {
            write!(f, "{:.3}{}", value, self.symbol())
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Quantity<T: ToF64> {
    value: T,
    unit: Units,
}
impl<T: ToF64> Quantity<T> {
    #[must_use]
    pub const fn new(value: T, unit: Units) -> Self {
        Self { value, unit }
    }
    #[must_use]
    pub const fn unit(&self) -> &Units {
        &self.unit
    }
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }
}
impl<T: ToF64> Deref for Quantity<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T: ToF64> DerefMut for Quantity<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: ToF64> core::fmt::Display for Quantity<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.unit.display(self.value(), f)
    }
}

pub const ONE_HYPERFINE_SECOND: Quantity<u64> = Quantity::new(9_192_631_770, Units::Hertz);
pub const SPEED_OF_LIGHT_VACUUM: Quantity<u64> = Quantity::new(299_792_458, Units::MeterPerSecond);
pub const ELEMENTARY_CHARGE: Quantity<f64> = Quantity::new(1.602176634e-19, Units::Coulomb);

#[cfg(test)]
mod test {
    use core::fmt::Error;
    use core::str::Utf8Error;
    use irox_tools::buf::StrBuf;
    use irox_tools::cfg_feature_alloc;
    cfg_feature_alloc! {
        extern crate alloc;
        use alloc::string::ToString;
    }

    use crate::quantities::{Quantity, Units};

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub enum TestError {
        Fmt(core::fmt::Error),
        UTF(core::str::Utf8Error),
    }
    impl From<core::fmt::Error> for TestError {
        fn from(value: Error) -> Self {
            Self::Fmt(value)
        }
    }
    impl From<core::str::Utf8Error> for TestError {
        fn from(value: Utf8Error) -> Self {
            Self::UTF(value)
        }
    }
    macro_rules! check_item {
        ($val:literal, $unit:expr, $ex:expr) => {
            let mut buf = StrBuf::<256>::new();
            Units::display(&Units::Volt, &$ex, &mut buf)?;
            assert_eq!($val, buf.as_str()?);
        };
    }
    macro_rules! check_qty {
        ($val:literal, $ex:ident) => {
            let mut buf = StrBuf::<256>::new();
            Units::display(&$ex.unit, &$ex.value, &mut buf)?;
            assert_eq!($val, buf.as_str()?);
        };
    }

    #[test]
    pub fn test() -> Result<(), TestError> {
        check_item!("1.025mV", Units::Volt, 1.025e-3);
        check_item!("10.250nV", Units::Volt, 1.025e-8);

        let mut q = Quantity::new(1.0256e-3, Units::Volt);
        check_qty!("1.026mV", q);
        cfg_feature_alloc! {
            assert_eq!("1.026mV", q.to_string());
            assert_eq!("1.026mV", irox_tools::format!("{q}"));
        }
        *q = 1.025e-8;
        check_qty!("10.250nV", q);
        cfg_feature_alloc! {
            assert_eq!("10.250nV", q.to_string());
            assert_eq!("10.250nV", irox_tools::format!("{q}"));
        }
        *q = 1.025e4;
        check_qty!("10.250kV", q);
        cfg_feature_alloc! {
            assert_eq!("10.250kV", q.to_string());
            assert_eq!("10.250kV", irox_tools::format!("{q}"));
        }

        let q = Quantity::new(1.0256e-8, Units::Ohm);
        check_qty!("10.256n\u{03A9}", q);
        cfg_feature_alloc! {
            assert_eq!("10.256n\u{03A9}", q.to_string());
        }

        let q = Quantity::new(1.0256e-8, Units::Celsius);
        check_qty!("10.256n\u{00B0}C", q);
        cfg_feature_alloc! {
            assert_eq!("10.256n\u{00B0}C", q.to_string());
        }

        let q = Quantity::new(1.0256e-8, Units::SquareMeter);
        check_qty!("10.256nm\u{00B2}", q);
        cfg_feature_alloc! {
            assert_eq!("10.256nm\u{00B2}", q.to_string());
        }
        let q = Quantity::new(1.0256e-8, Units::CubicMeter);
        check_qty!("10.256nm\u{00B3}", q);
        cfg_feature_alloc! {
            assert_eq!("10.256nm\u{00B3}", q.to_string());
        }
        let q = Quantity::new(1.0256e-8, Units::MeterPerSecondPerSecond);
        check_qty!("10.256nm/s\u{00B2}", q);
        cfg_feature_alloc! {
            assert_eq!("10.256nm/s\u{00B2}", q.to_string());
        }

        Ok(())
    }
}
