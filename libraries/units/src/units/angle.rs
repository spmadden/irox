#[derive(Debug, Clone, Copy)]
pub enum AngleUnits {
    Degrees,
    Radians,
}

basic_unit!(Angle, AngleUnits);

impl Angle {
    pub fn as_units(&self, units: AngleUnits) -> Angle {
        match units {
            AngleUnits::Degrees => self.as_degrees(),
            AngleUnits::Radians => todo!(),
        }
    }

    pub fn as_degrees(&self) -> Angle {
        match self.units {
            AngleUnits::Degrees => *self,
            AngleUnits::Radians => Angle::new(self.value * RAD_2_DEG, AngleUnits::Radians),
        }
    }

    pub fn as_radians(&self) -> Angle {
        match self.units {
            AngleUnits::Degrees => Angle::new(self.value * DEG_2_RAD, AngleUnits::Degrees),
            AngleUnits::Radians => *self,
        }
    }
}

pub const DEG_2_RAD: f64 = 0.017_453_292_519_943_295;
pub const RAD_2_DEG: f64 = 57.295_779_513_082_32;
