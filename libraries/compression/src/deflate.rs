// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use core::fmt::Debug;
use irox_bits::{BitStreamDecoder, Bits, BitsError, BitsErrorKind, BitsWrapper, Error, MutBits};
use irox_tools::buf::ZeroedBuffer;
use std::cmp::Ordering;
use std::collections::VecDeque;

static CODE_LENGTH_ORDER: &[usize] = &[
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];
static LENGTH_BASE: &[u16] = &[
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
static DISTANCE_BASE: &[u32] = &[
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeflateBlockType {
    Uncompressed,
    CompressedFixed,
    CompressedDynamic,
}
impl TryFrom<u8> for DeflateBlockType {
    type Error = BitsError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => DeflateBlockType::Uncompressed,
            1 => DeflateBlockType::CompressedFixed,
            2 => DeflateBlockType::CompressedDynamic,
            _ => return Err(BitsErrorKind::InvalidInput.into()),
        })
    }
}
pub enum DeflateBlockCommand {
    Literal(u8),
    Copy { length: usize, distance: usize },
}
enum DeflateBlockDecoder {
    // Uncompressed,
    CompressedHuffman {
        literals: HuffTable,
        distances: HuffTable,
    },
}
impl DeflateBlockDecoder {
    pub fn read_next<T: Bits>(
        &self,
        st: &mut BitStreamDecoder<'_, T>,
    ) -> Result<Option<DeflateBlockCommand>, BitsError> {
        match self {
            DeflateBlockDecoder::CompressedHuffman {
                literals,
                distances,
            } => {
                let r = literals.find_next_code(st)?;
                let len = match r.code {
                    0..=255 => {
                        return Ok(Some(DeflateBlockCommand::Literal(r.code as u8)));
                    }
                    256 => {
                        return Ok(None);
                    }
                    257..=285 => {
                        let Some(b) = LENGTH_BASE.get(r.code as usize - 257).copied() else {
                            return Err(BitsError::new(
                                BitsErrorKind::InvalidInput,
                                "code out of range",
                            ));
                        };
                        b + if (265..=284).contains(&r.code) {
                            let ext = ((r.code - 257) >> 2) - 1;
                            st.read_le_u32_bits(ext as u8)? as u16
                        } else {
                            0
                        }
                    }
                    _ => {
                        return Err(BitsError::new(
                            BitsErrorKind::InvalidInput,
                            "len exceeded 285",
                        ));
                    }
                };
                let r = distances.find_next_code(st)?;
                let dist = match r.code {
                    0..=29 => {
                        let Some(b) = DISTANCE_BASE.get(r.code as usize).copied() else {
                            return Err(BitsError::new(
                                BitsErrorKind::InvalidInput,
                                "code out of range",
                            ));
                        };
                        b + if (2..=29).contains(&r.code) {
                            let ext = (r.code >> 1) - 1;
                            st.read_le_u32_bits(ext as u8)?
                        } else {
                            0
                        }
                    }
                    _ => {
                        return Err(BitsError::new(
                            BitsErrorKind::InvalidInput,
                            "dist exceeded 30",
                        ));
                    }
                };
                Ok(Some(DeflateBlockCommand::Copy {
                    length: len as usize,
                    distance: dist as usize,
                }))
            }
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq)]
struct HuffLength {
    code: u16,
    bits: u16,
    symbol: u32,
    revsym: u32,
}
impl Debug for HuffLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HuffLength")
            .field("code", &self.code)
            .field("bits", &self.bits)
            .field(
                "sym",
                &format!("{:02} {:08b} {:02X}", self.symbol, self.symbol, self.symbol),
            )
            .field(
                "revsym",
                &format!("{:02} {:08b} {:02X}", self.revsym, self.revsym, self.revsym),
            )
            .finish()
    }
}

