// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::io::Write;

mod types;

pub struct Writer<T>
where
    T: Write + Sized,
{
    output: T,
    columns: Option<Vec<String>>,
}
