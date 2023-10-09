// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! The dialects module has the different ways a CSV file (or any repeating
//! record / fields in record) file can be represented.
//!

///
/// A dialect represents the variations in how this record/field format can
/// be encoded.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Dialect {
    line_separators: &'static str,
    field_separators: &'static str,
}
impl Default for Dialect {
    fn default() -> Self {
        RFC4180_DIALECT
    }
}

impl Dialect {
    pub const fn new(line_separators: &'static str, field_separators: &'static str) -> Dialect {
        Dialect {
            line_separators,
            field_separators,
        }
    }

    ///
    /// Returns the line/record separator for this tokenizer type
    /// Defaults to "\n"
    pub const fn get_line_separators(&self) -> &str {
        self.line_separators
    }

    ///
    /// Returns the field separator for this tokenizer type,
    /// Defaults to ","
    pub const fn get_field_separators(&self) -> &str {
        self.field_separators
    }
}

///
/// RFC4180 Dialect, uses the industry defaults '\n' for record separator,
/// and ',' for field separator.
pub const RFC4180_DIALECT: Dialect = Dialect::new("\n", ",");

///
/// Microsoft Excel tokenizer, uses '\r\n' for the record separator.
pub const EXCEL_DIALECT: Dialect = Dialect::new("\r\n", ",");

///
/// Tab dialect, uses '\n' for newlines and '\t' for the field separator.
pub const UNIX_TAB_DIALECT: Dialect = Dialect::new("\n", "\t");

///
/// Excel tab dialect, uses '\r\n' for newlines and '\t' for the field separator.
pub const EXCEL_TAB_DIALECT: Dialect = Dialect::new("\r\n", "\t");

///
/// Piped Field Dialect, uses vertical pipes '|' for the field separators
pub const PIPE_FIELD_DIALECT: Dialect = Dialect::new("\n", "|");
