// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Pure-Rust implementation of the Tape Archive (TAR) (aka ustar) file format as standardized in
//! IEEE 1003.1 Posix ca 1988.
//! 
//! The tar/star/ustar format goes back to 1979 (Unix 7th ed.).
//!

#![forbid(unsafe_code)]

use std::io::Read;

mod reader;
mod error;

/// Block size - 512b
pub const TBLOCK: usize = 512;
/// Name size - 100b
pub const NAMSIZ: usize = 100;
///
pub const PFXSIZ: usize = 155;

/// File mode length - 8b
pub const TMODLEN: usize = 8;
/// UID (user id) length - 8b
pub const TUIDLEN: usize = 8;
/// GID (group id) length - 8b
pub const TGIDLEN: usize = 8;
/// File size length - 12b;
pub const TSIZLEN: usize = 12;
/// File modification time length - 12b
pub const TMTMLEN: usize = 12;
/// Checksum length - 8b
pub const TCKSLEN: usize = 8;

/// File magic "ustar"
pub const TMAGIC: &str = "ustar\0";
/// Length of the magic with trailing null
pub const TMAGLEN: usize = 6;
/// Version number - "00"
pub const TVERSION: &str = "00";
/// Length of the version number
pub const TVERSLEN: usize = 2;
/// Username length - 32b
pub const TUNMLEN: usize = 32;
/// Group name length - 32b
pub const TGNMLEN: usize = 32;
/// Device Major/Minor length - 8b
pub const TDEVLEN: usize = 8;

/// File Type Flags
pub enum FileTypeFlag {
    /// Regular File
    RegType = '0' as isize,
    /// Hard Link
    LinkType = '1' as isize,
    /// Symbolic Link
    SymType = '2' as isize,
    /// Character Special
    ChrType = '3' as isize,
    /// Block Special
    BlkType = '4' as isize,
    /// Directory
    DirType = '5' as isize,
    /// FIFO (named pipe)
    FifoType = '6' as isize,
    /// Contiguous File
    ContType = '7' as isize,

    /// POSIX.1-2001 Global Extended Header
    GlobalExtendedHdr = 'g' as isize,
    /// POSIX.1-2001 Extended Header
    ExtendedHdr = 'x' as isize,

    /// Next file has a long linkname
    LongLinkName = 'K' as isize,
    /// Next file has a long name
    LongName = 'L' as isize,
}

/// ustar header.  All fields are ascii printable characters.  Valid characters
/// are ascii `0x20` through `0x7E` and nulls (`0x0`).  "Numbers" like UID, GID,
/// file size, etc are all ascii characters `0-9` (0x30-0x39).
pub struct Header {
    /// Filename - ascii characters, space padded
    pub name: [u8;NAMSIZ],
    /// File mode - ascii octal characters, space padded
    pub mode: [u8;TMODLEN],
    /// User id - ascii decimal characters, space padded
    pub uid: [u8;TUIDLEN],
    /// Group id - ascii decimal characters, space padded
    pub gid: [u8;TGIDLEN],
    /// File size in bytes
    pub size: [u8;TSIZLEN],
    /// File mod time, unix timestamp
    pub mod_time: [u8;TMTMLEN],
    /// Checksum
    pub checksum: [u8;TCKSLEN],
    /// File type, see [`FileTypeFlag`]
    pub typeflag: u8,
    /// Target of links
    pub linkname: [u8;NAMSIZ],
    /// File magic - ustar
    pub magic: [u8;TMAGLEN],
    /// File version, usually fixed to "00"
    pub version: [u8;TVERSLEN],
    /// User name
    pub username: [u8;TUNMLEN],
    /// Group name
    pub groupname: [u8;TGNMLEN],
    /// Special Device Major number
    pub device_major: [u8;TDEVLEN],
    /// Special Device Minor number 
    pub device_minor: [u8;TDEVLEN],
    /// Prefix for 'name'
    pub prefix: [u8;PFXSIZ],
    /// Filler to round off the struct to 512b
    pub filler: [u8;12],
    
}

impl Header {
    pub fn read_from<T: Read>(file: &mut T) -> Result<Header, crate::error::Error> {
        
        todo!()
    }
}