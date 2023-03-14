use super::FromUnits;

#[derive(Debug, Clone, Copy, Default)]
pub enum DataSizeUnits {
    #[default]
    Bytes,
    KiloBytes,
    MegaBytes,
    GigaBytes,
    TeraBytes,
    PetaBytes,
}

macro_rules! from_units_datasize {
    ($type:ident) => {
        impl FromUnits<$type> for DataSizeUnits {
            fn from(&self, value: $type, units: Self) -> $type {
                match self {
                    DataSizeUnits::Bytes => match units {
                        DataSizeUnits::Bytes => value,
                        DataSizeUnits::KiloBytes => value / KB as $type,
                        DataSizeUnits::MegaBytes => value / MB as $type,
                        DataSizeUnits::GigaBytes => value / GB as $type,
                        DataSizeUnits::TeraBytes => value / TB as $type,
                        DataSizeUnits::PetaBytes => value / PB as $type,
                    },
                    DataSizeUnits::KiloBytes => match units {
                        DataSizeUnits::Bytes => value * KB as $type,
                        DataSizeUnits::KiloBytes => value,
                        DataSizeUnits::MegaBytes => value / KB as $type,
                        DataSizeUnits::GigaBytes => value / MB as $type,
                        DataSizeUnits::TeraBytes => value / GB as $type,
                        DataSizeUnits::PetaBytes => value / TB as $type,
                    },
                    DataSizeUnits::MegaBytes => match units {
                        DataSizeUnits::Bytes => value * MB as $type,
                        DataSizeUnits::KiloBytes => value * KB as $type,
                        DataSizeUnits::MegaBytes => value,
                        DataSizeUnits::GigaBytes => value / KB as $type,
                        DataSizeUnits::TeraBytes => value / MB as $type,
                        DataSizeUnits::PetaBytes => value / GB as $type,
                    },
                    DataSizeUnits::GigaBytes => match units {
                        DataSizeUnits::Bytes => value * GB as $type,
                        DataSizeUnits::KiloBytes => value * MB as $type,
                        DataSizeUnits::MegaBytes => value * KB as $type,
                        DataSizeUnits::GigaBytes => value,
                        DataSizeUnits::TeraBytes => value / KB as $type,
                        DataSizeUnits::PetaBytes => value / MB as $type,
                    },
                    DataSizeUnits::TeraBytes => match units {
                        DataSizeUnits::Bytes => value * TB as $type,
                        DataSizeUnits::KiloBytes => value * GB as $type,
                        DataSizeUnits::MegaBytes => value * MB as $type,
                        DataSizeUnits::GigaBytes => value * KB as $type,
                        DataSizeUnits::TeraBytes => value,
                        DataSizeUnits::PetaBytes => value / KB as $type,
                    },
                    DataSizeUnits::PetaBytes => match units {
                        DataSizeUnits::Bytes => value * PB as $type,
                        DataSizeUnits::KiloBytes => value * TB as $type,
                        DataSizeUnits::MegaBytes => value * GB as $type,
                        DataSizeUnits::GigaBytes => value * MB as $type,
                        DataSizeUnits::TeraBytes => value * KB as $type,
                        DataSizeUnits::PetaBytes => value,
                    },
                }
            }
        }
    };
}

from_units_datasize!(f32);
from_units_datasize!(f64);
from_units_datasize!(i64);
from_units_datasize!(u64);
from_units_datasize!(usize);

basic_unit!(DataSize, DataSizeUnits, Bytes);

impl DataSize {
    pub fn new_bytes(&self, value: u64) -> DataSize {
        Self::new(value as f64, DataSizeUnits::Bytes)
    }

    pub fn as_bytes(&self) -> u64 {
        match self.units {
            DataSizeUnits::Bytes => self.value as u64,
            DataSizeUnits::KiloBytes => (self.value * KB as f64) as u64,
            DataSizeUnits::MegaBytes => (self.value * MB as f64) as u64,
            DataSizeUnits::GigaBytes => (self.value * GB as f64) as u64,
            DataSizeUnits::TeraBytes => (self.value * TB as f64) as u64,
            DataSizeUnits::PetaBytes => (self.value * PB as f64) as u64,
        }
    }

    pub fn human(&self) -> String {
        human_bytes(self.as_bytes())
    }

    pub fn human_frac(&self) -> String {
        human_bytes_frac(self.as_bytes())
    }
}

pub fn human_bytes(bytes: u64) -> String {
    if bytes < KB {
        format!("{bytes} bytes")
    } else if bytes < MB {
        let val = bytes as f64 / KB as f64;
        return format!("{val:.3} KB");
    } else if bytes < GB {
        let val = bytes as f64 / MB as f64;
        return format!("{val:.3} MB");
    } else if bytes < TB {
        let val = bytes as f64 / GB as f64;
        return format!("{val:.3} GB");
    } else if bytes < PB {
        let val = bytes as f64 / TB as f64;
        return format!("{val:.3} TB");
    } else {
        let val = bytes as f64 / PB as f64;
        return format!("{val:.3} PB");
    }
}

pub fn human_bytes_frac(bytes: u64) -> String {
    if bytes < KB {
        format!("{} bytes", bytes)
    } else if bytes < MB {
        let val = bytes as f64 / KB as f64;
        return format!("{val:.3} KB");
    } else if bytes < GB {
        let val = bytes as f64 / MB as f64;
        return format!("{val:.3} MB");
    } else if bytes < TB {
        let val = bytes as f64 / GB as f64;
        return format!("{val:.3} GB");
    } else if bytes < PB {
        let val = bytes as f64 / TB as f64;
        return format!("{val:.3} TB");
    } else {
        let val = bytes as f64 / PB as f64;
        return format!("{val:.3} PB");
    }
}

pub const KB: u64 = 1024;
pub const MB: u64 = KB * 1024;
pub const GB: u64 = MB * 1024;
pub const TB: u64 = GB * 1024;
pub const PB: u64 = TB * 1024;
