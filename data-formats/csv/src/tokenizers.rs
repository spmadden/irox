// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::io::{BufRead, BufReader, Read};

use crate::error::CSVError;

///
/// Output from the Tokenizers as they detects individual tokens from the input stream.
pub enum Token {
    Field(String),
    EndRow,
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
    reader: BufReader<T>,
    line_number: u64,
    char_number: u64,
}

impl<T: Read + Sized> BasicTokenReader<T> {
    ///
    /// Creates a new Tokenizer, consuming the underlying reader.
    pub fn new(reader: T) -> BasicTokenReader<T> {
        BasicTokenReader {
            reader: BufReader::new(reader),
            line_number: 0,
            char_number: 0,
        }
    }
}

impl<T: Read + Sized> TokenReader for BasicTokenReader<T> {
    ///
    /// Attempts to scan the line and return the immediate next set of [`Token`]s it finds.
    /// The real brunt of the processing work is done here.
    fn next_tokens(&mut self) -> Result<Option<Vec<Token>>, CSVError> {
        use crate::error::CSVErrorType;
        use std::io::ErrorKind;

        let mut output: Vec<u8> = Vec::new();

        loop {
            match crate::consume_one(&mut self.reader) {
                Ok(Some(elem)) => {
                    self.char_number += 1;
                    match elem {
                        // Comma means 'End of Field', return whatever has been found (which may be empty)
                        b',' => {
                            let out: String = String::from_utf8_lossy(&output).into();
                            return Ok(Some(vec![Token::Field(out)]));
                        }
                        // CR/LF mean 'End of Row', but repeated CR/LF characters are ignored.
                        b'\r' | b'\n' => {
                            while let Some(b'\r') | Some(b'\n') = crate::peek_one(&mut self.reader)?
                            {
                                // if it's another CR/LF, consume it.
                                self.reader.consume(1);
                            }
                            self.char_number = 0;
                            self.line_number += 1;
                            let out: String = String::from_utf8_lossy(&output).into();
                            return Ok(Some(vec![Token::Field(out), Token::EndRow]));
                        }
                        // Special case for quotes, consume all characters until we receive the next quote.
                        b'"' => {
                            while let Some(v) = crate::consume_one(&mut self.reader)? {
                                // special special case for a double quote within a quoted block,
                                // it just becomes a single double quote
                                if v == b'"' {
                                    // handle special "" within a quoted block meaning: literal quote
                                    if let Some(b'"') = crate::peek_one(&mut self.reader)? {
                                        output.push(b'"');
                                        self.reader.consume(1);
                                        continue;
                                    }
                                    break;
                                }
                                output.push(v);
                            }
                        }
                        // standard loop, it's part of the value
                        _ => output.push(elem),
                    }
                }
                Ok(None) => {
                    if !output.is_empty() {
                        let out: String = String::from_utf8_lossy(&output).into();
                        return Ok(Some(vec![Token::Field(out), Token::EndRow]));
                    }
                    return Ok(None);
                }
                Err(e) => {
                    return match e.kind() {
                        ErrorKind::UnexpectedEof => Ok(None),
                        kind => CSVError::err(
                            CSVErrorType::IOError,
                            format!(
                                "IO Error at line {} char {}: {:?}: {:?}",
                                self.line_number, self.char_number, kind, e
                            ),
                        ),
                    }
                }
            }
        }
    }
}