impl PartialOrd<Self> for HuffLength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffLength {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.bits == other.bits {
            self.code.cmp(&other.code)
        } else {
            self.bits.cmp(&other.bits)
        }
    }
}
#[derive(Debug, Clone)]
struct HuffTable {
    lengths: Vec<HuffLength>,
}
impl HuffTable {
    fn new_ordered(bootstrap: &[u16]) -> Result<Self, Error> {
        let iter = bootstrap
            .iter()
            .enumerate()
            .map(|(k, v)| (k, *v as i16))
            .collect::<Vec<_>>();
        Self::new(&iter)
    }
    fn new(bootstrap: &[(usize, i16)]) -> Result<Self, Error> {
        let mut lengths = Vec::new();
        let mut iter = bootstrap.iter();
        let Some((mut start, mut bits)) = iter.next().copied() else {
            return Err(Error::new(BitsErrorKind::InvalidInput, "empty bootstrap"));
        };
        for (finish, endbits) in iter {
            if bits > 0 {
                for code in start..*finish {
                    lengths.push(HuffLength {
                        code: code as u16,
                        bits: bits as u16,
                        symbol: u32::MAX,
                        revsym: u32::MAX,
                    })
                }
            }
            start = *finish;
            bits = *endbits;
            if *endbits == -1 {
                break;
            }
        }
        if start > 0 && bits > 0 {
            for code in start..bootstrap.len() {
                lengths.push(HuffLength {
                    code: code as u16,
                    bits: bits as u16,
                    symbol: u32::MAX,
                    revsym: u32::MAX,
                })
            }
        }
        lengths.sort();
        let mut bits = -1;
        let mut symbol = -1;
        for length in &mut lengths {
            symbol += 1;
            if length.bits as i32 != bits {
                symbol <<= length.bits as i32 - bits;
                bits = length.bits as i32;
            }
            length.symbol = symbol as u32;
            length.revsym = reverse_bits(symbol as u32, bits);
        }
        Ok(Self { lengths })
    }
    fn find_next_code<T: Bits>(
        &self,
        st: &mut BitStreamDecoder<'_, T>,
    ) -> Result<&HuffLength, BitsError> {
        for v in &self.lengths {
            if v.bits == 0 {
                continue;
            }
            if st.peek_le_u32_bits(v.bits as u8)? == v.revsym {
                st.read_le_u32_bits(v.bits as u8)?;
                return Ok(v);
            }
        }
        Err(BitsErrorKind::NotFound.into())
    }
    fn read_codelen_codes<T: Bits>(
        &self,
        cnt: usize,
        st: &mut BitStreamDecoder<'_, T>,
    ) -> Result<Vec<u16>, BitsError> {
        let mut real_code_lengths = Vec::new();
        let mut i = 0;
        while i < cnt {
            let l = self.find_next_code(st)?;
            let code = l.code;
            match code {
                0..=15 => {
                    real_code_lengths.push(code);
                    i += 1;
                }
                16 => {
                    let Some(last) = real_code_lengths.last().copied() else {
                        return Err(Error::new(
                            BitsErrorKind::InvalidInput,
                            "can't use 16 on empty set",
                        ));
                    };
                    let cnt = st.read_le_u32_bits(2)? as usize + 3;
                    for _ in 0..cnt {
                        real_code_lengths.push(last);
                    }
                    i += cnt;
                }
                17 => {
                    let cnt = st.read_le_u32_bits(3)? as usize + 3;
                    let end = real_code_lengths.len() + cnt;
                    real_code_lengths.resize(end, 0);
                    i += cnt;
                }
                18 => {
                    let cnt = st.read_le_u32_bits(7)? as usize + 11;
                    let end = real_code_lengths.len() + cnt;
                    real_code_lengths.resize(end, 0);
                    i += cnt;
                }
                _ => {
                    return Err(BitsErrorKind::InvalidInput.into());
                }
            }
        }
        Ok(real_code_lengths)
    }
}
pub struct Inflater<'a, T: Bits> {
    stream: BitStreamDecoder<'a, T>,
    bits_read: u8,
    block: VecDeque<u8>,
    block_offset: usize,
    complete: bool,
}

