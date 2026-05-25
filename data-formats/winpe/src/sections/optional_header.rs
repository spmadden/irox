// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use irox_bits::{Bits, BitsErrorKind, Error, MutBits, Seek};
use irox_structs::Struct;

#[derive(Debug, Clone)]
pub enum OptionalHeader {
    OptionalPE32Header(OptionalPE32Header),
    OptionalPEPlusHeader(OptionalPEPlusHeader),
}

#[derive(Debug, Clone)]
pub struct OptionalHeaderReader {
    pub size_of_optional_header: u16,
}
impl OptionalHeaderReader {
    pub fn write_to<T: MutBits>(&self, _out: &mut T) -> Result<(), Error> {
        todo!()
    }

    pub fn parse_from<T: Bits + Seek>(&self, input: &mut T) -> Result<OptionalHeader, Error> {
        let start = Seek::position(input)?;
        let t = input.read_le_u16()?;
        let res = match t {
            0x10B => OptionalHeader::OptionalPE32Header(OptionalPE32Header::parse_from(input)?),
            0x20B => OptionalHeader::OptionalPEPlusHeader(OptionalPEPlusHeader::parse_from(input)?),
            _ => {
                return Err(Error::new(
                    BitsErrorKind::InvalidInput,
                    "Unsupported header signature",
                ))
            }
        };
        let len = Seek::position(input)? - start;
        if len != self.size_of_optional_header as u64 {
            return Err(Error::new(
                BitsErrorKind::InvalidInput,
                "Header length mismatch!",
            ));
        }
        Ok(res)
    }
}

#[derive(Debug, Clone, Struct)]
#[little_endian]
pub struct OptionalPE32Header {
    pub standard_fields: StandardFields,
    pub base_of_data: u32,

    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win_version: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,

    pub data_directories: Tables,
}
#[derive(Debug, Clone)]
pub struct OptionalPEPlusOffsets {
    pub standard_fields: u64,

    pub image_base: u64,
    pub section_alignment: u64,
    pub file_alignment: u64,
    pub major_operating_system_version: u64,
    pub minor_operating_system_version: u64,
    pub major_image_version: u64,
    pub minor_image_version: u64,
    pub major_subsystem_version: u64,
    pub minor_subsystem_version: u64,
    pub win_version: u64,
    pub size_of_image: u64,
    pub size_of_headers: u64,
    pub checksum: u64,
    pub subsystem: u64,
    pub dll_characteristics: u64,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u64,
    pub number_of_rva_and_sizes: u64,

    pub data_directories: u64,
}

#[derive(Debug, Clone, Struct)]
#[little_endian]
pub struct OptionalPEPlusHeader {
    pub standard_fields: StandardFields,

    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win_version: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,

    pub data_directories: Tables,

    #[offsets]
    pub offsets: OptionalPEPlusOffsets,
}

#[derive(Debug, Clone, Struct)]
#[little_endian]
pub struct StandardFields {
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
}

#[derive(Debug, Default, Copy, Clone, Struct)]
#[little_endian]
pub struct DataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

#[derive(Debug, Clone, Struct)]
#[little_endian]
pub struct Tables {
    pub export_table: DataDirectory,
    pub import_table: DataDirectory,
    pub resource_table: DataDirectory,
    pub exception_table: DataDirectory,
    pub certificate_table: DataDirectory,
    pub base_relocation_table: DataDirectory,
    pub debug_table: DataDirectory,
    pub architecture_table: DataDirectory,
    pub global_ptr_table: DataDirectory,
    pub tls_table: DataDirectory,
    pub load_config_table: DataDirectory,
    pub bound_import_table: DataDirectory,
    pub import_address_table: DataDirectory,
    pub delay_import_table: DataDirectory,
    pub clr_runtime_header: DataDirectory,
    pub reserved: DataDirectory,

    #[offsets]
    pub offsets: TableOffsets,
}
#[derive(Debug, Clone)]
pub struct TableOffsets {
    pub export_table: u64,
    pub import_table: u64,
    pub resource_table: u64,
    pub exception_table: u64,
    pub certificate_table: u64,
    pub base_relocation_table: u64,
    pub debug_table: u64,
    pub architecture_table: u64,
    pub global_ptr_table: u64,
    pub tls_table: u64,
    pub load_config_table: u64,
    pub bound_import_table: u64,
    pub import_address_table: u64,
    pub delay_import_table: u64,
    pub clr_runtime_header: u64,
    pub reserved: u64,
}
