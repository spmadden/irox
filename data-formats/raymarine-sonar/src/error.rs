use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorType {
    SQLError,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    error_type: ErrorType,
    msg: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}: {}", self.error_type, self.msg))
    }
}
impl std::error::Error for Error {}

impl Error {
    pub fn new(error_type: ErrorType, msg: String) -> Error {
        Error { error_type, msg }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        let str = value.to_string();
        Error {
            error_type: ErrorType::SQLError,
            msg: str,
        }
    }
}
