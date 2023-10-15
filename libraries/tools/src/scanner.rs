// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

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
/// A token represents a searching string in the input data stream.  If the
/// sequence of bytes `search` is found, then the response will be `response`.
///
/// Can optionally provide an `escape_char`, which will preclude `search` from
/// matching if it is immediately preceded by that character.
///
/// Can optionally provide an `quote_char` to indicate that `search` should be
/// precluded if wrapped in those characters.
#[derive(Clone)]
pub struct Token<T: Clone> {
    search: Vec<u8>,
    response: T,
    escape_char: Option<u8>,
    quote_char: Option<QuotedChars>,
}

impl<T: Clone> Token<T> {
    pub fn new<S: AsRef<[u8]>>(search: S, response: T) -> Self {
        Token {
            search: search.as_ref().to_owned(),
            response,
            escape_char: None,
            quote_char: None,
        }
    }
    #[must_use]
    pub fn with_escape_char(self, escape: u8) -> Self {
        Token {
            search: self.search,
            response: self.response,
            quote_char: self.quote_char,
            escape_char: Some(escape),
        }
    }
    #[must_use]
    pub fn with_quote_char(self, quote_char: QuotedChars) -> Self {
        Token {
            search: self.search,
            response: self.response,
            quote_char: Some(quote_char),
            escape_char: self.escape_char,
        }
    }

    #[must_use]
    pub fn get_search(&self) -> &[u8] {
        self.search.as_ref()
    }

    #[must_use]
    pub fn get_response(&self) -> &T {
        &self.response
    }
}

///
/// Used as a return type to provide:
/// `Found` if a token was found, which token, and where
/// `EndOfData` the scanner hit the end of the buffer/EOF before finding the token
/// `NotFound` if there is no more data in the buffer
pub enum FoundToken<'a, T: Clone> {
    Found { offset: usize, token: &'a Token<T> },
    EndOfData { remaining_length: usize },
    NotFound,
}

///
/// Used as a return type to provide:
/// `Found` if the token was found, which token, and the data preceding it
/// `EndOfData` if the scanner hit EOF and found no token
/// `NotFound` if there is no more data in the boffer
pub enum ReadToken<'a, T: Clone> {
    Found { data: Vec<u8>, token: &'a Token<T> },
    EndOfData { data: Vec<u8> },
    NotFound,
}

struct TokenWorkingMem<'a, T: Clone> {
    token: &'a Token<T>,
    ringbuf: VecDeque<u8>,
    found_escape: bool,
    last_found_quote_char: Option<u8>,
    offset: usize,
}
impl<'a, T: Clone> TokenWorkingMem<'a, T> {
    pub fn new(token: &'a Token<T>) -> Self {
        TokenWorkingMem {
            token,
            ringbuf: VecDeque::with_capacity(token.search.len()),
            found_escape: false,
            last_found_quote_char: None,
            offset: 0,
        }
    }

    /// Is there any unfilled capacity?
    pub fn is_full(&self) -> bool {
        self.ringbuf.capacity() - self.ringbuf.len() == 0
    }

