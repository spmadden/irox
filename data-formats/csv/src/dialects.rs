// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! The dialects module has the different ways a CSV file (or any repeating
//! record / fields in record) file can be represented.
//!

///
/// A dialect represents the variations in how this record/field format can
/// be encoded.  The base dialect
pub trait CSVDialect {
    ///
    /// Returns the line/record separator for this tokenizer type
    /// Defaults to "\n"
    fn get_line_separators(&self) -> &[u8];

    ///
    /// Returns the field separator for this tokenizer type,
    /// Defaults to ","
    fn get_field_separators(&self) -> &[u8];
}

///
/// RFC4180 Dialect, uses the industry defaults '\n' for record separator,
/// and ',' for field separator.
#[derive(Default, Copy, Clone)]
pub struct RFC4180Dialect {}

impl CSVDialect for RFC4180Dialect {
    fn get_line_separators(&self) -> &[u8] {
        b"\n"
    }

    fn get_field_separators(&self) -> &[u8] {
        b","
    }
}

///
/// Microsoft Excel tokenizer, uses '\r\n' for the record separator.
#[derive(Default, Copy, Clone)]
pub struct ExcelDialect {}

impl CSVDialect for ExcelDialect {
    fn get_line_separators(&self) -> &[u8] {
        b"\r\n"
    }

    fn get_field_separators(&self) -> &[u8] {
        b","
    }
}

///
/// Tab dialect, uses '\n' for newlines and '\t' for the field separator.
#[derive(Default, Copy, Clone)]
pub struct UnixTabDialect {}
impl CSVDialect for UnixTabDialect {
    fn get_line_separators(&self) -> &[u8] {
        b"\n"
    }

    fn get_field_separators(&self) -> &[u8] {
        b"\t"
    }
}

///
/// Excel tab dialect, uses '\r\n' for newlines and '\t' for the field separator.
#[derive(Default, Copy, Clone)]
pub struct ExcelTabDialect {}
impl CSVDialect for ExcelTabDialect {
    fn get_line_separators(&self) -> &[u8] {
        b"\r\n"
    }

    fn get_field_separators(&self) -> &[u8] {
        b"\t"
    }
}

///
/// Allows a downstream user to specify the parameters for tokenization.
#[derive(Clone)]
pub struct CustomDialect {
    /// The end-of-line/record separators, usually '\r\n' or the like
    pub line_separators: Vec<u8>,
    /// The end-of-field separators, usually ',' for CSV
    pub field_separators: Vec<u8>,
}

impl CSVDialect for CustomDialect {
    fn get_line_separators(&self) -> &[u8] {
        self.line_separators.as_ref()
    }

    fn get_field_separators(&self) -> &[u8] {
        self.field_separators.as_ref()
    }
}

impl CustomDialect {
    pub fn new<L: AsRef<[u8]>, F: AsRef<[u8]>>(
        line_separators: L,
        field_separators: F,
    ) -> CustomDialect {
        CustomDialect {
            line_separators: line_separators.as_ref().to_owned(),
            field_separators: field_separators.as_ref().to_owned(),
        }
    }
}

impl<T: CSVDialect> From<&T> for CustomDialect {
    fn from(value: &T) -> Self {
        CustomDialect::new(value.get_line_separators(), value.get_field_separators())
    }
}
