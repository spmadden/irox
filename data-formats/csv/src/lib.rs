// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]

use std::io::BufRead;
use std::{
    collections::BTreeMap,
    io::{Read, Write},
};

pub use dialects::*;
pub use error::*;
pub use tokenizers::*;

mod dialects;
mod error;
mod tokenizers;

///
/// Flexible CSV writer, wherein one can specify
pub struct CSVWriter<T>
where
    T: Write + Sized,
{
    pub(crate) output: T,
    pub(crate) columns: Option<Vec<String>>,
    pub(crate) column_separator: Option<String>,
    pub(crate) newlines: Option<String>,
    pub(crate) wrote_header: bool,
}

impl<T: Write + Sized> CSVWriter<T> {
    ///
    /// Ensures that the header (if specified and present) is written to the file.
    pub fn write_header(&mut self) -> Result<(), CSVError> {
        if self.wrote_header {
            return Ok(());
        }
        let Some(cols) = &self.columns else {
            self.wrote_header = true;
            return Ok(());
        };

        let line = self.make_line(cols);
        self.output.write_all(line.as_bytes())?;
        self.wrote_header = true;
        Ok(())
    }

    ///
    /// Serializes the fields in iteration order using the optionally specified Column Separator and
    /// newline character(s)
    pub(crate) fn make_line(&self, fields: &[String]) -> String {
        let sep = match &self.column_separator {
            Some(sep) => sep.as_str(),
            None => ",",
        };
        let newlines = match &self.newlines {
            Some(nl) => nl.as_str(),
            None => "\n",
        };
        let line = fields.join(sep);
        format!("{line}{newlines}")
    }

    ///
    /// Raw low-level write of a set of fields to this file in simple iteration order.  This does
    /// NOT check against previous lines to ensure the fields are the same length as priors.
    pub fn write_line<R: AsRef<str>>(&mut self, fields: &[R]) -> Result<(), CSVError> {
        self.write_header()?;
        let fields: Vec<String> = fields.iter().map(|f| f.as_ref().to_string()).collect();
        let line = self.make_line(fields.as_slice());
        self.output.write_all(line.as_bytes())?;
        Ok(())
    }

    ///
    /// Write the set of fields to the CSV file.  You must have already provided a set of headers/columns
    /// or else this function will fail with a [`CSVErrorType::MissingHeaderError`].
    ///
    /// It will write the fields in the order defined by the columns.  
    ///
    /// Note:  It is NOT required for the fields map to have every header/column within it.  Any
    /// missing fields will be replaced with an empty string.
    pub fn write_fields(&mut self, fields: &BTreeMap<String, String>) -> Result<(), CSVError> {
        self.write_header()?;
        let Some(cols) = &self.columns else {
            return CSVError::err(
                CSVErrorType::MissingHeaderError,
                "No header columns specified".to_string(),
            );
        };
        let mut out = Vec::new();
        for col in cols {
            let Some(val) = fields.get(col) else {
                out.push(String::new());
                continue;
            };
            out.push(val.to_string().clone());
        }
        let line = self.make_line(&out);
        self.output.write_all(line.as_bytes())?;
        Ok(())
    }
}

///
/// Helper builder to lazily create a [`CSVWriter`]
#[derive(Debug, Clone, Default)]
pub struct CSVWriterBuilder {
    columns: Option<Vec<String>>,
    newlines: Option<String>,
    column_separator: Option<String>,
}

impl CSVWriterBuilder {
    ///
    /// Start here.
    #[must_use]
    pub fn new() -> CSVWriterBuilder {
        Default::default()
    }

    ///
    /// Specify a set of Headers to use.  Without calling
    #[must_use]
    pub fn with_columns<T: ToString>(mut self, columns: &[T]) -> Self {
        self.columns = Some(columns.iter().map(ToString::to_string).collect());
        self
    }

    #[must_use]
    pub fn with_newlines(mut self, newlines: String) -> Self {
        self.newlines = Some(newlines);
        self
    }

    #[must_use]
    pub fn with_column_separator(mut self, column_separator: String) -> Self {
        self.column_separator = Some(column_separator);
        self
    }

    pub fn build<T: Write + Sized>(self, output: T) -> CSVWriter<T> {
        CSVWriter {
            output,
            columns: self.columns,
            newlines: self.newlines,
            column_separator: self.column_separator,
            wrote_header: false,
        }
    }
}

///
/// Consume a single byte from the underlying reader.
fn consume_one<T: BufRead>(mut reader: &mut T) -> Result<Option<u8>, std::io::Error> {
    let Some(val) = peek_one(&mut reader)? else {
        return Ok(None);
    };

    reader.consume(1);
    Ok(Some(val))
}

///
/// Reads a single byte from the underlying reader, but does NOT consume it.  Repeated calls to this
/// function will return the same value.
fn peek_one<T: BufRead>(reader: &mut T) -> Result<Option<u8>, std::io::Error> {
    let val = {
        let buf = reader.fill_buf()?;
        let Some(val) = buf.first() else {
            return Ok(None);
        };
        *val
    };

    Ok(Some(val))
}

