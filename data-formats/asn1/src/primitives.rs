// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::EncodeDER;
use irox_bits::{BitsError, MutBits};
use irox_types::PrimitiveValue;

impl EncodeDER for PrimitiveValue {
    fn write_der<T: MutBits>(&self, tag_id: u32, out: &mut T) -> Result<usize, BitsError> {
        match self {
            PrimitiveValue::u8(_) => {}
            PrimitiveValue::i8(_) => {}
            PrimitiveValue::u16(_) => {}
            PrimitiveValue::i16(_) => {}
            PrimitiveValue::u32(_) => {}
            PrimitiveValue::i32(_) => {}
            PrimitiveValue::f32(_) => {}
            PrimitiveValue::u64(_) => {}
            PrimitiveValue::i64(_) => {}
            PrimitiveValue::f64(_) => {}
            PrimitiveValue::u128(_) => {}
            PrimitiveValue::i128(_) => {}
            PrimitiveValue::bool(_) => {}
            PrimitiveValue::char(_) => {}
            PrimitiveValue::null => {}
            _ => {}
        }
        todo!()
    }
}

impl EncodeDER for bool {
    fn write_der<T: MutBits>(&self, tag_id: u32, out: &mut T) -> Result<usize, BitsError> {
        todo!()
    }
}