    /// Process the new element
    pub fn push_back(&mut self, elem: u8) {
        if !self.is_full() {
            // it's not full yet, just push the new element in and skip out.
            self.ringbuf.push_back(elem);
            return;
        }
        // we are full, grab the front element, check it for escape and quotes
        // and then append the new guy.
        let ret = self.ringbuf.pop_front();

        if let Some(first) = ret {
            self.offset += 1;
            if let Some(esc) = self.token.escape_char {
                self.found_escape = first == esc;
            }

            if let Some(quoted) = self.token.quote_char {
                //
                // the following logic is a little bit complex, because
                // SingleOrDoubleQuotes can match either ' or ", but we don't
                // want to match the opposite character, IE, a ' won't terminate
                // a " run.

                if let Some(last_char) = self.last_found_quote_char {
                    // if we're searching for the next quote character
                    if last_char == first {
                        // and we've found it - clear the flag
                        self.last_found_quote_char = None;
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
                    self.last_found_quote_char = Some(first);
                }
            }
        }

        self.ringbuf.push_back(elem);
    }

    /// Have we found the token?
    pub fn matches(&self) -> bool {
        if !self.is_full() {
            return false;
        }
        if self.found_escape {
            // escape character found, we'll never match the token.
            return false;
        }
        if self.last_found_quote_char.is_some() {
            // we're in a quoted sequence, we'll never match the token.
            return false;
        }
        self.ringbuf.iter().eq(&self.token.search)
    }
}

///
/// A Scanner is a forward lookahead struct that scans through an stream of
/// data looking for the indicated tokens.
///
/// The amount of possible forward lookahead is specified by the internal buffer
/// size of the [`BufReader`]
pub struct Scanner<T, R>
where
    T: Read + Sized,
    R: Clone,
{
    reader: BufReader<T>,
    tokens: Vec<Token<R>>,
}

impl<T: Read + Sized, R: Clone> Scanner<T, R> {
    ///
    /// Creates a scanner with the default buffer capacity, 8KB
    pub fn new(input: T, delimiters: &[Token<R>]) -> Self {
        Scanner {
            reader: BufReader::with_capacity(8 * 1024, input),
            tokens: Vec::from(delimiters),
        }
    }

    ///
    /// Creates a scanner with the specified buffer capacity
    pub fn with_max_lookahead(input: T, max_buffer: usize, delimiters: &[Token<R>]) -> Self {
        Scanner {
            reader: BufReader::with_capacity(max_buffer, input),
            tokens: Vec::from(delimiters),
        }
    }

    ///
    /// Scans through the buffer, looking for the specified token.  Returns the
    /// number of bytes in the stream needed to position the cursor to JUST BEFORE
    /// the token.  I.E., after calling `read_exact(scan_until())`, the next
    /// call to `read()` will return the token itself.
    ///
    /// Returns `Ok(N)` if it found the token in the input stream, or hit the end of the buffer without finding the token
    /// Returns `Ok(None)` if there are no additional characters to read in the buffer - we've hit EOF.
    /// Returns `Err(e)` if there's an error reading from the underlying stream
    pub fn scan_until_next(&mut self) -> Result<FoundToken<R>, std::io::Error> {
        let data = self.reader.fill_buf()?;
        if data.is_empty() {
            // EOF
            return Ok(FoundToken::NotFound);
        }

        let mut workingmem: Vec<TokenWorkingMem<R>> =
            self.tokens.iter().map(TokenWorkingMem::new).collect();
        let mut num_read = 0;
        for char in data {
            for mem in &mut workingmem {
                mem.push_back(*char);

                if mem.matches() {
                    return Ok(FoundToken::Found {
                        offset: mem.offset,
                        token: mem.token,
                    });
                }
            }

            num_read += 1;
        }
        Ok(FoundToken::EndOfData {
            remaining_length: num_read,
        })
    }

    pub fn read_next(&mut self) -> Result<ReadToken<R>, std::io::Error> {
        let data = self.reader.fill_buf()?;
        if data.is_empty() {
            // EOF
            return Ok(ReadToken::NotFound);
        }

        let mut workingmem: Vec<TokenWorkingMem<R>> =
            self.tokens.iter().map(TokenWorkingMem::new).collect();
        let mut num_read = 0;
        for char in data {
            num_read += 1;
            for mem in &mut workingmem {
                mem.push_back(*char);

                if mem.matches() {
                    if let Some(field) = data.get(0..mem.offset) {
                        let vec = Vec::from(field);
                        self.reader.consume(num_read);
                        return Ok(ReadToken::Found {
                            data: vec,
                            token: mem.token,
                        });
                    }
                }
            }
        }
        if let Some(field) = data.get(0..num_read) {
            let vec = Vec::from(field);
            self.reader.consume(num_read);
            return Ok(ReadToken::EndOfData { data: vec });
        }
        Ok(ReadToken::NotFound)
    }
}

impl<T: Read + Sized, R: Clone> Read for Scanner<T, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<T: Read + Sized, R: Clone> BufRead for Scanner<T, R> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}

