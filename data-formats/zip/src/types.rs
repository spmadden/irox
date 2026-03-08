// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_tools::hex;
extern crate alloc;
use alloc::boxed::Box;

#[derive(Default, Debug, Clone)]
pub struct Headers {
    pub local_file_header: LocalFileHeader,
    pub encryption_header: Option<()>,
    pub data_descriptor: Option<DataDescriptor>,
}

#[derive(Default, Debug, Clone)]
pub struct LocalFileHeader {
    pub version_needed_to_extract: u16,
    pub gp_bit_flag: u16,
    pub compression_method: u16,
    pub last_mod_file_time: u16,
    pub last_mod_file_date: u16,
    pub crc32: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub filename_length: u16,
    pub extra_field_length: u16,
    pub filename: Box<[u8]>,
    pub extra_field: Option<Box<[u8]>>,
}
impl LocalFileHeader {
    pub const SIGNATURE: [u8; 4] = hex!("504B0304");
}

#[derive(Default, Debug, Clone)]
pub struct DataDescriptor {
    pub crc32: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
}

#[derive(Default, Debug, Clone)]
pub struct CentralDirectoryHeader {
    pub version_made_by: u16,
    pub version_needed_to_extract: u16,
    pub gp_bit_flag: u16,
    pub compression_method: u16,
    pub last_mod_file_time: u16,
    pub last_mod_file_date: u16,
    pub crc32: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub filename_length: u16,
    pub extra_field_length: u16,
    pub file_comment_length: u16,
    pub disk_number_start: u16,
    pub internal_file_attributes: u16,
    pub external_file_attributes: u32,
    pub relative_localheader_offset: u32,
    pub filename: Box<[u8]>,
    pub extra_field: Option<Box<[u8]>>,
    pub file_comment: Option<Box<[u8]>>,
}
impl CentralDirectoryHeader {
    pub const SIGNATURE: [u8; 4] = hex!("504B0102");
}

#[derive(Default, Debug, Clone)]
pub struct EndOfCentralDirectory {
    pub this_disk_number: u16,
    pub disk_with_central_directory: u16,
    pub this_disk_num_entries: u16,
    pub total_num_entries: u16,
    pub central_directory_length: u32,
    pub central_directory_start_disk_offset: u32,
    pub file_comment_length: u16,
    pub file_comment: Option<Box<[u8]>>,
}
impl EndOfCentralDirectory {
    pub const SIGNATURE: [u8; 4] = hex!("504B0506");
}

#[derive(Default, Debug, Clone)]
pub struct EndOfCentral64DirectoryLocator {
    pub num_disk_with_64central_directory: u32,
    pub relative_offset_64central: u64,
    pub total_num_disks: u32,
}
impl EndOfCentral64DirectoryLocator {
    pub const SIGNATURE: [u8; 4] = hex!("504B0607");
}

#[derive(Debug, Clone)]
pub struct Record {
    pub start_offset: u64,
    pub length: u64,
    pub element: RecordType,
}
#[derive(Debug, Clone)]
pub enum RecordType {
    InnerFileEntry(Headers),
    CentralDirectory(CentralDirectoryHeader),
    Zip64CentralDirectory(),
    Zip64CentralDirectoryEnd(EndOfCentral64DirectoryLocator),
    CentralDirectoryEnd(EndOfCentralDirectory),
}

#[derive(Default, Debug, Clone)]
pub struct ZipFile {
    pub records: Vec<Record>,
}
