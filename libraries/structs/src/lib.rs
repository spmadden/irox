// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! A very simple 'Struct' library that provides a single trait.  This is intended to be most
//! generally used in concert with [`irox_structs_derive`] to generate the impls.
//!
//! A Struct is a linear sequence in memory of serialized bytes - serialized in the order the
//! fields are present in the struct with no adjacency or packing.
//!
//! ## Strict Sizing
//! By default, [`irox_structs_derive`] will allow variably sized types like [`String`] and [`Vec<u8>`].
//! If you know you don't need/use these, apply a `#[strict_sizing]` attribute, and the generator
//! will add a `pub const STRUCT_SIZE: usize` to the struct
//!
//! ## Choosing Endianness:
//! Apply either the `#[big_endian]` or `#[little_endian]` attributes when deriving [`Struct`] and
//! it will use the appropriate serializers.  If not specified, it defaults to big endian.
//!
//! ### Example Big Endian:
//! ```
//! use irox_structs::Struct;
//! use irox_bits::Error;
//!
//! #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Struct)]
//! #[strict_sizing]
//! pub struct UdpHeader {
//!     source_port: u16,
//!     dest_port: u16,
//!     length: u16,
//!     checksum: u16,
//! }
//!
//! pub fn main() -> Result<(), Error>{
//!     assert_eq!(8, UdpHeader::STRUCT_SIZE);
//!
//!    let header = UdpHeader {
//!         source_port: 0x0064,
//!         dest_port: 0x0400,
//!         length: 0x1388,
//!         checksum: 0x01C2,
//!     };
//!    let mut output_buf: Vec<u8> = Vec::new();
//!
//!    header.write_to(&mut output_buf)?;
//!    assert_eq!(output_buf.len(), 8);
//!    assert_eq!(&[0x00u8, 0x64, 0x04, 0x00, 0x13, 0x88, 0x01, 0xC2],
//!                 output_buf.as_slice());
//!
//!    let parsed = UdpHeader::parse_from(&mut output_buf.as_slice())?;
//!     assert_eq!(header, parsed);
//!  Ok(())
//! }
//! ```
//! ### Example Little Endian:
//! ```
//! use irox_structs::Struct;
//! use irox_bits::Error;
//!
//! #[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Struct)]
//! #[little_endian]
//! #[strict_sizing]
//! pub struct UdpHeader {
//!     source_port: u16,
//!     dest_port: u16,
//!     length: u16,
//!     checksum: u16,
//! }
//!
//! pub fn main() -> Result<(), Error>{
//!     assert_eq!(8, UdpHeader::STRUCT_SIZE);
//!    let header = UdpHeader {
//!         source_port: 0x0064,
//!         dest_port: 0x0400,
//!         length: 0x1388,
//!         checksum: 0x01C2,
//!     };
//!    let mut output_buf: Vec<u8> = Vec::new();
//!
//!    header.write_to(&mut output_buf)?;
//!    assert_eq!(output_buf.len(), 8);
//!    assert_eq!(&[0x64u8, 0x00, 0x00, 0x04, 0x88, 0x13, 0xC2, 0x01],
//!                 output_buf.as_slice());
//!
//!    let parsed = UdpHeader::parse_from(&mut output_buf.as_slice())?;
//!     assert_eq!(header, parsed);
//!  Ok(())
//! }
//! ```

#[cfg(feature = "alloc")]
extern crate alloc;
pub use irox_bits::{Bits, Error, MutBits};
pub use irox_structs_derive::*;

///
/// A struct is a series of bytes in memory, serialized in the order that the
/// fields are present in the struct.
///
/// Generally speaking, you shouldn't need to implement this, unless you need
/// some custom encoding.  It's intended to be used with [`irox_structs_derive`] to automatically
/// generate the impl.
pub trait Struct {
    type ImplType;

    ///
    /// Write the encoding of the type to the specified output buffer
    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), Error>;

    ///
    /// Returns the encoded bytes as a vector
    #[cfg(feature = "alloc")]
    fn as_bytes(&self) -> Result<alloc::vec::Vec<u8>, Error> {
        let mut buf: Vec<u8> = Vec::new();
        self.write_to(&mut buf)?;
        Ok(buf)
    }

    ///
    /// Parses and creates the impl type from the input stream, consuming bytes along
    /// the way.
    fn parse_from<T: Bits>(input: &mut T) -> Result<Self::ImplType, Error>;
}
