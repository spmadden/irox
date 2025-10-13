// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}

impl std::error::Error for Error {}

macro_rules! impl_error {
    ($msg:literal, $($typ:ty)+) => {
        impl From<$($typ)+> for Error {
            fn from(value: $($typ)+) -> Error {
                Error {
                    msg: format!("{value}")
                }
            }
        }
    };
}

impl_error!("IOError", std::io::Error);
impl_error!("Git", std::path::StripPrefixError);
#[cfg(all(feature = "git", not(target_arch = "wasm32")))]
impl_error!("Git", git2::Error);
#[cfg(all(feature = "git", not(target_arch = "wasm32")))]
impl_error!("Git", irox_git_tools::Error);
