// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::collections::VecDeque;
use std::fmt::{Display, Formatter, UpperHex, Write};

///
/// This struct purely exists to implement 'Display' for a borrowed vec, whose elements implement [`std::fmt::Display`]
pub struct PrettyVec<'a, T>(pub &'a Vec<T>);

pub struct PrettyVecDeque<'a, T>(pub &'a VecDeque<T>);

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

impl<'a, T> UpperHex for PrettyVec<'a, T>
where
    T: UpperHex,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for val in self.0 {
            buf.write_fmt(format_args!("{val:0X}"))?;
        }
        f.write_fmt(format_args!("{buf}"))
    }
}

impl<'a, T> UpperHex for PrettyVecDeque<'a, T>
where
    T: UpperHex,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for val in self.0 {
            buf.write_fmt(format_args!("{val:0X}"))?;
        }
        f.write_fmt(format_args!("{buf}"))
    }
}
