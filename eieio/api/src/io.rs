// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! I/O Traits - ReadFromXYZ and WriteToXYZ

use crate::error::Error;
use std::fmt::{Debug, Formatter};
use std::io::{Read, Write};
use std::path::Path;

use crate::Message;

///
/// Reads a set of messages from the specified file path on disk.  The format of the file is
/// implementation-specific.
pub trait ReadFromFile {
    ///
    /// Reads a set of messages from the specified file path on disk
    fn read_from_file(&mut self, path: &Path) -> Result<Vec<Message>, Error>;
}

///
/// Reads a set of messages from the provided byte array slice.  The format of the bytes in memory
/// is implementation-specific.
pub trait ReadFromBytes {
    ///
    /// Reads a set of messages from the provided byte array slice
    fn read_from_bytes(&mut self, bytes: &[u8]) -> Result<Vec<Message>, Error>;
}

///
/// Reads a set of messages from the provided directory path on disk.  The names of the files,
/// how they are organized, and the formats of the files are all implementation-specific.
pub trait ReadFromDirectory {
    ///
    /// Reads a set of messages from the provided directory path on disk.
    fn read_from_directory(&mut self, directory: &Path) -> Result<Vec<Message>, Error>;
}

///
/// Reads a set of messages from the provided Read trait.  The format of the bytes provided is
/// implementation-specific.
pub trait ReadFromRead {
    ///
    /// Reads a set of messages from the provided Read trait.
    fn read_from_read(&mut self, read: &dyn Read) -> Result<Vec<Message>, Error>;
}
///
/// Reads a set of messages from the provided String. The format of the string provided is
/// implementation-specific.
pub trait ReadFromString {
    ///
    /// Reads a set of messages from the provided String.
    fn read_from_string(&mut self, read: &str) -> Result<Vec<Message>, Error>;
}

///
/// A struct providing a convenient shuttle of different sources of message data parsers.
#[derive(Default)]
pub struct SupportedReaders<'a> {
    from_file: Option<&'a dyn ReadFromFile>,
    from_bytes: Option<&'a dyn ReadFromBytes>,
    from_dir: Option<&'a dyn ReadFromDirectory>,
    from_read: Option<&'a dyn ReadFromRead>,
    from_string: Option<&'a dyn ReadFromString>,
}

impl SupportedReaders<'_> {
    ///
    /// Returns [`Some(ReadFromFile)`] If this [`crate::Codec`] supports reading from a file.
    pub fn file(&self) -> Option<&dyn ReadFromFile> {
        self.from_file
    }

    ///
    /// Returns [`Some(ReadFromBytes)`] if this [`crate::Codec`] supports parsing bytes in memory.
    pub fn bytes(&self) -> Option<&dyn ReadFromBytes> {
        self.from_bytes
    }

    ///
    /// Returns [`Some(ReadFromDirectory)`] if this [`crate::Codec`] supports reading files in a directory
    pub fn dir(&self) -> Option<&dyn ReadFromDirectory> {
        self.from_dir
    }

    ///
    /// Returns [`Some(ReadFromRead)`] if this [`crate::Codec`] supports reading bytes from a [`Read`]
    /// trait
    pub fn read(&self) -> Option<&dyn ReadFromRead> {
        self.from_read
    }

    ///
    /// Returns [`Some(ReadFromString)`] if this [`crate::Codec`] supports parsing a [`String`]
    pub fn string(&self) -> Option<&dyn ReadFromString> {
        self.from_string
    }
}

impl Debug for SupportedReaders<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("supported readers")
            .field("file", &self.file().is_some())
            .field("bytes", &self.bytes().is_some())
            .field("dir", &self.dir().is_some())
            .field("read", &self.read().is_some())
            .field("string", &self.string().is_some())
            .finish()
    }
}

///
/// A convenient way to allow a [`crate::Codec`] to build a [`SupportedReaders`] struct
#[derive(Default)]
pub struct SupportedReadersBuilder<'a> {
    readers: SupportedReaders<'a>,
}
impl<'a> SupportedReadersBuilder<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_file(mut self, file: &'a dyn ReadFromFile) -> Self {
        self.readers.from_file = Some(file);
        self
    }

    #[must_use]
    pub fn with_dir(mut self, dir: &'a dyn ReadFromDirectory) -> Self {
        self.readers.from_dir = Some(dir);
        self
    }

    #[must_use]
    pub fn with_bytes(mut self, bytes: &'a dyn ReadFromBytes) -> Self {
        self.readers.from_bytes = Some(bytes);
        self
    }

    #[must_use]
    pub fn with_read(mut self, read: &'a dyn ReadFromRead) -> Self {
        self.readers.from_read = Some(read);
        self
    }

    #[must_use]
    pub fn with_string(mut self, string: &'a dyn ReadFromString) -> Self {
        self.readers.from_string = Some(string);
        self
    }

    #[must_use]
    pub fn build(self) -> SupportedReaders<'a> {
        self.readers
    }
}

