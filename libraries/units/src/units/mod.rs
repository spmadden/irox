macro_rules! basic_unit {
    ($struct_type:ident, $units_type: ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $struct_type {
            value: f64,
            units: $units_type,
        }

        impl $struct_type {
            pub fn new(value: f64, units: $units_type) -> Self {
                Self { value, units }
            }

            pub fn value(&self) -> f64 {
                self.value
            }

            pub fn units(&self) -> $units_type {
                self.units
            }
        }
    };
}

pub mod angle;
pub mod length;
