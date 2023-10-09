// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! The dialects module has the different ways a CSV file (or any repeating
//! record / fields in record) file can be represented.
//!

///
/// A dialect represents the variations in how this record/field format can
/// be encoded.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Dialect {
    line_separators: &'static str,
    field_separators: &'static str,
    comment_chars: &'static str,
}
impl Default for Dialect {
    fn default() -> Self {
        RFC4180_DIALECT
    }
}

impl Dialect {
    pub const fn new(
        line_separators: &'static str,
        field_separators: &'static str,
        comment_chars: &'static str,
    ) -> Dialect {
        Dialect {
            line_separators,
            field_separators,
            comment_chars,
        }
    }

    ///
    /// Returns the line/record separator for this tokenizer type
    /// Defaults to "\n"
    #[must_use]
    pub const fn get_line_separators(&self) -> &str {
        self.line_separators
    }

    ///
    /// Returns the field separator for this tokenizer type,
    /// Defaults to ","
    #[must_use]
    pub const fn get_field_separators(&self) -> &str {
        self.field_separators
    }

    ///
    /// Returns the optional comment character for this tokenizer type,
    /// Defaults to `None`
    #[must_use]
    pub const fn get_comment_chars(&self) -> &str {
        self.comment_chars
    }
}

///
/// RFC4180 Dialect, uses the industry defaults '\r\n' for record separator,
/// and ',' for field separator.
pub const RFC4180_DIALECT: Dialect = Dialect::new("\r\n", ",", "#");

///
/// Microsoft Excel tokenizer, effectively the same as RFC4180.
pub const EXCEL_DIALECT: Dialect = RFC4180_DIALECT;

///
/// Standard unix dialect, uses '\n' instead of CRLF for line separators.
pub const UNIX_DIALECT: Dialect = Dialect::new("\n", ",", "#");

///
/// Tab dialect, uses '\n' for newlines and '\t' for the field separator.
pub const UNIX_TAB_DIALECT: Dialect = Dialect::new("\n", "\t", "#");

///
/// Excel tab dialect, uses '\r\n' for newlines and '\t' for the field separator.
pub const EXCEL_TAB_DIALECT: Dialect = Dialect::new("\r\n", "\t", "#");

///
/// Piped Field Dialect, uses vertical pipes '|' for the field separators
pub const PIPE_FIELD_DIALECT: Dialect = Dialect::new("\n", "|", "#");
