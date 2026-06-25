use core::fmt::Display;
use core::fmt::Formatter;
use irox_bits::BitsError;

#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            msg: msg.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<BitsError> for Error {
    fn from(value: BitsError) -> Self {
        Error {
            msg: format!("BitsError: {value}"),
        }
    }
}

impl core::error::Error for Error {}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            msg: format!("IOE: {value}"),
        }
    }
}
