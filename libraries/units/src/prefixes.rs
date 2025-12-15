// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use core::cmp::Ordering;
use irox_tools::cfg_feature_alloc;

#[allow(unused_imports)]
use irox_tools::f64::FloatExt;

cfg_feature_alloc! {
    extern crate alloc;
    use alloc::format;
}

#[derive(Debug, Copy, Clone)]
pub struct SIPrefix {
    name: &'static str,
    symbol: &'static str,
    base_exponent: i8,
    scale_factor: f64,
}
impl PartialEq for SIPrefix {
    fn eq(&self, other: &Self) -> bool {
        self.base_exponent == other.base_exponent
    }
}
impl Eq for SIPrefix {}
impl PartialOrd for SIPrefix {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SIPrefix {
    fn cmp(&self, other: &Self) -> Ordering {
        self.base_exponent.cmp(&other.base_exponent)
    }
}
impl SIPrefix {
    #[must_use]
    pub const fn new(
        name: &'static str,
        symbol: &'static str,
        base_exponent: i8,
        scale_factor: f64,
    ) -> SIPrefix {
        Self {
            name,
            symbol,
            base_exponent,
            scale_factor,
        }
    }
    #[must_use]
    pub const fn base_exponent(&self) -> i8 {
        self.base_exponent
    }
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }
    #[must_use]
    pub const fn symbol(&self) -> &'static str {
        self.symbol
    }
    #[must_use]
    pub const fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    cfg_feature_alloc! {
        pub fn format<T: irox_tools::ToF64>(&self, t: &T) -> alloc::string::String {
            let val = t.to_f64() / self.scale_factor;
            format!("{val:.3}{}", self.symbol)
        }
        pub fn format_args<T: irox_tools::ToF64>(&self, fmt: PrefixFormat, t: &T) -> alloc::string::String {
            let val = t.to_f64() / self.scale_factor;

            format!("{val:precision$.width$}{}", self.symbol, width = fmt.width, precision = fmt.precision)
        }
    }

    pub fn display<T: irox_tools::ToF64>(&self, t: &T, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let val = t.to_f64() / self.scale_factor;
        core::write!(f, "{val:.3}{}", self.symbol)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PrefixFormat {
    width: usize,
    precision: usize,
}
impl PrefixFormat {
    #[must_use]
    pub fn new() -> Self {
        Self {
            precision: 0,
            width: 0,
        }
    }
    #[must_use]
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    #[must_use]
    pub fn with_precision(mut self, precision: usize) -> Self {
        self.precision = precision;
        self
    }
}

pub const QUETTA: SIPrefix = SIPrefix::new("quetta", "Q", 30, 1e30);
pub const RONNA: SIPrefix = SIPrefix::new("ronna", "R", 27, 1e27);
pub const YOTTA: SIPrefix = SIPrefix::new("yotta", "Y", 24, 1e24);
pub const ZETTA: SIPrefix = SIPrefix::new("zeta", "Z", 21, 1e21);
pub const EXA: SIPrefix = SIPrefix::new("exa", "E", 18, 1e18);
pub const PETA: SIPrefix = SIPrefix::new("peta", "P", 15, 1e15);
pub const TERA: SIPrefix = SIPrefix::new("tera", "T", 12, 1e12);
pub const GIGA: SIPrefix = SIPrefix::new("giga", "G", 9, 1e9);
pub const MEGA: SIPrefix = SIPrefix::new("mega", "M", 6, 1e6);
pub const KILO: SIPrefix = SIPrefix::new("kilo", "k", 3, 1e3);
pub const HECTO: SIPrefix = SIPrefix::new("hecto", "h", 2, 1e2);
pub const DECA: SIPrefix = SIPrefix::new("deca", "da", 1, 1e1);
pub const DECI: SIPrefix = SIPrefix::new("deci", "d", -1, 1e-1);
pub const CENTI: SIPrefix = SIPrefix::new("centi", "c", -2, 1e-2);
pub const MILLI: SIPrefix = SIPrefix::new("milli", "m", -3, 1e-3);
pub const MICRO: SIPrefix = SIPrefix::new("micro", "\u{03BC}", -6, 1e-6);
pub const NANO: SIPrefix = SIPrefix::new("nano", "n", -9, 1e-9);
pub const PICO: SIPrefix = SIPrefix::new("pico", "p", -12, 1e-12);
pub const FEMTO: SIPrefix = SIPrefix::new("femto", "f", -15, 1e-15);
pub const ATTO: SIPrefix = SIPrefix::new("atto", "a", -18, 1e-18);
pub const ZEPTO: SIPrefix = SIPrefix::new("zepto", "z", -21, 1e-21);
pub const YOCTO: SIPrefix = SIPrefix::new("yocto", "y", -24, 1e-24);
pub const RONTO: SIPrefix = SIPrefix::new("ronto", "r", -27, 1e-27);
pub const QUECTO: SIPrefix = SIPrefix::new("quecto", "q", -30, 1e-30);

/// All SI Prefixes
pub const ALL_PREFIXES: &[SIPrefix] = &[
    QUETTA, RONNA, YOTTA, ZETTA, EXA, PETA, TERA, GIGA, MEGA, KILO, HECTO, DECA, DECI, CENTI,
    MILLI, MICRO, NANO, PICO, FEMTO, ATTO, ZEPTO, YOCTO, RONTO, QUECTO,
];
/// Only the common power-of-three prefixes. Excludes [`QUETTA`], [`RONNA`], [`HECTO`], [`DECA`], [`DECI`], [`CENTI`], [`RONTO`], [`QUECTO`]
pub const COMMON_PREFIXES: &[SIPrefix] = &[
    YOTTA, ZETTA, EXA, PETA, TERA, GIGA, MEGA, KILO, MILLI, MICRO, NANO, PICO, FEMTO, ATTO, ZEPTO,
    YOCTO,
];

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum PrefixSet {
    All,
    #[default]
    Common,
}
impl PrefixSet {
    #[must_use]
    pub fn prefixes(&self) -> &'static [SIPrefix] {
        match self {
            Self::All => ALL_PREFIXES,
            Self::Common => COMMON_PREFIXES,
        }
    }
    pub fn best_prefix_for<T: irox_tools::ToF64>(&self, t: &T) -> Option<SIPrefix> {
        let v = t.to_f64().abs();
        let e = v.log10();
        if (0. ..1.).contains(&e) {
            return None;
        }
        let mut last_matched = None;
        let fixes: &'static [SIPrefix] = self.prefixes();
        for prefix in fixes {
            let exp = prefix.base_exponent as f64;

            last_matched = Some(*prefix);
            if exp <= e {
                break;
            }
        }
        if let Some(lm) = last_matched {
            let var = e - lm.base_exponent as f64;
            if !(0. ..3.).contains(&var) {
                return None;
            }
        }
        last_matched
    }
}

