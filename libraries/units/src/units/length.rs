#[derive(Debug, Clone, Copy)]
pub enum LengthUnits {
    Meters,
}

basic_unit!(Length, LengthUnits);
