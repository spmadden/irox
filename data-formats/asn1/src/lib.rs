// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! ITU-T X.690 ASN.1 DER/BER/CER Encoding and Decoding
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod primitives;
mod tags;
mod objects;

pub use tags::*;
pub use objects::*;
pub use primitives::*;

use irox_bits::{Bits, BitsError, MutBits};

pub trait EncodeDER {
    fn write_der<T: MutBits>(&self, tag_id: u32, out: &mut T) -> Result<usize, BitsError>;
}

pub trait DecodeDER {
    type Output;
    fn decode_der<T: Bits>(input: &mut T) -> Result<Self::Output, BitsError>;
}

pub struct TagValue<T> {
    tag: u32,
    value: T,
}
