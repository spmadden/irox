use std::option::Option;
use std::fmt::{Debug};

#[derive(Debug)]
pub enum ErrorKind {
    Serialization,
    Deserialization,
    Validation,
    Other
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub info: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind, info: Option<String>) -> Error
    {
        Error{kind, info}
    }
}

impl<T> From<ciborium::de::Error<T>> for Error where T: Debug {
    fn from(error: ciborium::de::Error<T>) -> Self {
        let info = format!("{:?}", error);
        Error::new(ErrorKind::Deserialization, Option::Some(info))
    }
}

impl<T> From<ciborium::ser::Error<T>> for Error where T: Debug {
    fn from(error: ciborium::ser::Error<T>) -> Self {
        let info = format!("{:?}", error);
        Error::new(ErrorKind::Serialization, Option::Some(info))
    }
}