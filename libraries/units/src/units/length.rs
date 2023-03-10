#[derive(Debug, Clone, Copy, Default)]
pub enum LengthUnits {
    #[default]
    Meters,

    Feet,
}

basic_unit!(Length, LengthUnits, Meters);

impl Length {
    pub fn new_meters(value: f64) -> Length {
        Self {
            value,
            units: LengthUnits::Meters,
        }
    }
}
