macro_rules! basic_unit {
    ($struct_type:ident, $units_type: ident, $default_units: ident) => {
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $struct_type {
            value: f64,
            units: $units_type,
        }

        impl $struct_type {
            pub const fn new(value: f64, units: $units_type) -> Self {
                Self { value, units }
            }

            pub fn value(&self) -> f64 {
                self.value
            }

            pub fn units(&self) -> $units_type {
                self.units
            }
        }

        pub const ZERO: $struct_type = $struct_type::new(0.0, $units_type::$default_units);
    };
}

pub mod angle;
pub mod datasize;
pub mod length;