impl<T: Read + Sized + Seek, R: Clone> Seek for Scanner<T, R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.reader.seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use crate::scanner::*;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum Tokens {
        Space,
        Other,
    }

    #[test]
    pub fn test_scan_until() -> Result<(), std::io::Error> {
        let data = "this is a basic test\nthis is a second line";

        let delims = &[Token::new(b" ", Tokens::Space)];

        let mut scanner = Scanner::new(data.as_bytes(), delims);

        for exp in [4, 2, 1, 5, 9, 2, 1, 6, 4] {
            match scanner.scan_until_next()? {
                FoundToken::Found { token, offset } => {
                    assert_eq!(offset, exp);
                    assert_eq!(token.response, Tokens::Space);
                    assert_ne!(token.response, Tokens::Other);
                }
                FoundToken::EndOfData { remaining_length } => {
                    assert_eq!(remaining_length, exp);
                }
                FoundToken::NotFound => {
                    panic!("None not expected")
                }
            }
            scanner.consume(exp + 1);
        }

        Ok(())
    }

    #[test]
    pub fn test_scan_escaped() -> Result<(), std::io::Error> {
        let data = "this is a basic \"escaped\\ test\nthis\" is a second line";

        let delims = &[Token::new(b" ", Tokens::Space).with_escape_char(b'\\')];
        let mut scanner = Scanner::new(data.as_bytes(), delims);

        for exp in [4, 2, 1, 5, 20, 2, 1, 6, 4] {
            match scanner.scan_until_next()? {
                FoundToken::Found { token, offset } => {
                    assert_eq!(offset, exp);
                    assert_eq!(token.response, Tokens::Space);
                    assert_ne!(token.response, Tokens::Other);
                }
                FoundToken::EndOfData { .. } => {}
                FoundToken::NotFound => {
                    panic!("None not expected")
                }
            }
            scanner.consume(exp + 1);
        }

        Ok(())
    }

    #[test]
    pub fn test_scan_quoted_double() -> Result<(), std::io::Error> {
        let data = "this is a basic \"escaped\\ test\nthis\" is a second line";
        let delims = &[Token::new(b" ", Tokens::Space).with_quote_char(QuotedChars::DoubleQuotes)];
        let mut scanner = Scanner::new(data.as_bytes(), delims);

        for exp in [4, 2, 1, 5, 20, 2, 1, 6, 4] {
            match scanner.scan_until_next()? {
                FoundToken::Found { token, offset } => {
                    assert_eq!(offset, exp);
                    assert_eq!(token.response, Tokens::Space);
                    assert_ne!(token.response, Tokens::Other);
                }
                FoundToken::EndOfData { .. } => {}
                FoundToken::NotFound => {
                    panic!("None not expected")
                }
            }
            scanner.consume(exp + 1);
        }

        Ok(())
    }
    #[test]
    pub fn test_scan_quoted_single() -> Result<(), std::io::Error> {
        let data = "this is a basic \'escaped\\ test\nthis\' is a second line";
        let delims = &[Token::new(b" ", Tokens::Space).with_quote_char(QuotedChars::SingleQuotes)];

        let mut scanner = Scanner::new(data.as_bytes(), delims);

        for exp in [4, 2, 1, 5, 20, 2, 1, 6, 4] {
            match scanner.scan_until_next()? {
                FoundToken::Found { token, offset } => {
                    assert_eq!(offset, exp);
                    assert_eq!(token.response, Tokens::Space);
                    assert_ne!(token.response, Tokens::Other);
                }
                FoundToken::EndOfData { .. } => {}
                FoundToken::NotFound => {
                    panic!("None not expected")
                }
            }
            scanner.consume(exp + 1);
        }

        Ok(())
    }

    #[test]
    pub fn test_scan_quoted_other() -> Result<(), std::io::Error> {
        let data = "this is a basic |escaped\\ test\nthis| is a second line";

        let delims = &[Token::new(b" ", Tokens::Space).with_quote_char(QuotedChars::Other(b'|'))];
        let mut scanner = Scanner::new(data.as_bytes(), delims);

        for exp in [4, 2, 1, 5, 20, 2, 1, 6, 4] {
            match scanner.scan_until_next()? {
                FoundToken::Found { token, offset } => {
                    assert_eq!(offset, exp);
                    assert_eq!(token.response, Tokens::Space);
                    assert_ne!(token.response, Tokens::Other);
                }
                FoundToken::EndOfData { .. } => {}
                FoundToken::NotFound => {
                    panic!("None not expected")
                }
            }
            scanner.consume(exp + 1);
        }

        Ok(())
    }

    #[test]
    pub fn test_scan_quoted_both() -> Result<(), std::io::Error> {
        let data = "this is a \"more\' advanced\" \'escaped\\ \"test\nthis\' is a second line";
        let delims =
            &[Token::new(b" ", Tokens::Space).with_quote_char(QuotedChars::SingleOrDoubleQuotes)];
        let mut scanner = Scanner::new(data.as_bytes(), delims);

        for exp in [4, 2, 1, 16, 21, 2, 1, 6, 4] {
            match scanner.scan_until_next()? {
                FoundToken::Found { token, offset } => {
                    assert_eq!(offset, exp);
                    assert_eq!(token.response, Tokens::Space);
                    assert_ne!(token.response, Tokens::Other);
                }
                FoundToken::EndOfData { .. } => {}
                FoundToken::NotFound => {
                    panic!("None not expected")
                }
            }
            scanner.consume(exp + 1);
        }

        Ok(())
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum CSVTokens {
        Field,
        Newline,
    }
    #[test]
    pub fn test_scan_csv() -> Result<(), std::io::Error> {
        let data = "name1,name2,name3,name4\r\nescaped\\,value1,\"quoted,value2\",\'quoted,value3\',\"long value\"\n\n";

        let delims = &[
            Token::new(b",", CSVTokens::Field)
                .with_escape_char(b'\\')
                .with_quote_char(QuotedChars::SingleOrDoubleQuotes),
            Token::new(b"\r\n", CSVTokens::Newline),
            Token::new(b"\n", CSVTokens::Newline),
        ];
        let mut scanner = Scanner::new(data.as_bytes(), delims);

        let exp = &[
            (5, CSVTokens::Field),
            (5, CSVTokens::Field),
            (5, CSVTokens::Field),
            (5, CSVTokens::Newline),
            (15, CSVTokens::Field),
            (15, CSVTokens::Field),
            (15, CSVTokens::Field),
            (12, CSVTokens::Newline),
            (0, CSVTokens::Newline),
        ];

        let mut ctr = 0;
        for (exp_off, exp_ret) in exp {
            let to_consume = match scanner.scan_until_next()? {
                FoundToken::Found { token, offset } => {
                    assert_eq!(offset, *exp_off, "{ctr}{:?}", token.response);
                    assert_eq!(token.response, *exp_ret, "{ctr}");
                    token.search.len()
                }
                FoundToken::EndOfData { .. } => {
                    panic!("EOD Not expected {ctr}")
                }
                FoundToken::NotFound => {
                    panic!("None not expected {ctr}")
                }
            };
            let consumed = exp_off + to_consume;
            scanner.consume(consumed);
            ctr += 1;
        }

        Ok(())
    }

    #[test]
    pub fn test_three_delim() -> Result<(), std::io::Error> {
        let data = "this is a test of the testing test";
        let mut scanner = Scanner::new(data.as_bytes(), &[Token::new("test", "test")]);
        for (exp_off, exp) in &[(10, "test"), (8, "test"), (4, "test")] {
            let to_consume = match scanner.scan_until_next()? {
                FoundToken::Found { offset, token } => {
                    assert_eq!(*exp_off, offset);
                    assert_eq!(*exp, token.response);
                    token.search.len()
                }
                FoundToken::EndOfData { remaining_length } => {
                    assert_eq!(remaining_length, 0);
                    remaining_length
                }
                FoundToken::NotFound => {
                    panic!("Not found");
                }
            };
            scanner.consume(exp_off + to_consume);
        }
        Ok(())
    }
}