impl<'a, T: Bits> Inflater<'a, T> {
    pub fn new_zlib(stream: BitsWrapper<'a, T>) -> Self {
        let mut out = Self {
            stream: BitStreamDecoder::new(stream),
            bits_read: 0,
            block: <VecDeque<u8> as ZeroedBuffer>::new_zeroed(32768),
            block_offset: 0,
            complete: false,
        };
        let _ = out.read_zlib_header();
        out
    }
    fn read_zlib_header(&mut self) -> Result<(), BitsError> {
        let _cm = self.stream.read_le_u32_bits(4)?;
        let _cinfo = self.stream.read_le_u32_bits(4)?;
        let _fcheck = self.stream.read_le_u32_bits(5)?;
        let _fdict = self.stream.read_le_u32_bits(1)?;
        let _flevel = self.stream.read_le_u32_bits(2)?;
        // println!("cm: {cm}, cinfo: {cinfo}, fcheck: {fcheck}, fdict: {fdict}, flevel: {flevel}");
        Ok(())
    }
    pub fn read_deflate_block(&mut self) -> Result<Option<&[u8]>, BitsError> {
        if self.complete {
            return Ok(None);
        }
        let bfinal = self.stream.read_le_u32_bits(1)? == 1;
        let btype = self.stream.read_le_u32_bits(2)? as u8;
        self.complete = bfinal;
        let btype = DeflateBlockType::try_from(btype)?;
        if btype == DeflateBlockType::Uncompressed {
            let rem = 8 - (self.bits_read & 0x7);
            if rem != 0 && rem != 8 {
                let _ = self.stream.read_le_u32_bits(rem)?;
                self.bits_read += rem;
            }
            let del = self.stream.delegate();
            let len = del.read_le_u16()?;
            let _nlen = del.read_le_u16()?;
            self.block.clear();
            let mut bl = &mut self.block;
            del.read_exact_into(len as usize, &mut bl)?;
            return Ok(self.block.as_slices().0.get(0..len as usize));
        }
        let dec = if btype == DeflateBlockType::CompressedDynamic {
            let literals = self.stream.read_le_u32_bits(5)? as u16 + 257;
            let distances = self.stream.read_le_u32_bits(5)? as u16 + 1;
            let code_lengths = self.stream.read_le_u32_bits(4)? as usize + 4;

            let mut lengths = vec![0; 19];
            for idx in CODE_LENGTH_ORDER.iter().take(code_lengths) {
                let v = self.stream.read_le_u32_bits(3)? as u16;
                if let Some(l) = lengths.get_mut(*idx) {
                    *l = v;
                }
            }
            let table = HuffTable::new_ordered(&lengths)?;

            let literals = table.read_codelen_codes(literals as usize, &mut self.stream)?;
            let literals = HuffTable::new_ordered(&literals)?;
            let distances = table.read_codelen_codes(distances as usize, &mut self.stream)?;
            let distances = HuffTable::new_ordered(&distances)?;

            DeflateBlockDecoder::CompressedHuffman {
                literals,
                distances,
            }
        } else {
            let lit = vec![(0, 8), (144, 9), (256, 7), (280, 8), (288, -1)];
            let literals = HuffTable::new(&lit)?;
            let dist = vec![(0, 5), (32, -1)];
            let distances = HuffTable::new(&dist)?;
            DeflateBlockDecoder::CompressedHuffman {
                literals,
                distances,
            }
        };
        while self.block.len() > 32768 {
            self.block.pop_front();
        }
        self.block_offset = self.block.len();
        while let Some(cmd) = dec.read_next(&mut self.stream)? {
            match cmd {
                DeflateBlockCommand::Literal(v) => {
                    self.block.push_back(v);
                }
                DeflateBlockCommand::Copy {
                    mut length,
                    distance,
                } => {
                    while length > 0 {
                        let pos = self.block.len() - distance;
                        let len = length.min(self.block.len() - pos);
                        for i in 0..len {
                            if let Some(v) = self.block.get(pos + i).copied() {
                                self.block.push_back(v);
                            }
                        }
                        length -= len;
                    }
                }
            }
        }
        let v = self.block.make_contiguous();
        Ok(v.get(self.block_offset..))
    }
}

