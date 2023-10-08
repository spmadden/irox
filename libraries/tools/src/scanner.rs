// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

use crate::read::read_exact_into_sized;

///
/// What characters are considered "quotes"
#[derive(Debug, Copy, Clone, Default)]
pub enum QuotedChars {
    /// Matches " or '
    #[default]
    SingleOrDoubleQuotes,
    /// Matches only "
    DoubleQuotes,
    /// Matches only '
    SingleQuotes,
    /// Matches only the specified character
    Other(u8),
}

///
/// A Scanner is a forward lookahead struct that scans through an stream of
/// data looking for the indicated tokens.
///
/// The amount of possible forward lookahead is specified by the internal buffer
/// size of the [`BufReader`]
pub struct Scanner<T>
where
    T: Read + Sized,
{
    reader: BufReader<T>,
}

impl<T: Read + Sized> Scanner<T> {
    ///
    /// Creates a scanner with the default buffer capacity, 8KB
    pub fn new(input: T) -> Self {
        Scanner {
            reader: BufReader::with_capacity(8 * 1024, input),
        }
    }

    ///
    /// Creates a scanner with the specified buffer capacity
    pub fn with_max_lookahead(input: T, max_buffer: usize) -> Self {
        Scanner {
            reader: BufReader::with_capacity(max_buffer, input),
        }
    }

    ///
    /// Scans through the buffer, looking for the specified token.  Returns the
    /// number of bytes in the stream needed to position the cursor to JUST BEFORE
    /// the token.  I.E., after calling `read_exact(scan_until(token))`, the next
    /// call to `read()` will return the token itself.
    ///
    /// Returns `Ok(N)` if it found the token in the input stream, or hit the end of the buffer without finding the token
    /// Returns `Ok(None)` if there are no additional characters to read in the buffer - we've hit EOF.
    /// Returns `Err(e)` if there's an error reading from the underlying stream
    pub fn scan_until(&mut self, token: &[u8]) -> Result<Option<usize>, std::io::Error> {
        let mut data = self.reader.fill_buf()?;
        if data.is_empty() {
            // EOF
            return Ok(None);
        }
        let mut ringbuf: VecDeque<u8> = VecDeque::with_capacity(token.len());
        read_exact_into_sized(&mut data, &mut ringbuf, token.len())?;

        let mut num_read = 0;
        for char in data {
            if ringbuf.iter().eq(token) {
                return Ok(Some(num_read));
            }

            ringbuf.pop_front();
            ringbuf.push_back(*char);

            num_read += 1;
        }
        Ok(Some(num_read + token.len()))
    }

    ///
    /// Operates the same as [`scan_until`] but ignores any token if it's preceded
    /// by the escape character
    pub fn scan_until_except_escaped(
        &mut self,
        token: &[u8],
        escape_char: u8,
    ) -> Result<Option<usize>, std::io::Error> {
        let mut data = self.reader.fill_buf()?;
        if data.is_empty() {
            // EOF
            return Ok(None);
        }
        let mut ringbuf: VecDeque<u8> = VecDeque::with_capacity(token.len());
        read_exact_into_sized(&mut data, &mut ringbuf, token.len())?;

        let mut num_read = 0;
        let mut found_escape = false;
        for char in data {
            if ringbuf.iter().eq(token) && !found_escape {
                return Ok(Some(num_read));
            }

            if let Some(first) = ringbuf.pop_front() {
                found_escape = first == escape_char;
            }

            ringbuf.push_back(*char);

            num_read += 1;
        }
        Ok(Some(num_read + token.len()))
    }

    ///
    /// Operates the same as [`scan_until`] but ignores tokens within quotes
    pub fn scan_until_except_quoted(
        &mut self,
        token: &[u8],
        quoted: QuotedChars,
    ) -> Result<Option<usize>, std::io::Error> {
        let mut data = self.reader.fill_buf()?;
        if data.is_empty() {
            // EOF
            return Ok(None);
        }
        let mut ringbuf: VecDeque<u8> = VecDeque::with_capacity(token.len());
        read_exact_into_sized(&mut data, &mut ringbuf, token.len())?;

        let mut num_read = 0;
        let mut last_found_quote_char: Option<u8> = None;
        for char in data {
            if ringbuf.iter().eq(token) && last_found_quote_char.is_none() {
                // if we've matched the token, and we're not in a quoted set.
                return Ok(Some(num_read));
            }

            if let Some(first) = ringbuf.pop_front() {
                //
                // the following logic is a little bit complex, because
                // SingleOrDoubleQuotes can match either ' or ", but we don't
                // want to match the opposite character, IE, a ' won't terminate
                // a " run.

                if let Some(last_char) = last_found_quote_char {
                    // if we're searching for the next quote character
                    if last_char == first {
                        // and we've found it - clear the flag
                        last_found_quote_char = None;
                    }
                } else if match quoted {
                    // if we're not searching, check against the possible quote
                    // chars to see if we've found a start
                    QuotedChars::SingleOrDoubleQuotes => first == b'\'' || first == b'\"',
                    QuotedChars::DoubleQuotes => first == b'\"',
                    QuotedChars::SingleQuotes => first == b'\'',
                    QuotedChars::Other(o) => first == o,
                } {
                    // we've found a start, flag it.
                    last_found_quote_char = Some(first);
                }
            }

            ringbuf.push_back(*char);

            num_read += 1;
        }
        Ok(Some(num_read + token.len()))
    }
}

