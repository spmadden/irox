// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::codec::EncodeGroupVarintTo;
use core::hash::Hash;
use core::ops::DerefMut;
use irox_bits::{BitsWrapper, Error, MutBits, WriteToBEBits};
use std::collections::HashMap;

///
/// Simple auto-incrementing dictionary indexed by hash value.  Creates new codes
/// for new values when first seen.  Increments in order of query.
#[derive(Debug, Default)]
pub struct CodeDictionary<T: Eq + Hash> {
    dictionary: HashMap<T, u32>,
    counter: u32,
}
impl<T: Eq + Hash + Default> CodeDictionary<T> {
    pub fn new() -> CodeDictionary<T> {
        Default::default()
    }
}
impl<T: Eq + Hash + Copy> CodeDictionary<T> {
    ///
    /// Looks up a code for a specific value
    pub fn lookup_value(&self, value: &T) -> Option<u32> {
        self.dictionary.get(value).copied()
    }

    ///
    /// Returns the code for the specified value and if a new code was generated
    /// for the value (first time seeing the value).
    pub fn get_code(&mut self, value: &T) -> (bool, u32) {
        let mut new_code = false;
        let code = self.dictionary.entry(*value).or_insert_with(|| {
            new_code = true;
            self.counter += 1;
            self.counter
        });
        (new_code, *code)
    }
}

///
/// Converts values into codes using [`CodeDictionary`], then uses [`GroupVarintCodeEncoder`]
/// to encode a sequence of 4 codes to the stream.  If a code hasn't been written before,
/// we immediately follow the group varint block with the specific coded value(s) (up to 4).
///
/// Block format: `[control byte][4..=16 code bytes][0..=4 code-mapped-values]`
pub struct GroupVarintCodeEncoder<'a, T: Eq + Hash, B: MutBits> {
    inner: BitsWrapper<'a, B>,
    dict: CodeDictionary<T>,
}
impl<'a, T: Eq + Hash + Default, B: MutBits> GroupVarintCodeEncoder<'a, T, B> {
    pub fn new(inner: BitsWrapper<'a, B>) -> Self {
        Self {
            inner,
            dict: CodeDictionary::new(),
        }
    }
}
impl<'a, T: Eq + Hash + Default + Copy + WriteToBEBits, B: MutBits>
    GroupVarintCodeEncoder<'a, T, B>
{
    pub fn encode_4(&mut self, vals: &[T; 4]) -> Result<usize, Error> {
        let [a, b, c, d] = vals;
        let ea = self.dict.get_code(a);
        let eb = self.dict.get_code(b);
        let ec = self.dict.get_code(c);
        let ed = self.dict.get_code(d);

        let codes = [ea.1, eb.1, ec.1, ed.1];
        let mut used = codes.encode_group_varint_to(self.inner.deref_mut())?;
        if ea.0 {
            used += a.write_be_to(self.inner.deref_mut())?;
        }
        if eb.0 {
            used += b.write_be_to(self.inner.deref_mut())?;
        }
        if ec.0 {
            used += c.write_be_to(self.inner.deref_mut())?;
        }
        if ed.0 {
            used += d.write_be_to(self.inner.deref_mut())?;
        }

        Ok(used)
    }
}

#[cfg(test)]
mod test {
    use crate::buf::FixedU8Buf;
    use crate::codec::GroupVarintCodeEncoder;
    use crate::hex::HexDump;
    use irox_bits::{BitsWrapper, Error};

    #[test]
    pub fn test_encoder() -> Result<(), Error> {
        let mut buf = FixedU8Buf::<48>::new();
        {
            let mut codec = GroupVarintCodeEncoder::<u32, _>::new(BitsWrapper::Borrowed(&mut buf));
            let used = codec.encode_4(&[0xAAAA, 0xBBBBBB, 0xCC, 0xDDDDDDDD])?;
            assert_eq!(used, 5 + 16);
            let used = codec.encode_4(&[0xAAAA, 0xBBBBBB, 0xCC, 0xDDDDDDDD])?;
            assert_eq!(used, 5);
        }
        buf.hexdump();

        assert_eq!(5 + 16 + 5, buf.len());
        assert_eq_hex_slice!(
            &[
                0x00, // control char for first code block
                0x01, 0x02, 0x03, 0x04, // first 4 codes in code block
                0x00, 0x00, 0xAA, 0xAA, // first coded value,
                0x00, 0xBB, 0xBB, 0xBB, // second coded value,
                0x00, 0x00, 0x00, 0xCC, // third coded value,
                0xDD, 0xDD, 0xDD, 0xDD, // fourth coded value
                0x00, // control char for second code block
                0x01, 0x02, 0x03, 0x04, // second 4 code in code block
            ],
            buf.as_ref_used()
        );
        Ok(())
    }
}