pub(crate) fn reverse_bits(v: u32, n: i32) -> u32 {
    let mut a = 1;
    let mut b = 1 << (n - 1);
    let mut z = 0;
    for i in 0..((n + 1) / 2) {
        let i = (n - 1) + -2 * i;
        z |= (v >> i) & a;
        z |= (v << i) & b;
        a <<= 1;
        b >>= 1;
    }
    z
}

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum CompressLevel {
    None = 0,
    BestSpeed = 1,
    #[default]
    Default = 6,
    BestCompression = 9,
    UberCompression = 10,
}
#[derive(Default)]
pub struct DeflaterBuilder {
    deflater: Deflater,
}
impl DeflaterBuilder {
    pub fn set_compression_level(&mut self, level: CompressLevel) -> &mut Self {
        self.deflater.compression_level = level;
        self
    }

    pub fn set_write_zlib_header(&mut self, write_zlib_header: bool) -> &mut Self {
        self.deflater.write_zlib_header = Some(write_zlib_header);
        self
    }
}

#[derive(Default)]
pub struct Deflater {
    writebuf: VecDeque<u8>,
    readbuf: VecDeque<u8>,
    compression_level: CompressLevel,
    write_zlib_header: Option<bool>,
}
impl Drop for Deflater {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}
impl Deflater {
    pub fn flush(&mut self) -> Result<(), BitsError> {
        Ok(())
    }
    pub fn finish(mut self) -> Result<(), BitsError> {
        self.flush()?;

        Ok(())
    }
}
impl MutBits for Deflater {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.writebuf.push_back(val);
        todo!()
    }
}
impl Bits for Deflater {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        Ok(self.readbuf.pop_front())
    }
}
#[cfg(test)]
mod tests {
    use crate::deflate::Inflater;
    use irox_bits::{BitsError, BitsWrapper, SharedCountingBits};
    use irox_tools::hash::SHA1;
    use irox_tools::{assert_eq_hex_slice, hex};
    // 6f3edd9512fe21e1aaa4e5691f00894a364612e4
    #[test]
    pub fn test_inflate1() -> Result<(), BitsError> {
        let file = std::fs::File::open("doc/td2.zlib")?;
        let file = std::io::BufReader::new(file);
        let file = SharedCountingBits::new(BitsWrapper::Owned(file));
        let mut inf = Inflater::new_zlib(BitsWrapper::Owned(file));
        let mut hash = SHA1::default();
        while let Ok(Some(h)) = inf.read_deflate_block() {
            hash.write(h);
        }
        let hash = hash.finish();
        assert_eq_hex_slice!(hash, hex!("f540774a4dc9e45e5221a80a3bfa621ef4ffb9b6"));
        Ok(())
    }

    #[test]
    pub fn test_inflate2() -> Result<(), BitsError> {
        let file = std::fs::File::open("doc/big.zlib")?;
        let file = std::io::BufReader::new(file);
        let file = SharedCountingBits::new(BitsWrapper::Owned(file));
        let mut inf = Inflater::new_zlib(BitsWrapper::Owned(file));
        let mut hash = SHA1::default();
        while let Ok(Some(h)) = inf.read_deflate_block() {
            hash.write(h);
        }
        let hash = hash.finish();
        assert_eq_hex_slice!(hash, hex!("6f3edd9512fe21e1aaa4e5691f00894a364612e4"));
        Ok(())
    }
}