#[cfg(test)]
mod test {
    use crate::prefixes::{
        PrefixSet, ATTO, CENTI, DECA, DECI, EXA, FEMTO, GIGA, HECTO, KILO, MEGA, MICRO, MILLI,
        NANO, PETA, PICO, QUECTO, QUETTA, RONNA, RONTO, TERA, YOCTO, YOTTA, ZEPTO, ZETTA,
    };

    macro_rules! impl_test {
        ($name:ident, $com:expr, $v:literal, $all:expr) => {
            #[test]
            pub fn $name() {
                assert_eq!($com, PrefixSet::Common.best_prefix_for(&$v), "{:e}", $v);
                assert_eq!($all, PrefixSet::All.best_prefix_for(&$v), "{:e}", $v);

                let v: f64 = ($v as f64).abs();
                let f = 10f64.powf(v.log10() + 0.3f64);
                assert_eq!($com, PrefixSet::Common.best_prefix_for(&f), "{v:e} {f:e}");
                let f = 10f64.powf(v.log10() + 0.7f64);
                assert_eq!($com, PrefixSet::Common.best_prefix_for(&f), "{v:e} {f:e}");
            }
        };
    }
    impl_test!(test_quecto_30, None, 1e-30, Some(QUECTO));
    impl_test!(test_quecto_29, None, 1e-29, Some(QUECTO));
    impl_test!(test_quecto_28, None, 1e-28, Some(QUECTO));

    impl_test!(test_ronto_27, None, 1e-27, Some(RONTO));
    impl_test!(test_ronto_26, None, 1e-26, Some(RONTO));
    impl_test!(test_ronto_25, None, 1e-25, Some(RONTO));

    impl_test!(test_yocto_24, Some(YOCTO), 1e-24, Some(YOCTO));
    impl_test!(test_yocto_23, Some(YOCTO), 1e-23, Some(YOCTO));
    impl_test!(test_yocto_22, Some(YOCTO), 1e-22, Some(YOCTO));

    impl_test!(test_zepto_21, Some(ZEPTO), 1e-21, Some(ZEPTO));
    impl_test!(test_zepto_20, Some(ZEPTO), 1e-20, Some(ZEPTO));
    impl_test!(test_zepto_19, Some(ZEPTO), 1e-19, Some(ZEPTO));

    impl_test!(test_atto_18, Some(ATTO), 1e-18, Some(ATTO));
    impl_test!(test_atto_17, Some(ATTO), 1e-17, Some(ATTO));
    impl_test!(test_atto_16, Some(ATTO), 1e-16, Some(ATTO));

