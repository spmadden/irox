// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};

///
/// This struct purely exists to implement 'Display' for a borrowed vec, whose elements implement ['std::fmt::Display']
pub struct PrettyVec<'a, T>(pub &'a Vec<T>);

impl<'a, T> Display for PrettyVec<'a, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vals: Vec<String> = self.0.iter().map(|f| format!("{f}")).collect();
        if f.alternate() {
            f.write_fmt(format_args!("{vals:#?}"))
        } else {
            f.write_fmt(format_args!("{vals:?}"))
        }
    }
}
