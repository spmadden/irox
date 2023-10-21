// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::io::Read;

use irox_tools::scanner as sc;
use irox_tools::scanner::{QuotedChars, ReadToken, Scanner};

use crate::error::CSVError;
use crate::Dialect;

///
/// Output from the Tokenizers as they detects individual tokens from the input stream.
#[derive(Debug, Clone)]
pub enum Token {
    Field(String),
    EndRow,
    Comment(String),
}

#[derive(Clone)]
enum InnerToken {
    Field,
    Newline,
    Comment,
}

///
/// A Token Reader reads tokens from input stream
pub trait TokenReader {
    ///
    /// Attempts to scan the line and return the immediate next set of [`Token`]s it finds.
    /// Call this function repeatedly until it returns `Ok(None)` or errors.
    fn next_tokens(&mut self) -> Result<Option<Vec<Token>>, CSVError>;
}

///
/// A Token Writer writes the set of specified tokens to the output stream
pub trait TokenWriter {
    ///
    /// Attempts to write these tokens to the output stream.
    fn write_tokens(&mut self, tokens: &[Token]) -> Result<(), CSVError>;
}

///
/// Scans the provided input stream and outputs [`Token`]s as it detects them.
pub struct BasicTokenReader<T>
where
    T: Read + Sized,
{
    scanner: Scanner<T, InnerToken>,
}

impl<T: Read + Sized> BasicTokenReader<T> {
    ///
    /// Creates a new Tokenizer using the default RFC4180 Dialect, consuming the
    /// underlying reader.
    pub fn new(reader: T) -> Self {
        let dialect = Dialect::default();
        Self::dialect(reader, dialect)
    }

    ///
    /// Token reader using the specified dialect
    pub fn dialect(reader: T, dialect: Dialect) -> Self {
        let delims = &[
            sc::Token::new(dialect.get_field_separators(), InnerToken::Field)
                .with_quote_char(QuotedChars::DoubleQuotes),
            sc::Token::new(dialect.get_line_separators(), InnerToken::Newline)
                .with_quote_char(QuotedChars::DoubleQuotes),
            sc::Token::new(dialect.get_comment_chars(), InnerToken::Comment),
        ];
        Self {
            scanner: Scanner::new(reader, delims),
        }
    }
}

impl<T: Read + Sized> TokenReader for BasicTokenReader<T> {
    ///
    /// Attempts to scan the line and return the immediate next set of [`Token`]s it finds.
    /// The real brunt of the processing work is done here.
    fn next_tokens(&mut self) -> Result<Option<Vec<Token>>, CSVError> {
        match self.scanner.read_next()? {
            ReadToken::Found { data, token } => {
                let name = String::from_utf8_lossy(&data).to_string();
                match token.get_response() {
                    InnerToken::Field => Ok(Some(vec![Token::Field(name)])),
                    InnerToken::Newline => Ok(Some(vec![Token::Field(name), Token::EndRow])),
                    InnerToken::Comment => Ok(Some(vec![Token::Comment(name)])),
                }
            }
            ReadToken::EndOfData { data } => Ok(Some(vec![
                Token::Field(String::from_utf8_lossy(&data).to_string()),
                Token::EndRow,
            ])),
            ReadToken::NotFound => Ok(None),
        }
    }
}
