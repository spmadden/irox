use std::collections::BTreeMap;
use std::io::Write;

use crate::{CSVError, CSVErrorType};

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
