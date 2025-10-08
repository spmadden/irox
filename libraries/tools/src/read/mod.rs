// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Helper functions around [`std::io::Read`]

extern crate alloc;
use std::io::{Error, Read, Write};

pub use conv::*;
pub use counting::*;
#[cfg(all(feature = "std", any(unix, windows)))]
pub use pagefile::*;
pub use readerator::*;

pub use buffer::*;
mod buffer;

#[cfg(all(feature = "std", any(unix, windows)))]
mod multi_stream;
#[cfg(all(feature = "std", any(unix, windows)))]
pub use multi_stream::*;

mod conv;
mod counting;
#[cfg(all(feature = "std", any(unix, windows)))]
mod pagefile;
mod readerator;

///
/// Reads the exact amount of bytes into an array and returns it
/// ```
/// # use irox_tools::read::read_exact;
/// # let mut input : Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// # let mut input = input.as_slice();
///
/// let buf : [u8;10] = read_exact(&mut input).expect("Expected at least 10 bytes.");
/// ```
pub fn read_exact<T: Read, const N: usize>(input: &mut T) -> Result<[u8; N], Error> {
    let mut buf: [u8; N] = [0; N];
    input.read_exact(&mut buf)?;
    Ok(buf)
}

///
/// Reads the exact amount of bytes specified into a vector, and returns it
pub fn read_exact_vec<T: Read>(input: &mut T, size: usize) -> Result<Vec<u8>, Error> {
    let mut buf: Vec<u8> = vec![0; size];
    input.read_exact(buf.as_mut_slice())?;
    Ok(buf)
}

///
/// Reads the exact amount of bytes specified and writes it into the target output
pub fn read_exact_into<R: Read, W: Write, const N: usize>(
    input: &mut R,
    output: &mut W,
) -> Result<(), Error> {
    let mut buf: [u8; N] = [0; N];
    input.read_exact(&mut buf)?;
    output.write_all(&buf)
}

///
/// Reads the exact amount of bytes specified and writes it into the target output
pub fn read_exact_into_sized<R: Read, W: Write>(
    input: &mut R,
    output: &mut W,
    size: usize,
) -> Result<(), Error> {
    let mut buf: Vec<u8> = vec![0; size];
    input.read_exact(&mut buf)?;
    output.write_all(&buf)
}
