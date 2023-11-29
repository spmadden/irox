use std::collections::BTreeMap;
use std::io::Write;

use crate::{CSVError, CSVErrorType, Dialect};

///
/// Flexible CSV writer, wherein one can specify the dialect and optional column
/// headers
pub struct CSVWriter<T>
where
    T: Write + Sized,
{
    pub(crate) output: T,
    pub(crate) columns: Option<Vec<String>>,
    pub(crate) dialect: Dialect,
    pub(crate) wrote_header: bool,
}

impl<T: Write + Sized> CSVWriter<T> {
    ///
    /// Creates a new writer using the default dialect.
    #[must_use]
    pub fn new(output: T) -> Self {
        CSVWriter {
            output,
            columns: None,
            dialect: Dialect::default(),
            wrote_header: false,
        }
    }

    ///
    /// Sets the dialect to use
    #[must_use]
    pub fn with_dialect(self, dialect: Dialect) -> Self {
        CSVWriter { dialect, ..self }
    }

    ///
    /// Sets the column names to use as the header
    #[must_use]
    pub fn with_column_names(self, columns: &[&str]) -> Self {
        let columns: Vec<String> = columns.iter().map(ToString::to_string).collect();
        CSVWriter {
            columns: Some(columns),
            ..self
        }
    }

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
    #[must_use]
    pub(crate) fn make_line(&self, fields: &[String]) -> String {
        let line = fields.join(self.dialect.get_field_separators());
        format!("{line}{}", self.dialect.get_line_separators())
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
    pub fn write_fields<K: AsRef<str>, V: AsRef<str>>(
        &mut self,
        fields: &BTreeMap<K, V>,
    ) -> Result<(), CSVError> {
        self.write_header()?;
        let Some(cols) = &self.columns else {
            return CSVError::err(
                CSVErrorType::MissingHeaderError,
                "No header columns specified".to_string(),
            );
        };
        let mut out = Vec::new();
        for col in cols {
            out.push(
                fields
                    .iter()
                    .find_map(|(k, v)| {
                        if col == k.as_ref() {
                            return Some(String::from(v.as_ref()));
                        }
                        None
                    })
                    .unwrap_or_default(),
            );
        }
        let line = self.make_line(&out);
        self.output.write_all(line.as_bytes())?;
        Ok(())
    }
}
