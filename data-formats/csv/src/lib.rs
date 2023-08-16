// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::io::{Read, Write};

mod types;

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
    pub fn new() -> WriterBuilder {
        Default::default()
    }

    pub fn with_columns(mut self, columns: Vec<String>) -> Self {
        self.columns = Some(columns);
        self
    }

    pub fn with_newlines(mut self, newlines: String) -> Self {
        self.newlines = Some(newlines);
        self
    }

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

pub struct Tokenizer {}
