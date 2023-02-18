use std::{io::{Read, Seek}, fs::read};

use bytes::{Buf, Bytes};

use crate::error::Error;

#[derive(Debug, Clone, Default)]
pub struct Header {
    /// The header string: "SQLite format 3\000" 
    pub header: String,

    /// The database page size in bytes. Must be a power of two between 512 
    /// and 32768 inclusive, or the value 1 representing a page size of 65536. 
    pub page_size: u16,

    /// File format write version. 1 for legacy; 2 for WAL. 
    pub write_version: u8,

    /// File format read version. 1 for legacy; 2 for WAL. 
    pub read_version: u8,

    /// Bytes of unused "reserved" space at the end of each page. Usually 0. 
    pub reserved_len: u8,

    /// Maximum embedded payload fraction. Must be 64. 
    pub max_emb_payload_fraction: u8,

    /// Minimum embedded payload fraction. Must be 32. 
    pub min_emb_payload_fraction: u8,

    /// Leaf payload fraction. Must be 32. 
    pub leaf_payload_fraction: u8,

    /// File change counter. 
    pub file_change_counter: u32,

    /// Size of the database file in pages. The "in-header database size". 
    pub page_count: u32,

    /// Page number of the first freelist trunk page. 
    pub first_freelist_page_id: u32,

    /// Total number of freelist pages. 
    pub freelist_page_count: u32,

    /// The schema cookie. 
    pub schema_cookie: u32,

    /// The schema format number. Supported schema formats are 1, 2, 3, and 4. 
    pub schema_format_number: u32,
    
    /// Default page cache size. 
    pub default_page_cache_size: u32,

    /// The page number of the largest root b-tree page when in auto-vacuum or 
    /// incremental-vacuum modes, or zero otherwise. 
    pub auto_vacuum_max_leaf_page: u32,

    /// The database text encoding. A value of 1 means UTF-8. A value of 2 
    /// means UTF-16le. A value of 3 means UTF-16be. 
    pub text_encoding: u32,

    /// The "user version" as read and set by the user_version pragma. 
    pub user_version: u32,

    /// True (non-zero) for incremental-vacuum mode. False (zero) otherwise. 
    pub incremental_vacuum_mode: u32,

    /// The "Application ID" set by PRAGMA application_id. 
    pub application_id: u32,

    // skip 5 u32s

    pub version_valid_for: u32,
    pub sqlite_version_number: u32,
}

impl Header {
    pub fn read_from<T>(reader: &mut T) -> Result<Header, Error>
    where
        T: Read + Seek,
    {
        let mut hdr: [u8; 100] = [0; 100];

        reader.rewind()?;
        reader.read_exact(&mut hdr)?;

        let mut buf = hdr.as_slice();

        let mut out = Header::default();
        out.header = String::from_utf8_lossy(&buf[..16]).to_string();
        buf.advance(16);
        out.page_size = buf.get_u16();
        out.write_version = buf.get_u8();
        out.read_version = buf.get_u8();
        out.reserved_len = buf.get_u8();
        out.max_emb_payload_fraction = buf.get_u8();
        out.min_emb_payload_fraction = buf.get_u8();
        out.leaf_payload_fraction = buf.get_u8();
        out.file_change_counter = buf.get_u32();
        out.page_count = buf.get_u32();
        out.first_freelist_page_id = buf.get_u32();
        out.freelist_page_count = buf.get_u32();
        out.schema_cookie = buf.get_u32();
        out.schema_format_number = buf.get_u32();
        out.default_page_cache_size = buf.get_u32();
        out.auto_vacuum_max_leaf_page = buf.get_u32();
        out.text_encoding = buf.get_u32();
        out.user_version = buf.get_u32();
        out.incremental_vacuum_mode = buf.get_u32();
        out.application_id = buf.get_u32();

        buf.advance(20);

        out.version_valid_for = buf.get_u32();
        out.sqlite_version_number = buf.get_u32();

        Ok(out)
    }
}