///
/// Writes this particular message to the path on disk specified.  The specific format of the bytes
/// is implementation-specific.
pub trait WriteToFile {
    ///
    /// Writes this particular message to the path on disk specified.
    fn write_to_file(&self, file: &Path) -> Result<(), Error>;
}
///
/// Writes this particular message to the directory specified.  It is assumed the implementation
/// will know how to properly format the file name.  The specific format of the directory structure
/// the file names, and the byte encoding of the files is implementation-specific.
pub trait WriteToDirectory {
    ///
    /// Writes this particular message to the directory specified.
    fn write_to_dir(&self, dir: &Path) -> Result<(), Error>;
}
///
/// Writes this particular message to bytes in memory.  The format of the encoded bytes is
/// implementation-specific.
pub trait WriteToBytes {
    ///
    /// Writes this particular message to bytes in memory.
    fn get_bytes(&self) -> Result<Vec<u8>, Error>;
}
///
/// Writes this particular message to the [`Write`] trait provided.  The format of the encoded bytes
/// is implementation-specific.
pub trait WriteToWrite {
    ///
    /// Writes this particular message to the [`Write`] trait provided.
    fn write(&self, out: &mut dyn Write) -> Result<(), Error>;
}

///
/// Writes this particular message to a [`String`] in-memory.  The format of the encoded string is
/// implementation-specific.
pub trait WriteToString {
    ///
    /// Writes this particular message to a [`String`] in-memory.
    fn write_to_string(&self) -> Result<String, Error>;
}

///
/// A struct providing a convenient shuttle of different destination of message data encoders.
#[derive(Default)]
pub struct SupportedWriters<'a> {
    file: Option<&'a dyn WriteToFile>,
    dir: Option<&'a dyn WriteToDirectory>,
    bytes: Option<&'a dyn WriteToBytes>,
    write: Option<&'a dyn WriteToWrite>,
    string: Option<&'a dyn WriteToString>,
}
impl SupportedWriters<'_> {
    ///
    /// Returns [`Some(WriteToFile)`] if this message supports writing itself to a file.
    pub fn file(&self) -> Option<&dyn WriteToFile> {
        self.file
    }

    ///
    /// Returns [`Some(WriteToDirectory)`] if this message supports writing itself to a directory.
    pub fn dir(&self) -> Option<&dyn WriteToDirectory> {
        self.dir
    }

    ///
    /// Returns [`Some(WriteToBytes)`] if this message supports writing itself to bytes in-memory.
    pub fn bytes(&self) -> Option<&dyn WriteToBytes> {
        self.bytes
    }

    /// Returns [`Some(WriteToWrite)`] if this message supports writing itself to a [`Write`]
    pub fn write(&self) -> Option<&dyn WriteToWrite> {
        self.write
    }

    ///
    /// Returns [`Some(WriteToString)`] if this message supports encoding itself as a [`String`]
    pub fn string(&self) -> Option<&dyn WriteToString> {
        self.string
    }
}

///
/// A convenient way to allow a message to build a [`SupportedWriters`] struct
#[derive(Default)]
pub struct SupportedWritersBuilder<'a> {
    writers: SupportedWriters<'a>,
}
impl<'a> SupportedWritersBuilder<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_file(mut self, file: &'a dyn WriteToFile) -> Self {
        self.writers.file = Some(file);
        self
    }

    #[must_use]
    pub fn with_dir(mut self, dir: &'a dyn WriteToDirectory) -> Self {
        self.writers.dir = Some(dir);
        self
    }

    #[must_use]
    pub fn with_bytes(mut self, bytes: &'a dyn WriteToBytes) -> Self {
        self.writers.bytes = Some(bytes);
        self
    }

    #[must_use]
    pub fn with_write(mut self, write: &'a dyn WriteToWrite) -> Self {
        self.writers.write = Some(write);
        self
    }

    #[must_use]
    pub fn with_string(mut self, string: &'a dyn WriteToString) -> Self {
        self.writers.string = Some(string);
        self
    }

    #[must_use]
    pub fn build(self) -> SupportedWriters<'a> {
        self.writers
    }
}