    impl_test!(test_femto_15, Some(FEMTO), 1e-15, Some(FEMTO));
    impl_test!(test_femto_14, Some(FEMTO), 1e-14, Some(FEMTO));
    impl_test!(test_femto_13, Some(FEMTO), 1e-13, Some(FEMTO));

    impl_test!(test_pico_12, Some(PICO), 1e-12, Some(PICO));
    impl_test!(test_pico_11, Some(PICO), 1e-11, Some(PICO));
    impl_test!(test_pico_10, Some(PICO), 1e-10, Some(PICO));

    impl_test!(test_nano_09, Some(NANO), 1e-9, Some(NANO));
    impl_test!(test_nano_08, Some(NANO), 1e-8, Some(NANO));
    impl_test!(test_nano_07, Some(NANO), 1e-7, Some(NANO));

    impl_test!(test_micro_06, Some(MICRO), 1e-6, Some(MICRO));
    impl_test!(test_micro_m06, Some(MICRO), -1e-6, Some(MICRO));
    impl_test!(test_micro_05, Some(MICRO), 1e-5, Some(MICRO));
    impl_test!(test_micro_m05, Some(MICRO), -1e-5, Some(MICRO));
    impl_test!(test_micro_04, Some(MICRO), 1e-4, Some(MICRO));
    impl_test!(test_micro_m04, Some(MICRO), -1e-4, Some(MICRO));

    impl_test!(test_milli_03, Some(MILLI), 1e-3, Some(MILLI));
    impl_test!(test_milli_02, Some(MILLI), 1e-2, Some(CENTI));
    impl_test!(test_milli_01, Some(MILLI), 1e-1, Some(DECI));

    impl_test!(test_none, None, 1e0, None);
    impl_test!(test_deca_01, None, 1e1, Some(DECA));
    impl_test!(test_hecto_02, None, 1e2, Some(HECTO));

    impl_test!(test_kilo_03, Some(KILO), 1e3, Some(KILO));
    impl_test!(test_kilo_04, Some(KILO), 1e4, Some(KILO));
    impl_test!(test_kilo_05, Some(KILO), 1e5, Some(KILO));

    impl_test!(test_mega_06, Some(MEGA), 1e6, Some(MEGA));
    impl_test!(test_mega_07, Some(MEGA), 1e7, Some(MEGA));
    impl_test!(test_mega_08, Some(MEGA), 1e8, Some(MEGA));

    impl_test!(test_giga_09, Some(GIGA), 1e9, Some(GIGA));
    impl_test!(test_giga_10, Some(GIGA), 1e10, Some(GIGA));
    impl_test!(test_giga_11, Some(GIGA), 1e11, Some(GIGA));

    impl_test!(test_tera_12, Some(TERA), 1e12, Some(TERA));
    impl_test!(test_tera_13, Some(TERA), 1e13, Some(TERA));
    impl_test!(test_tera_14, Some(TERA), 1e14, Some(TERA));

    impl_test!(test_peta_15, Some(PETA), 1e15, Some(PETA));
    impl_test!(test_peta_16, Some(PETA), 1e16, Some(PETA));
    impl_test!(test_peta_17, Some(PETA), 1e17, Some(PETA));

    impl_test!(test_exa_18, Some(EXA), 1e18, Some(EXA));
    impl_test!(test_exa_19, Some(EXA), 1e19, Some(EXA));
    impl_test!(test_exa_20, Some(EXA), 1e20, Some(EXA));

    impl_test!(test_zetta_21, Some(ZETTA), 1e21, Some(ZETTA));
    impl_test!(test_zetta_22, Some(ZETTA), 1e22, Some(ZETTA));
    impl_test!(test_zetta_23, Some(ZETTA), 1e23, Some(ZETTA));

    impl_test!(test_yotta_24, Some(YOTTA), 1e24, Some(YOTTA));
    impl_test!(test_yotta_25, Some(YOTTA), 1e25, Some(YOTTA));
    impl_test!(test_yotta_26, Some(YOTTA), 1e26, Some(YOTTA));

    impl_test!(test_ronna_27, None, 1e27, Some(RONNA));
    impl_test!(test_ronna_28, None, 1e28, Some(RONNA));
    impl_test!(test_ronna_29, None, 1e29, Some(RONNA));
    impl_test!(test_quetta_30, None, 1e30, Some(QUETTA));
    impl_test!(test_quetta_31, None, 1e31, Some(QUETTA));

    #[cfg(feature = "alloc")]
    #[test]
    pub fn test_format() {
        use crate::prefixes::PrefixFormat;
        assert_eq!("2.000k", KILO.format(&2e3));
        assert_eq!(
            "2.25k",
            KILO.format_args(
                PrefixFormat::new().with_width(2).with_precision(4),
                &2.2501e3
            )
        );
    }
}
