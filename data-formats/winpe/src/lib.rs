// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

//!
//! PE File Format
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use crate::sections::*;
use irox_bits::{Bits, BitsErrorKind, Error, MutBits, Seek, SeekFrom};
use irox_structs::Struct;
use irox_tools::hash::Hasher;
use std::fs::File;

mod sections;

#[cfg(feature = "authenticode")]
pub mod authenticode;

#[derive(Debug, Clone)]
pub struct PEFile {
    pub pe_header: u32,
    pub coff_header: COFFHeader,
    pub optional_header: Option<OptionalHeader>,
    pub sections: Vec<SectionHeader>,
}
impl PEFile {
    pub fn write_to<T: MutBits>(&self, _out: &mut T) -> Result<(), Error> {
        todo!()
    }

    pub fn parse_from<T: Bits + Seek>(input: &mut T) -> Result<Self, Error> {
        input.advance(0x3C)?;
        let offset = input.read_le_u32()? as usize;
        input.advance(offset - 0x40)?;
        let pe_header = input.read_le_u32()?;
        let coff_header = COFFHeader::parse_from(input)?;
        let optional_header = if coff_header.size_of_optional_header > 0 {
            let orh = OptionalHeaderReader {
                size_of_optional_header: coff_header.size_of_optional_header,
            };
            let opt_header = orh.parse_from(input)?;

            Some(opt_header)
        } else {
            None
        };
        let mut sections = Vec::new();
        for _ in 0..coff_header.number_of_sections {
            sections.push(SectionHeader::parse_from(input)?);
        }
        Ok(PEFile {
            pe_header,
            coff_header,
            optional_header,
            sections,
        })
    }
}

pub fn load_authenticode_hash(f: &mut File, pe: &PEFile, hasher: &mut Hasher) -> Result<(), Error> {
    let Some(opt) = &pe.optional_header else {
        return Err(Error::new(
            BitsErrorKind::InvalidInput,
            "Missing optional header",
        ));
    };
    let (soh, csum_offset, ci_offset, cert_table) = match opt {
        OptionalHeader::OptionalPE32Header(_pe) => todo!(),
        OptionalHeader::OptionalPEPlusHeader(pe) => (
            pe.size_of_headers,
            pe.offsets.checksum,
            pe.data_directories.offsets.certificate_table,
            pe.data_directories.certificate_table,
        ),
    };
    // let end_of_headers = Seek::position(&mut f)?;
    // println!("{end_of_headers:08X} {end_of_headers}");

    f.seek(SeekFrom::Start(0x0))?;
    let mut sumofhashed: usize = 12;

    // read to checksum
    let next_len = csum_offset as usize;
    // println!("POS0: 0 // {next_len} //  {next_len:08X}");
    let v1 = f.read_exact_vec(next_len)?;
    hasher.write_all_bytes(&v1)?;
    sumofhashed += v1.len();

    f.advance(4)?;
    let cp = Seek::position(f)?;
    let next_len = ci_offset - cp;
    // println!("POS1: {cp} // {next_len} // {cp:08X} {next_len:08X}");
    let v1 = f.read_exact_vec(next_len as usize)?;
    hasher.write_all_bytes(&v1)?;
    sumofhashed += v1.len();

    f.advance(8)?;
    let cp = Seek::position(f)?;
    let next_len = soh as u64 - cp;
    // println!("POS2: {cp} // {next_len} // {cp:08X} {next_len:08X}");
    let v1 = f.read_exact_vec(next_len as usize)?;
    hasher.write_all_bytes(&v1)?;
    sumofhashed += v1.len();

    debug_assert_eq!(sumofhashed, soh as usize, "position: {cp}");

    for section in &pe.sections {
        let secoffset = section.pointer_to_raw_data;
        let seclen = section.size_of_raw_data;
        f.seek(SeekFrom::Start(secoffset as u64))?;
        let v = f.read_exact_vec(seclen as usize)?;
        hasher.write_all_bytes(&v)?;
        sumofhashed += v.len();
    }

    f.seek(SeekFrom::End(0))?;
    let flen = Seek::position(f)?;
    debug_assert_eq!(
        flen - cert_table.size as u64,
        sumofhashed as u64,
        "Missing extra length"
    );

    Ok(())
}