///
/// Incredibly basic CSV reader.
///
/// Has some equivalent functionality as String.split(","), except it handles quoted entries.
pub struct CSVReader<T>
where
    T: Read + Sized,
{
    tokenizer: BasicTokenReader<T>,
}

impl<T: Read + Sized> CSVReader<T> {
    ///
    /// Create a new CSV Reader from the input.  Accepts anything that implements [`Read`]
    pub fn new(reader: T) -> CSVReader<T> {
        CSVReader {
            tokenizer: BasicTokenReader::new(reader),
        }
    }

    ///
    /// Read and parse a single line from the CSV file.
    ///
    /// Will return [`Result::Ok(None)`] upon EOF.
    /// Will return [`Result::Err(CSVError)`] upon any I/O error.
    /// Will return [`Result::Ok(Option::Some(Vec<String>))`] upon success, with each element within
    /// the line separated inside of the innermost [`Vec<String>`]
    pub fn read_line(&mut self) -> Result<Option<Vec<String>>, CSVError> {
        let mut out: Vec<String> = Vec::new();

        loop {
            if let Some(toks) = self.tokenizer.next_tokens()? {
                for tok in toks {
                    match tok {
                        Token::Field(f) => out.push(f),
                        Token::EndRow => return Ok(Some(out)),
                    }
                }
            } else {
                if !out.is_empty() {
                    return Ok(Some(out));
                }
                return Ok(None);
            }
        }
    }
}

///
/// Returns each row as a Key => Value Mapping, rather than a simple list of values.
///
/// CSVMapReader has more validation than [`CSVReader`], as it REQUIRES that each line in the
/// csv file have the same number of elements as the header.
pub struct CSVMapReader<T>
where
    T: Read + Sized,
{
    reader: CSVReader<T>,
    keys: Vec<String>,
}

impl<T: Read + Sized> CSVMapReader<T> {
    ///
    /// Creates a new [`CSVMapReader`]
    ///
    /// Will return [`Result::Ok(CSVMapReader)`] if it can read the CSV's header.
    /// Will return [`Result::Err(CSVError)`] if any I/O Error, or no header.
    pub fn new(read: T) -> Result<CSVMapReader<T>, CSVError> {
        let mut reader = CSVReader::new(read);
        let keys = reader.read_line()?;
        match keys {
            Some(keys) => Ok(CSVMapReader { reader, keys }),
            None => CSVError::err(
                CSVErrorType::MissingHeaderError,
                "Missing header or empty file".to_string(),
            ),
        }
    }

    ///
    /// Maybe return a single row from the CSV file.
    ///
    /// Will return [`std::result::Result::Ok(None)`] upon EOF
    /// Will return [`std::result::Result::Err(CSVError)`] upon underlying I/O error, or if the
    /// particular row doesn't have enough elements to match up against the header.
    pub fn next_row(&mut self) -> Result<Option<Row>, CSVError> {
        let data = self.reader.read_line()?;
        let Some(data) = data else {
            return Ok(None);
        };
        let hdrlen = self.keys.len();
        let datalen = data.len();
        if hdrlen != datalen {
            return CSVError::err(
                CSVErrorType::HeaderDataMismatchError,
                format!("Headers length ({hdrlen}) != data length ({datalen})"),
            );
        }

        Ok(Some(Row {
            keys: self.keys.clone(),
            data,
        }))
    }

    ///
    /// Apply the specified function on each element of the read CSV file.  This WILL iteratively
    /// consume the underlying reader, and will continue until the reader exhausts.
    pub fn for_each<F: FnMut(Row)>(mut self, mut func: F) -> Result<(), CSVError> {
        while let Some(row) = self.next_row()? {
            func(row);
        }
        Ok(())
    }
}

///
/// A row represents a single Map line from a CSV table
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Row {
    /// A list of the Map Keys (may be repeats!)
    keys: Vec<String>,

    /// A list of the row values (may be repeats!)
    data: Vec<String>,
}

impl Row {
    ///
    /// Converts this row into a BTreeMap<String, String>.
    ///
    /// This WILL return a [`Err`] if there are duplicate keys
    pub fn into_map(self) -> Result<BTreeMap<String, String>, CSVError> {
        let mut out: BTreeMap<String, String> = BTreeMap::new();
        for (k, v) in self.into_items() {
            if let Some(_elem) = out.insert(k.clone(), v) {
                return CSVError::err(
                    CSVErrorType::DuplicateKeyInHeaderError,
                    format!("Duplicate key in header detected: {k}"),
                );
            }
        }
        Ok(out)
    }

    ///
    /// Convert into a [`BTreeMap<String, String>`].
    ///
    /// Unlike [`into_map`], this function will overwrite any previous keys with those found later in
    /// the row.
    #[must_use]
    pub fn into_map_lossy(self) -> BTreeMap<String, String> {
        BTreeMap::from_iter(self.into_items())
    }

    ///
    /// Converts into a [`std::vec::Vec<(String, String)>`], pairing each key with it's associated value
    #[must_use]
    pub fn into_items(self) -> Vec<(String, String)> {
        self.keys.into_iter().zip(self.data).collect()
    }
}
