// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

pub type Error = BitsError;
pub type ErrorKind = BitsErrorKind;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BitsError {
    kind: BitsErrorKind,
    msg: &'static str,
}

impl BitsError {
    pub fn new(kind: BitsErrorKind, msg: &'static str) -> Self {
        BitsError { kind, msg }
    }

    pub fn kind(&self) -> BitsErrorKind {
        self.kind
    }

    pub fn msg(&self) -> &'static str {
        self.msg
    }
}

impl From<BitsErrorKind> for BitsError {
    fn from(kind: BitsErrorKind) -> Self {
        BitsError {
            kind,
            msg: match kind {
                BitsErrorKind::InvalidData => "Invalid Data",
                BitsErrorKind::UnexpectedEof => "Unexpected EOF",
                BitsErrorKind::FormatError => "Unspecified Formatting Error",
                BitsErrorKind::OutOfMemory => "Out of Memory",

                BitsErrorKind::NotFound => "Not Found",
                BitsErrorKind::PermissionDenied => "Permission Denied",
                BitsErrorKind::ConnectionRefused => "Connection Refused",
                BitsErrorKind::ConnectionReset => "Connection Reset",
                BitsErrorKind::ConnectionAborted => "Connection Aborted",
                BitsErrorKind::AddrInUse => "Address In Use",
                BitsErrorKind::AddrNotAvailable => "Address Not Available",
                BitsErrorKind::BrokenPipe => "Broken Pipe",
                BitsErrorKind::AlreadyExists => "Already Exists",
                BitsErrorKind::WouldBlock => "Would Block",
                BitsErrorKind::InvalidInput => "Invalid Input",
                BitsErrorKind::TimedOut => "Timed Out",
                BitsErrorKind::WriteZero => "Write Zero",
                BitsErrorKind::Interrupted => "Interrupted",
                BitsErrorKind::NotConnected => "Not Connected",
                BitsErrorKind::Unsupported => "Unsupported",
                BitsErrorKind::Other => "Other",
            },
        }
    }
}

impl From<BitsError> for core::fmt::Error {
    fn from(_kind: BitsError) -> Self {
        core::fmt::Error
    }
}

impl From<core::fmt::Error> for BitsError {
    fn from(_value: core::fmt::Error) -> Self {
        BitsErrorKind::FormatError.into()
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for BitsError {
    fn from(value: std::io::Error) -> Self {
        BitsError {
            kind: value.kind().into(),
            msg: "IO Error",
        }
    }
}

#[cfg(feature = "std")]
impl From<BitsError> for std::io::Error {
    fn from(value: BitsError) -> Self {
        Into::<std::io::ErrorKind>::into(value.kind).into()
    }
}

#[cfg(feature = "std")]
impl From<std::io::ErrorKind> for BitsErrorKind {
    fn from(value: std::io::ErrorKind) -> Self {
        use std::io::ErrorKind;
        match value {
            ErrorKind::NotFound => BitsErrorKind::NotFound,
            ErrorKind::PermissionDenied => BitsErrorKind::PermissionDenied,
            ErrorKind::ConnectionRefused => BitsErrorKind::ConnectionRefused,
            ErrorKind::ConnectionReset => BitsErrorKind::ConnectionReset,
            ErrorKind::ConnectionAborted => BitsErrorKind::ConnectionAborted,
            ErrorKind::NotConnected => BitsErrorKind::NotConnected,
            ErrorKind::AddrInUse => BitsErrorKind::AddrInUse,
            ErrorKind::AddrNotAvailable => BitsErrorKind::AddrNotAvailable,
            ErrorKind::BrokenPipe => BitsErrorKind::BrokenPipe,
            ErrorKind::AlreadyExists => BitsErrorKind::AlreadyExists,
            ErrorKind::WouldBlock => BitsErrorKind::WouldBlock,
            ErrorKind::InvalidInput => BitsErrorKind::InvalidInput,
            ErrorKind::InvalidData => BitsErrorKind::InvalidData,
            ErrorKind::TimedOut => BitsErrorKind::TimedOut,
            ErrorKind::WriteZero => BitsErrorKind::WriteZero,
            ErrorKind::Interrupted => BitsErrorKind::Interrupted,
            ErrorKind::Unsupported => BitsErrorKind::Unsupported,
            ErrorKind::UnexpectedEof => BitsErrorKind::UnexpectedEof,
            ErrorKind::OutOfMemory => BitsErrorKind::OutOfMemory,
            _ => BitsErrorKind::Other,
        }
    }
}

#[cfg(feature = "std")]
impl From<BitsErrorKind> for std::io::ErrorKind {
    fn from(value: BitsErrorKind) -> Self {
        use std::io::ErrorKind;
        match value {
            BitsErrorKind::InvalidData => ErrorKind::InvalidData,
            BitsErrorKind::UnexpectedEof => ErrorKind::UnexpectedEof,
            BitsErrorKind::OutOfMemory => ErrorKind::OutOfMemory,
            BitsErrorKind::NotFound => ErrorKind::NotFound,
            BitsErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            BitsErrorKind::ConnectionRefused => ErrorKind::ConnectionRefused,
            BitsErrorKind::ConnectionReset => ErrorKind::ConnectionReset,
            BitsErrorKind::ConnectionAborted => ErrorKind::ConnectionAborted,
            BitsErrorKind::AddrInUse => ErrorKind::AddrInUse,
            BitsErrorKind::AddrNotAvailable => ErrorKind::AddrNotAvailable,
            BitsErrorKind::BrokenPipe => ErrorKind::BrokenPipe,
            BitsErrorKind::AlreadyExists => ErrorKind::AlreadyExists,
            BitsErrorKind::WouldBlock => ErrorKind::WouldBlock,
            BitsErrorKind::InvalidInput => ErrorKind::InvalidInput,
            BitsErrorKind::TimedOut => ErrorKind::TimedOut,
            BitsErrorKind::WriteZero => ErrorKind::WriteZero,
            BitsErrorKind::Interrupted => ErrorKind::Interrupted,
            BitsErrorKind::Unsupported => ErrorKind::Unsupported,
            BitsErrorKind::NotConnected => ErrorKind::NotConnected,
            _ => ErrorKind::Other,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BitsErrorKind {
    InvalidData,
    UnexpectedEof,
    FormatError,
    OutOfMemory,
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    TimedOut,
    WriteZero,
    Interrupted,
    NotConnected,
    Unsupported,
    Other,
}

#[cfg(feature = "std")]
impl std::error::Error for BitsError {}

impl core::fmt::Display for BitsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BitsError({:?}): {}", self.kind, self.msg)
    }
}
