// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]

use std::io::BufRead;

pub use dialects::*;
pub use error::*;
pub use reader::*;
pub use tokenizers::*;
pub use writer::*;

mod dialects;
mod error;
mod reader;
mod tokenizers;
mod writer;

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
