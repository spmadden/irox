// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::collections::VecDeque;
use std::io::{Error, Read, Write};

pub use counting::*;

mod counting;

///
/// Consumes data from the input stream until:
/// 1. The byte stream represented by 'search' has been found or
/// 2. The input reader returns 0 bytes read (or errors out)
///
/// Note: The input stream position is left JUST AFTER the found search string.
pub fn consume_until<T: Read>(input: &mut T, search: &[u8]) -> Result<(), Error> {
    let mut ringbuf: VecDeque<u8> = VecDeque::with_capacity(search.len());
    input.read_exact(ringbuf.as_mut_slices().0)?;

    let mut onebuf: [u8; 1] = [0; 1];
    loop {
        if ringbuf.iter().eq(search) {
            return Ok(());
        }

        if input.read(&mut onebuf)? == 0 {
            return Ok(());
        }

        ringbuf.pop_front();
        ringbuf.push_back(onebuf[0]);
    }
}

///
/// Reads from the input stream until:
/// 1. The byte stream represented by 'search' has been found or
/// 2. The input stream returns 0 bytes read (or errors out)
/// It returns all bytes read in the interim
pub fn read_until<T: Read>(input: &mut T, search: &[u8]) -> Result<Vec<u8>, Error> {
    let mut ringbuf: VecDeque<u8> = VecDeque::with_capacity(search.len());

    let mut out = Vec::new();
    let mut onebuf: [u8; 1] = [0; 1];
    loop {
        if ringbuf.iter().eq(search) {
            return Ok(out);
        }

        if input.read(&mut onebuf)? == 0 {
            return Ok(out);
        }

        if ringbuf.len() == search.len() {
            if let Some(val) = ringbuf.pop_front() {
                out.push(val);
            }
        }
        ringbuf.push_back(onebuf[0]);
    }
}

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
