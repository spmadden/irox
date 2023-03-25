use std::{fmt::Display, string::FromUtf8Error, str::Utf8Error, num::ParseIntError};

#[derive(Debug)]
pub struct Error {
    message : String
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error {message : message.into()}
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("GitError: {}", self.message))
    }
}

impl std::error::Error for Error{

}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error { message : format!("{} : {}", err.kind(), err.to_string())}
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error { message : err.to_string()}
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error { message : err.to_string()}
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error { message : format!("{:?} : {}", err.kind(), err.to_string())}
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error {message}
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Error { message : err.to_string()}
    }
}