impl<T: Read + Sized> Read for Scanner<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<T: Read + Sized> BufRead for Scanner<T> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}

impl<T: Read + Sized + Seek> Seek for Scanner<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.reader.seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use crate::scanner::{QuotedChars, Scanner};

    #[test]
    pub fn test_scan_until() -> Result<(), std::io::Error> {
        let data = "this is a basic test\nthis is a second line";

        let mut scanner = Scanner::new(data.as_bytes());

        for offset in [4, 2, 1, 5, 9, 2, 1, 6, 4] {
            match scanner.scan_until(b" ")? {
                Some(e) => assert_eq!(offset, e),
                None => panic!("None not expected"),
            }
            scanner.consume(offset + 1);
        }

        Ok(())
    }

    #[test]
    pub fn test_scan_escaped() -> Result<(), std::io::Error> {
        let data = "this is a basic \"escaped\\ test\nthis\" is a second line";

        let mut scanner = Scanner::new(data.as_bytes());

        for offset in [4, 2, 1, 5, 20, 2, 1, 6, 4] {
            match scanner.scan_until_except_escaped(b" ", b'\\')? {
                Some(e) => assert_eq!(offset, e),
                None => panic!("None not expected"),
            }
            scanner.consume(offset + 1);
        }

        Ok(())
    }

    #[test]
    pub fn test_scan_quoted_double() -> Result<(), std::io::Error> {
        let data = "this is a basic \"escaped\\ test\nthis\" is a second line";

        let mut scanner = Scanner::new(data.as_bytes());

        for offset in [4, 2, 1, 5, 20, 2, 1, 6, 4] {
            match scanner.scan_until_except_quoted(b" ", QuotedChars::DoubleQuotes)? {
                Some(e) => assert_eq!(offset, e),
                None => panic!("None not expected"),
            }
            scanner.consume(offset + 1);
        }

        Ok(())
    }
    #[test]
    pub fn test_scan_quoted_single() -> Result<(), std::io::Error> {
        let data = "this is a basic \'escaped\\ test\nthis\' is a second line";

        let mut scanner = Scanner::new(data.as_bytes());

        for offset in [4, 2, 1, 5, 20, 2, 1, 6, 4] {
            match scanner.scan_until_except_quoted(b" ", QuotedChars::SingleQuotes)? {
                Some(e) => assert_eq!(offset, e),
                None => panic!("None not expected"),
            }
            scanner.consume(offset + 1);
        }

        Ok(())
    }

    #[test]
    pub fn test_scan_quoted_other() -> Result<(), std::io::Error> {
        let data = "this is a basic |escaped\\ test\nthis| is a second line";

        let mut scanner = Scanner::new(data.as_bytes());

        for offset in [4, 2, 1, 5, 20, 2, 1, 6, 4] {
            match scanner.scan_until_except_quoted(b" ", QuotedChars::Other(b'|'))? {
                Some(e) => assert_eq!(offset, e),
                None => panic!("None not expected"),
            }
            scanner.consume(offset + 1);
        }

        Ok(())
    }

    #[test]
    pub fn test_scan_quoted_both() -> Result<(), std::io::Error> {
        let data = "this is a \"more\' advanced\" \'escaped\\ \"test\nthis\' is a second line";

        let mut scanner = Scanner::new(data.as_bytes());

        for offset in [4, 2, 1, 16, 21, 2, 1, 6, 4] {
            match scanner.scan_until_except_quoted(b" ", QuotedChars::SingleOrDoubleQuotes)? {
                Some(e) => assert_eq!(offset, e),
                None => panic!("None not expected"),
            }
            scanner.consume(offset + 1);
        }

        Ok(())
    }
}
