// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use core::ops::{Deref, DerefMut};
use irox_tools::{cfg_feature_std, ToF64};

#[allow(unused_imports)]
use irox_tools::f64::FloatExt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

    cfg_feature_std! {
        pub fn format<T: ToF64>(&self, v: &T) -> String {
            let value = v.to_f64();
            if let Some(prefix) = crate::prefixes::PrefixSet::Common.best_prefix_for(&value) {
                let scale = value / prefix.scale_factor();
                format!("{scale:.3}{}{}", prefix.symbol(), self.symbol())
            } else {
                format!("{:.3}{}", value, self.symbol() )
            }
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

cfg_feature_std! {
    use core::fmt::{Display, Formatter};
    impl<T: ToF64> Display for Quantity<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.write_str(&self.unit.format(self.value()))
        }
    }
}

pub const ONE_HYPERFINE_SECOND: Quantity<u64> = Quantity::new(9_192_631_770, Units::Hertz);
pub const SPEED_OF_LIGHT_VACUUM: Quantity<u64> = Quantity::new(299_792_458, Units::MeterPerSecond);
pub const ELEMENTARY_CHARGE: Quantity<f64> = Quantity::new(1.602176634e-19, Units::Coulomb);

#[cfg(all(test, feature = "std"))]
mod test {
    use crate::quantities::{Quantity, Units};

    #[test]
    pub fn test() {
        assert_eq!("1.025mV", Units::Volt.format(&1.025e-3));
        assert_eq!("10.250nV", Units::Volt.format(&1.025e-8));

        let mut q = Quantity::new(1.0256e-3, Units::Volt);
        assert_eq!("1.026mV", q.to_string());
        assert_eq!("1.026mV", format!("{q}"));
        *q = 1.025e-8;
        assert_eq!("10.250nV", q.to_string());
        assert_eq!("10.250nV", format!("{q}"));
        *q = 1.025e4;
        assert_eq!("10.250kV", q.to_string());
        assert_eq!("10.250kV", format!("{q}"));

        let q = Quantity::new(1.0256e-8, Units::Ohm);
        assert_eq!("10.256n\u{03A9}", q.to_string());

        let q = Quantity::new(1.0256e-8, Units::Celsius);
        assert_eq!("10.256n\u{00B0}C", q.to_string());

        let q = Quantity::new(1.0256e-8, Units::SquareMeter);
        assert_eq!("10.256nm\u{00B2}", q.to_string());
        let q = Quantity::new(1.0256e-8, Units::CubicMeter);
        assert_eq!("10.256nm\u{00B3}", q.to_string());
        let q = Quantity::new(1.0256e-8, Units::MeterPerSecondPerSecond);
        assert_eq!("10.256nm/s\u{00B2}", q.to_string());
    }
}
