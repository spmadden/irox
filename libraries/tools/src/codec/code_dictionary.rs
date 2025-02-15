// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
extern crate alloc;

use crate::codec::{DecodeGroupVarintFrom, EncodeGroupVarintTo};
use alloc::sync::Arc;
use core::hash::Hash;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicU64, Ordering};
use irox_bits::{
    Bits, BitsWrapper, Error, MutBits, ReadFromBEBits, SharedROCounter, WriteToBEBits,
};
use std::collections::HashMap;

///
/// Simple auto-incrementing dictionary indexed by hash value.  Creates new codes
/// for new values when first seen.  Increments in order of query.
#[derive(Debug)]
pub struct CodeDictionary<T: Eq + Hash> {
    dictionary: HashMap<T, u32>,
    inverse: HashMap<u32, T>,
    counter: Arc<AtomicU64>,
}
impl<T: Eq + Hash> Default for CodeDictionary<T> {
    fn default() -> CodeDictionary<T> {
        Self {
            dictionary: HashMap::new(),
            inverse: HashMap::new(),
            counter: Arc::new(AtomicU64::new(1)),
        }
    }
}
impl<T: Eq + Hash + Default> CodeDictionary<T> {
    pub fn new() -> CodeDictionary<T> {
        Default::default()
    }
}
impl<T: Eq + Hash + Clone> CodeDictionary<T> {
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
        let code = self.dictionary.entry(value.clone()).or_insert_with(|| {
            new_code = true;
            let ctr = self.counter.fetch_add(1, Ordering::SeqCst) as u32;
            self.inverse.insert(ctr, value.clone());
            ctr
        });
        (new_code, *code)
    }
    pub fn read_code<F: FnOnce() -> Result<T, E>, E>(
        &mut self,
        code: u32,
        value_producer: F,
    ) -> Result<T, E> {
        if let Some(val) = self.inverse.get(&code) {
            return Ok(val.clone());
        }
        let val = value_producer()?;
        self.inverse.insert(code, val.clone());
        self.dictionary.insert(val.clone(), code);
        Ok(val)
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
impl<T: Eq + Hash + Default + Clone + WriteToBEBits, B: MutBits> GroupVarintCodeEncoder<'_, T, B> {
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

    pub fn counter(&self) -> SharedROCounter {
        SharedROCounter::new(self.dict.counter.clone())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.inner.flush()
    }
}

///
/// Wraps [`CodeDictionary`] in an `Arc<RwLock>>` for shared access.
#[derive(Debug, Default, Clone)]
pub struct SharedCodeDictionary<T: Eq + Hash> {
    inner: Arc<std::sync::RwLock<CodeDictionary<T>>>,
}
impl<T: Eq + Hash + Default> SharedCodeDictionary<T> {
    pub fn new() -> SharedCodeDictionary<T> {
        Default::default()
    }
}
impl<T: Eq + Hash + Copy + Default> SharedCodeDictionary<T> {
    ///
    /// Looks up a code for a specific value
    pub fn lookup_value(&self, value: &T) -> Option<u32> {
        if let Ok(lock) = self.inner.read() {
            if let Some(code) = lock.lookup_value(value) {
                return Some(code);
            }
        }

        None
    }

    ///
    /// Returns the code for the specified value and if a new code was generated
    /// for the value (first time seeing the value).
    pub fn get_code(&mut self, value: &T) -> (bool, u32) {
        if let Ok(lock) = self.inner.read() {
            if let Some(code) = lock.lookup_value(value) {
                return (false, code);
            }
        }
        if let Ok(mut lock) = self.inner.write() {
            return lock.get_code(value);
        }
        (false, 0)
    }
    pub fn read_code<F: FnOnce() -> Result<T, E>, E>(
        &mut self,
        code: u32,
        value_producer: F,
    ) -> Result<T, E> {
        if let Ok(lock) = self.inner.read() {
            if let Some(val) = lock.inverse.get(&code) {
                return Ok(*val);
            }
        }
        if let Ok(mut lock) = self.inner.write() {
            let val = value_producer()?;
            lock.inverse.insert(code, val);
            lock.dictionary.insert(val, code);
            return Ok(val);
        }
        Ok(T::default())
    }
}

