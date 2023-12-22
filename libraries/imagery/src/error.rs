// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ImageError {
    msg: String
}

impl Display for ImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImageError: {}", self.msg)
    }
}

impl std::error::Error for ImageError {

}

macro_rules! impl_error {
    ($typ:path) => {

    };
}
