// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{Bits, BitsError};
use crate::{read_length, DecodeDER, Tag, TagValue};

#[derive(Debug, Clone)]
pub enum Asn1Object {
    Sequence(Vec<Asn1Object>),

}

impl DecodeDER for Asn1Object {
    type Output = Self;

    fn decode_der<T: Bits>(input: &mut T) -> Result<Self::Output, BitsError> {
        let tag = Tag::read_from(input)?;
        let length = read_length(input)?;
        println!("Tag: {tag:#?}, length: {length:#?}");
        if tag.value == 0x10 {
            // sequence.
            let mut out = Vec::<Asn1Object>::new();
            out.push(Asn1Object::decode_der(input)?);
            return Ok(Asn1Object::Sequence(out));
        }
        todo!()
    }
}