///
/// Converts values into codes using [`CodeDictionary`], then uses [`GroupVarintCodeEncoder`]
/// to encode a sequence of 4 codes to the stream.  If a code hasn't been written before,
/// we immediately follow the group varint block with the specific coded value(s) (up to 4).
///
/// Block format: `[control byte][4..=16 code bytes][0..=4 code-mapped-values]`
///
/// Must provide a shared dictionary to use this struct.  Decoding MUST be performed in the
/// exact same order as encoding or else the mapped values won't align correctly.
pub struct SharedGroupVarintCodeEncoder<'a, T: Eq + Hash, B: MutBits> {
    inner: BitsWrapper<'a, B>,
    dict: SharedCodeDictionary<T>,
}
impl<'a, T: Eq + Hash + Default, B: MutBits> SharedGroupVarintCodeEncoder<'a, T, B> {
    pub fn new(inner: BitsWrapper<'a, B>, dict: SharedCodeDictionary<T>) -> Self {
        Self { inner, dict }
    }
}
impl<T: Eq + Hash + Default + Copy + WriteToBEBits, B: MutBits>
    SharedGroupVarintCodeEncoder<'_, T, B>
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

pub struct GroupVarintCodeDecoder<'a, T: Hash + Eq, B: Bits> {
    inner: BitsWrapper<'a, B>,
    dict: CodeDictionary<T>,
}
impl<'a, T: Hash + Eq + Default, B: Bits> GroupVarintCodeDecoder<'a, T, B> {
    pub fn new(inner: BitsWrapper<'a, B>) -> Self {
        Self {
            inner,
            dict: CodeDictionary::new(),
        }
    }
}
impl<T: Hash + Eq + Default + ReadFromBEBits + Clone, B: Bits> GroupVarintCodeDecoder<'_, T, B> {
    fn decode_1(&mut self, code: u32) -> Result<T, Error> {
        self.dict
            .read_code(code, || T::read_from_be_bits(self.inner.deref_mut()))
    }

    pub fn decode_4(&mut self) -> Result<Option<[T; 4]>, Error> {
        let Some(val) = u32::decode_group_varint_from(self.inner.deref_mut())? else {
            return Ok(None);
        };
        let [a, b, c, d] = val;

        Ok(Some([
            self.decode_1(a)?,
            self.decode_1(b)?,
            self.decode_1(c)?,
            self.decode_1(d)?,
        ]))
    }
}

#[cfg(test)]
mod test {
    use crate::buf::{Buffer, FixedU8Buf, RoundU8Buffer};
    use crate::codec::{GroupVarintCodeDecoder, GroupVarintCodeEncoder};
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

    #[test]
    pub fn test_decoder() -> Result<(), Error> {
        let mut buf = RoundU8Buffer::from([
            0x00, // control char for first code block
            0x01, 0x02, 0x03, 0x04, // first 4 codes in code block
            0x00, 0x00, 0xAA, 0xAA, // first coded value,
            0x00, 0xBB, 0xBB, 0xBB, // second coded value,
            0x00, 0x00, 0x00, 0xCC, // third coded value,
            0xDD, 0xDD, 0xDD, 0xDD, // fourth coded value
            0x00, // control char for second code block
            0x01, 0x02, 0x03, 0x04, // second 4 code in code block
        ]);
        let mut dec = GroupVarintCodeDecoder::<u32, _>::new(BitsWrapper::Borrowed(&mut buf));
        let block1 = dec.decode_4()?;
        assert!(block1.is_some());
        if let Some(block1) = block1 {
            assert_eq_hex_slice!(&[0xAAAA, 0xBBBBBB, 0xCC, 0xDDDDDDDD], block1.as_ref())
        }
        let block2 = dec.decode_4()?;
        assert!(block2.is_some());
        if let Some(block2) = block2 {
            assert_eq_hex_slice!(&[0xAAAA, 0xBBBBBB, 0xCC, 0xDDDDDDDD], block2.as_ref())
        }
        let block3 = dec.decode_4()?;
        assert!(block3.is_none());
        assert_eq!(0, buf.len());
        Ok(())
    }
}
