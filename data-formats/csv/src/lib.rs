// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]

use std::{
    collections::BTreeMap,
    io::{Read, Write},
};

use error::CSVError;

pub mod error;

pub struct Writer<T>
where
    T: Write + Sized,
{
    pub(crate) output: T,
    pub(crate) columns: Option<Vec<String>>,
    pub(crate) column_separator: Option<String>,
    pub(crate) newlines: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct WriterBuilder {
    columns: Option<Vec<String>>,
    newlines: Option<String>,
    column_separator: Option<String>,
}

impl WriterBuilder {
    #[must_use]
    pub fn new() -> WriterBuilder {
        Default::default()
    }

    #[must_use]
    pub fn with_columns(mut self, columns: Vec<String>) -> Self {
        self.columns = Some(columns);
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

    pub fn build<T: Write + Sized>(self, output: T) -> Writer<T> {
        Writer {
            output,
            columns: self.columns,
            newlines: self.newlines,
            column_separator: self.column_separator,
        }
    }
}

pub enum Token {
    Field(String),
    Newline,
}

pub struct Tokenizer<T>
where
    T: Read + Sized,
{
    reader: T,
    line_number: u64,
    char_number: u64,
    skip_next_lf: bool,
}

impl<T: Read + Sized> Tokenizer<T> {
    pub fn new(reader: T) -> Tokenizer<T> {
        Tokenizer {
            reader,
            line_number: 0,
            char_number: 0,
            skip_next_lf: false,
        }
    }
    pub fn next_tokens(&mut self) -> Result<Option<Vec<Token>>, CSVError> {
        use std::io::ErrorKind;

        let mut output: Vec<u8> = Vec::new();
        let mut buffer: [u8; 1] = [0; 1];

        loop {
            match self.reader.read_exact(&mut buffer) {
                Ok(_) => {
                    self.char_number += 1;
                    let elem = buffer[0];
                    match elem {
                        b',' => {
                            let out: String = String::from_utf8_lossy(&output).into();
                            return Ok(Some(vec![Token::Field(out)]));
                        }
                        b'\r' | b'\n' => {
                            if elem == b'\r' {
                                self.skip_next_lf = true;
                            } else if self.skip_next_lf {
                                self.skip_next_lf = false;
                                continue;
                            }
                            self.char_number = 0;
                            self.line_number += 1;
                            let out: String = String::from_utf8_lossy(&output).into();
                            return Ok(Some(vec![Token::Field(out), Token::Newline]));
                        }
                        _ => {
                            self.skip_next_lf = false;
                            output.push(buffer[0])
                        }
                    }
                }
                Err(e) => {
                    return match e.kind() {
                        ErrorKind::UnexpectedEof => Ok(None),
                        kind => CSVError::err(
                            error::CSVErrorType::IOError,
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

pub struct CSVReader<T>
where
    T: Read + Sized,
{
    tokenizer: Tokenizer<T>,
}

impl<T: Read + Sized> CSVReader<T> {
    pub fn new(reader: T) -> CSVReader<T> {
        CSVReader {
            tokenizer: Tokenizer::new(reader),
        }
    }

    pub fn read_line(&mut self) -> Result<Option<Vec<String>>, CSVError> {
        let mut out: Vec<String> = Vec::new();

        loop {
            if let Some(toks) = self.tokenizer.next_tokens()? {
                for tok in toks {
                    match tok {
                        Token::Field(f) => out.push(f),
                        Token::Newline => return Ok(Some(out)),
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

pub struct CSVMapReader<T>
where
    T: Read + Sized,
{
    reader: CSVReader<T>,
    keys: Vec<String>,
}

impl<T: Read + Sized> CSVMapReader<T> {
    pub fn new(read: T) -> Result<CSVMapReader<T>, CSVError> {
        let mut reader = CSVReader::new(read);
        let keys = reader.read_line()?;
        match keys {
            Some(keys) => Ok(CSVMapReader { reader, keys }),
            None => CSVError::err(
                error::CSVErrorType::MissingHeaderError,
                "Missing header or empty file".to_string(),
            ),
        }
    }

    pub fn next_row(&mut self) -> Result<Option<Row>, CSVError> {
        let data = self.reader.read_line()?;
        let Some(data) = data else {
            return Ok(None);
        };
        let hdrlen = self.keys.len();
        let datalen = data.len();
        if hdrlen != datalen {
            return CSVError::err(
                error::CSVErrorType::HeaderDataMismatchError,
                format!("Headers length ({hdrlen}) != data length ({datalen})"),
            );
        }

        Ok(Some(Row {
            keys: self.keys.clone(),
            data,
        }))
    }

    pub fn for_each<F: FnMut(Row)>(mut self, mut func: F) -> Result<(), CSVError> {
        while let Some(row) = self.next_row()? {
            func(row);
        }
        Ok(())
    }
}

pub struct Row {
    keys: Vec<String>,
    data: Vec<String>,
}

impl Row {
    pub fn as_map(self) -> Result<BTreeMap<String, String>, CSVError> {
        let mut out: BTreeMap<String, String> = BTreeMap::new();
        for (k, v) in self.as_items() {
            if let Some(_elem) = out.insert(k.clone(), v) {
                return CSVError::err(
                    error::CSVErrorType::DuplicateKeyInHeaderError,
                    format!("Duplicate key in header detected: {k}"),
                );
            }
        }
        Ok(out)
    }

    #[must_use]
    pub fn as_map_lossy(self) -> BTreeMap<String, String> {
        BTreeMap::from_iter(self.as_items())
    }

    #[must_use]
    pub fn as_items(self) -> Vec<(String, String)> {
        self.keys.into_iter().zip(self.data).collect()
    }
}
