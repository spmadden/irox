use std::collections::BTreeMap;
use std::io::Read;

use crate::{BasicTokenReader, CSVError, CSVErrorType, Dialect, Token, TokenReader};

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
    /// Creates a new CSV reader based on the specified dialect.  Accepts any
    /// [`Read`]er and consumes it.
    pub fn dialect(reader: T, dialect: Dialect) -> CSVReader<T> {
        CSVReader {
            tokenizer: BasicTokenReader::dialect(reader, dialect),
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

        let mut in_a_comment = false;
        loop {
            if let Some(toks) = self.tokenizer.next_tokens()? {
                for tok in toks {
                    match tok {
                        Token::Field(f) => {
                            if !in_a_comment {
                                out.push(f);
                            }
                        }
                        Token::EndRow => {
                            if in_a_comment {
                                in_a_comment = false;
                            } else {
                                return Ok(Some(out));
                            }
                        }
                        Token::Comment(f) => {
                            // only a comment if it's the first token of a line
                            in_a_comment = out.is_empty() && f.is_empty();
                            if !in_a_comment {
                                out.push(f);
                            }
                        }
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
        Self::dialect(read, Dialect::default())
    }

    ///
    /// Creates a new [`CSVMapReader`] using the specified dialect
    ///
    /// Will return [`Result::Ok(CSVMapReader)`] if it can read the CSV's header.
    /// Will return [`Result::Err(CSVError)`] if any I/O Error, or no header.
    pub fn dialect(read: T, dialect: Dialect) -> Result<Self, CSVError> {
        let mut reader = CSVReader::dialect(read, dialect);
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
