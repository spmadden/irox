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

    pub fn new_feet(value: f64) -> Length {
        Self {
            value,
            units: LengthUnits::Feet,
        }
    }

    pub fn as_meters(&self) -> Length {
        match self.units {
            LengthUnits::Meters => *self,
            LengthUnits::Feet => Self::new(self.value * FEET_TO_METERS, LengthUnits::Meters),
        }
    }

    pub fn as_feet(&self) -> Length {
        match self.units {
            LengthUnits::Meters => Self::new(self.value * METERS_TO_FEET, LengthUnits::Feet),
            LengthUnits::Feet => *self,
        }
    }
}

pub const FEET_TO_METERS: f64 = 3.048E-01; // Exact, as per NIST 811.2008
pub const METERS_TO_FEET: f64 = 1. / FEET_TO_METERS;
