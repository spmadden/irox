// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! A [`Codec`] is a trait that provides `encode` and `decode` to convert to/from different byte
//! encoding formats
//!

use std::io::{Error, Read, Write};

/// Something that can encode or decode bytes to bytes
pub trait Codec {
    /// Encodes the input, writing to output.  Returns the total number of bytes written.
    fn encode<I: Read, O: Write>(&self, input: I, output: &mut O) -> Result<usize, Error>;
    /// Decodes the input, writing to output.  Returns the total number of bytes written.
    fn decode<I: Read, O: Write>(&self, input: I, output: &mut O) -> Result<usize, Error>;

    /// Reads the entirety of the input in the raw format, and produces the encoded UTF-8 results
    /// in an owned string.
    fn encode_to_str<I: Read>(&self, input: I) -> Result<String, Error> {
        let vec = self.encode_to_vec(input)?;
        Ok(String::from_utf8_lossy(vec.as_slice()).to_string())
    }
    /// Reads the entirety of the input in the raw format, and produces the encoded results in an
    /// owned Vec
    fn encode_to_vec<I: Read>(&self, input: I) -> Result<Vec<u8>, Error> {
        let mut vec = Vec::new();
        self.encode(input, &mut vec)?;
        Ok(vec)
    }
    /// Reads the entirety of input in coded format, and produces the results in an owned UTF-8 encoded
    /// string, dropping any non-UTF-8 characters.
    fn decode_to_str_lossy<I: Read>(&self, input: I) -> Result<String, Error> {
        let vec = self.decode_to_vec(input)?;
        Ok(String::from_utf8_lossy(&vec).to_string())
    }
    /// Reads the entirety of input in coded format, and produces the results in an owned Vec
    fn decode_to_vec<I: Read>(&self, input: I) -> Result<Vec<u8>, Error> {
        let mut vec = Vec::new();
        self.decode(input, &mut vec)?;
        Ok(vec)
    }
}
