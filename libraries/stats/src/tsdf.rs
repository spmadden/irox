// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::sampling::{IntSample64, Sample64, StrSample64};
use crate::streams::{
    AddingDecoder, CompressStream, Decoder, DeltaStream, F64ToU64Stream, I64ToU64Stream,
    InflateStream, Stream, StreamStageStats, U64ToF64Decoder, U64ToI64Decoder, ZigZagDecoder,
    ZigZagStream,
};
use alloc::sync::Arc;
use core::hash::Hash;
use irox_bits::{
    Bits, BitsError, BitsErrorKind, BitsWrapper, Error, MutBits, ReadFromBEBits,
    SharedCountingBits, SharedROCounter, WriteToBEBits,
};
use irox_time::Time64;
use irox_tools::buf::{Buffer, RoundBuffer};
use irox_tools::codec::{GroupVarintCodeDecoder, GroupVarintCodeEncoder};
use irox_tools::map::OrderedHashMap;
use irox_tools::read::{MultiStreamReader, MultiStreamWriter, StreamWriter};
use irox_tools::StrWrapper;
use std::path::Path;

macro_rules! new_bdc {
    ($writer:ident) => {
        Box::new(CompressStream::new(BitsWrapper::Owned(
            $writer.new_stream(),
        )))
    };
}

///
/// Breaks a [`u64`] into the 8 component bytes, and then compresses them individually.
pub struct EightByteStream<'a> {
    pub(crate) fb1: Box<CompressStream<'a, StreamWriter>>,
    pub(crate) fb2: Box<CompressStream<'a, StreamWriter>>,
    pub(crate) fb3: Box<CompressStream<'a, StreamWriter>>,
    pub(crate) fb4: Box<CompressStream<'a, StreamWriter>>,
    pub(crate) fb5: Box<CompressStream<'a, StreamWriter>>,
    pub(crate) fb6: Box<CompressStream<'a, StreamWriter>>,
    pub(crate) fb7: Box<CompressStream<'a, StreamWriter>>,
    pub(crate) fb8: Box<CompressStream<'a, StreamWriter>>,
}
impl<'a> EightByteStream<'a> {
    pub fn new(writer: &Arc<MultiStreamWriter>) -> Self {
        Self {
            fb1: new_bdc!(writer),
            fb2: new_bdc!(writer),
            fb3: new_bdc!(writer),
            fb4: new_bdc!(writer),
            fb5: new_bdc!(writer),
            fb6: new_bdc!(writer),
            fb7: new_bdc!(writer),
            fb8: new_bdc!(writer),
        }
    }

    pub fn written(&self) -> u64 {
        let mut out = 0u64;
        out = out.wrapping_add(self.fb1.written());
        out = out.wrapping_add(self.fb2.written());
        out = out.wrapping_add(self.fb3.written());
        out = out.wrapping_add(self.fb4.written());
        out = out.wrapping_add(self.fb5.written());
        out = out.wrapping_add(self.fb6.written());
        out = out.wrapping_add(self.fb7.written());
        out = out.wrapping_add(self.fb8.written());
        out
    }

    pub fn written_stats(&self) -> [u64; 8] {
        [
            self.fb1.written(),
            self.fb2.written(),
            self.fb3.written(),
            self.fb4.written(),
            self.fb5.written(),
            self.fb6.written(),
            self.fb7.written(),
            self.fb8.written(),
        ]
    }
}
impl<'a> Stream<u64> for EightByteStream<'a> {
    fn write_value(&mut self, v: u64) -> Result<usize, Error> {
        let [a, b, c, d, e, f, g, h] = v.to_be_bytes();
        self.fb1.write_value(a as i8)?;
        self.fb2.write_value(b as i8)?;
        self.fb3.write_value(c as i8)?;
        self.fb4.write_value(d as i8)?;
        self.fb5.write_value(e as i8)?;
        self.fb6.write_value(f as i8)?;
        self.fb7.write_value(g as i8)?;
        self.fb8.write_value(h as i8)?;
        Ok(8)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.fb1.flush()?;
        self.fb2.flush()?;
        self.fb3.flush()?;
        self.fb4.flush()?;
        self.fb5.flush()?;
        self.fb6.flush()?;
        self.fb7.flush()?;
        self.fb8.flush()?;
        Ok(())
    }

    fn written_stats(&self) -> String {
        format!("{:?} = {}", self.written_stats(), self.written())
    }
}
///
/// Time Series Data File using the SPDP encoding scheme
pub struct SPDPWriter<'a> {
    writer: Arc<MultiStreamWriter>,
    time_stream: EightByteStream<'a>,
    float_stream: EightByteStream<'a>,
    semi_last_value: f64,
    last_value: f64,
}

///
/// The rot54 operation is intended to exploit the ordered entropy of a float mantissa.  Most of the
/// zero values from the mantissa will be in the LSBs.  Rot54 is therefor:
///
/// Float: `0xEEE_MMMMMMMMMMMMMu64 = 0x00_0ABBCCDDEEFFGG`
/// Rot54: `0x00GGFFEEDDCCBB0A`
///
/// A value like `0.25f64` is encoded as `0x3FD0000000000000` if you call [`f64::to_bits`].  The rot54
/// operation rotates it to `0xD03F` - which usually encodes much nicer.
pub fn rot54(value: u64) -> u64 {
    let [_, b, c, d, e, f, g, h] = value.to_be_bytes();
    irox_bits::FromBEBytes::from_be_bytes([0, h, g, f, e, d, c, b])
}

impl<'a> SPDPWriter<'a> {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let writer = MultiStreamWriter::new(path)?;
        let time_stream = EightByteStream::new(&writer);
        let float_stream = EightByteStream::new(&writer);

        Ok(SPDPWriter {
            writer,
            time_stream,
            float_stream,
            last_value: f64::default(),
            semi_last_value: f64::default(),
        })
    }

    pub fn write_sample(&mut self, sample: &Sample64) -> Result<(), Error> {
        let Sample64 { time, value } = sample;
        self.time_stream.write_value(time.as_u64())?;

        let delta = value; // - self.semi_last_value;
        self.semi_last_value = self.last_value;
        self.last_value = *value;
        self.float_stream.write_value(delta.to_bits())?;
        Ok(())
    }
    pub fn flush(&mut self) -> Result<(), Error> {
        self.time_stream.flush()?;
        self.float_stream.flush()?;
        Ok(())
    }

    pub fn len(&self) -> Result<u64, Error> {
        self.writer.len()
    }
    pub fn is_empty(&self) -> Result<bool, Error> {
        Ok(self.len()? == 0)
    }
}
///
/// Basic stream to [`rot54`] the input sample.
pub struct Rot54Stream {
    writer: Box<dyn Stream<u64>>,
}
impl Rot54Stream {
    pub fn new(writer: Box<dyn Stream<u64>>) -> Self {
        Self { writer }
    }
}
impl Stream<u64> for Rot54Stream {
    fn write_value(&mut self, value: u64) -> Result<usize, Error> {
        let v = rot54(value);
        self.writer.write_value(v)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.writer.flush()
    }
}

///
/// Basic stream to split a [`f64`] into it's exponent and mantissa
pub struct FloatSplitter {
    mantissa_writer: Box<dyn Stream<u64>>,
    exponent_writer: Box<dyn Stream<u64>>,
}
impl FloatSplitter {
    pub fn new(
        mantissa_writer: Box<dyn Stream<u64>>,
        exponent_writer: Box<dyn Stream<u64>>,
    ) -> Self {
        Self {
            mantissa_writer,
            exponent_writer,
        }
    }
}
impl Stream<f64> for FloatSplitter {
    fn write_value(&mut self, value: f64) -> Result<usize, Error> {
        let bits = value.to_bits();
        let exponent = bits >> 52;
        let mantissa = bits & 0xFFFFFFFFFFFFF;
        self.mantissa_writer.write_value(mantissa)?;
        self.exponent_writer.write_value(exponent)?;
        Ok(8)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.mantissa_writer.flush()?;
        self.exponent_writer.flush()?;
        Ok(())
    }
}

///
/// Stream to collect values into groups of 4 and then run them through a [`GroupVarintCodeEncoder`]
pub struct GroupCodingStream<'a, T: Hash + Eq + Sized + Default + Clone + WriteToBEBits, B: MutBits>
{
    buf: RoundBuffer<4, T>,
    inner: GroupVarintCodeEncoder<'a, T, B>,
}
impl<'a, T: Hash + Eq + Default + Sized + Default + Clone + WriteToBEBits, B: MutBits>
    GroupCodingStream<'a, T, B>
{
    pub fn new(inner: BitsWrapper<'a, B>) -> Self {
        Self {
            buf: RoundBuffer::new(),
            inner: GroupVarintCodeEncoder::new(inner),
        }
    }
    pub fn counter(&self) -> SharedROCounter {
        self.inner.counter()
    }
}
impl<'a, T: Hash + Eq + Sized + Default + Clone + WriteToBEBits, B: MutBits> Stream<T>
    for GroupCodingStream<'a, T, B>
{
    fn write_value(&mut self, value: T) -> Result<usize, Error> {
        let _ = self.buf.push_back(value);
        if self.buf.is_full() {
            let a = self.buf.pop_front().unwrap_or_default();
            let b = self.buf.pop_front().unwrap_or_default();
            let c = self.buf.pop_front().unwrap_or_default();
            let d = self.buf.pop_front().unwrap_or_default();
            self.inner.encode_4(&[a, b, c, d])
        } else {
            Ok(0)
        }
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.inner.flush()
    }
}
impl<'a, T: Hash + Eq + Sized + Default + Clone + WriteToBEBits, B: MutBits> Drop
    for GroupCodingStream<'a, T, B>
{
    fn drop(&mut self) {
        let len = self.buf.len();
        if len > 0 {
            let needed = 4 - len;
            for _ in 0..needed {
                let _ = self.write_value(T::default());
            }
        }
        let _ = self.inner.flush();
    }
}

pub struct GroupDecodingStream<'a, T: Hash + Eq + Default, B: Bits> {
    inner: GroupVarintCodeDecoder<'a, T, B>,
    buf: RoundBuffer<4, T>,
}
impl<'a, T: Hash + Eq + Default + ReadFromBEBits + Clone, B: Bits> GroupDecodingStream<'a, T, B> {
    pub fn new(inner: BitsWrapper<'a, B>) -> Self {
        Self {
            inner: GroupVarintCodeDecoder::new(inner),
            buf: RoundBuffer::new(),
        }
    }
}
impl<'a, T: Hash + Eq + Default + ReadFromBEBits + Clone, B: Bits> Decoder<T>
    for GroupDecodingStream<'a, T, B>
{
    fn next(&mut self) -> Result<Option<T>, Error> {
        if self.buf.is_empty() {
            let Some(val) = self.inner.decode_4()? else {
                return Ok(None);
            };
            for v in val {
                let _ = self.buf.push_back(v.clone());
            }
        }
        Ok(self.buf.pop_front())
    }
}

///
/// Coded Time Series Sample File consists of 2 streams: a data stream and a time stream backed by a [`MultiStreamWriter`]
///
/// Data stream:
/// 1. Convert [`f64`] to [`u64`] bit-for-bit
/// 2. Run it through a [`irox_tools::codec::CodeDictionary`] to map the observed values into unique [`u32`] codes
/// 3. Group those codes into blocks of 4 using [`GroupCodingStream`] and then encode into Varint-GB using [`GroupVarintCodeEncoder`]
/// 4. Deflate/GZ the resultant byte stream.
///
/// It is assumed that the data stream samples come from something approximating a A2D sensor with a fixed number of detection bits
/// and as such, most of the data will be fairly similar, even if very noisy when it jumps around.
///
/// Time stream:
/// 0. Convert the time value into a [`u64`] (external)
/// 1. Run it through a [`DeltaStream`] to encode the first value, and then output the `N-1` difference
/// 2. Run it through the same 2, 3, 4 processing as the data stream.
///
/// It is assumed that the time series will be periodically sampled and atomically increasing
pub struct CodedTimeSeriesWriter<'a> {
    float_stream: Box<dyn Stream<f64>>,
    time_stream: Box<dyn Stream<u64>>,
    int_stream: Box<dyn Stream<u64>>,
    str_stream: Box<dyn Stream<StrWrapper<'a>> + 'a>,
    meta_stream: Box<dyn Stream<&'a str> + 'a>,
    stats: StreamStageStats,
}

impl<'a> CodedTimeSeriesWriter<'a> {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let mut stats = StreamStageStats::default();

        let writer = MultiStreamWriter::new(path)?;

        let meta_stream = writer.new_stream();
        let meta_stream = CompressStream::new(BitsWrapper::Owned(meta_stream));

        let time_stream = writer.new_stream();
        let time_stream = SharedCountingBits::new(BitsWrapper::Owned(time_stream));
        stats.stage_counting("1.1.time_gz", time_stream.get_count());
        let time_stream = CompressStream::new(BitsWrapper::Owned(time_stream));
        let time_stream = SharedCountingBits::new(BitsWrapper::Owned(time_stream));
        stats.stage_counting("1.2.time_vgb", time_stream.get_count());
        let time_stream = GroupCodingStream::<u64, _>::new(BitsWrapper::Owned(time_stream));
        stats.stage_counting("1.3.time_codes", time_stream.counter());
        let time_stream = ZigZagStream::new(Box::new(time_stream));
        let time_stream = DeltaStream::<i64>::new(Box::new(time_stream));

        let float_stream = writer.new_stream();
        let float_stream = SharedCountingBits::new(BitsWrapper::Owned(float_stream));
        stats.stage_counting("2.1.float_gz", float_stream.get_count());
        let float_stream = CompressStream::new(BitsWrapper::Owned(float_stream));
        let float_stream = SharedCountingBits::new(BitsWrapper::Owned(float_stream));
        stats.stage_counting("2.2.float_vgb", float_stream.get_count());
        let float_stream = GroupCodingStream::<u64, _>::new(BitsWrapper::Owned(float_stream));
        stats.stage_counting("2.3.float_codes", float_stream.counter());
        let float_stream = F64ToU64Stream::new(Box::new(float_stream));

        let int_stream = writer.new_stream();
        let int_stream = SharedCountingBits::new(BitsWrapper::Owned(int_stream));
        stats.stage_counting("3.1.int_gz", int_stream.get_count());
        let int_stream = CompressStream::new(BitsWrapper::Owned(int_stream));
        let int_stream = SharedCountingBits::new(BitsWrapper::Owned(int_stream));
        stats.stage_counting("3.2.int_vgb", int_stream.get_count());
        let int_stream = GroupCodingStream::<u64, _>::new(BitsWrapper::Owned(int_stream));
        let int_stream = I64ToU64Stream::new(Box::new(int_stream));
        let int_stream = DeltaStream::<i64>::new(Box::new(int_stream));

        let str_stream = writer.new_stream();
        let str_stream = SharedCountingBits::new(BitsWrapper::Owned(str_stream));
        stats.stage_counting("4.1.str_gz", str_stream.get_count());
        let str_stream = CompressStream::new(BitsWrapper::Owned(str_stream));
        let str_stream = GroupCodingStream::new(BitsWrapper::Owned(str_stream));

        Ok(Self {
            float_stream: Box::new(float_stream),
            time_stream: Box::new(time_stream),
            meta_stream: Box::new(meta_stream),
            int_stream: Box::new(int_stream),
            str_stream: Box::new(str_stream),
            stats,
        })
    }

    #[must_use]
    pub fn float_stream(self) -> CodedTimeSeriesFloatWriter<'a> {
        CodedTimeSeriesFloatWriter { writer: self }
    }
    #[must_use]
    pub fn int_stream(self) -> CodedTimeSeriesIntWriter<'a> {
        CodedTimeSeriesIntWriter { writer: self }
    }

    pub fn write_str(&mut self, time: Time64, value: StrWrapper<'a>) -> Result<(), Error> {
        self.time_stream.write_value(time.as_u64())?;
        self.str_stream.write_value(value)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.time_stream.flush()?;
        self.float_stream.flush()?;
        self.int_stream.flush()?;
        self.str_stream.flush()?;
        self.meta_stream.flush()?;
        Ok(())
    }
    pub fn written_stats(&self) -> Vec<String> {
        let mut out = self.stats.stats();
        out.push(self.meta_stream.written_stats());
        out.push(self.time_stream.written_stats());
        out.push(self.float_stream.written_stats());
        out.push(self.int_stream.written_stats());
        out.push(self.str_stream.written_stats());
        out
    }

    pub fn metadata<K: AsRef<str> + 'a, V: AsRef<str> + 'a>(
        &'a mut self,
        key: &'a K,
        value: &'a V,
    ) -> Result<(), Error> {
        let key = key.as_ref();
        let value = value.as_ref();
        self.meta_stream.write_value(key)?;
        self.meta_stream.write_value(value)?;
        Ok(())
    }
}
pub struct CodedTimeSeriesFloatWriter<'a> {
    writer: CodedTimeSeriesWriter<'a>,
}
impl<'a> CodedTimeSeriesFloatWriter<'a> {
    pub fn write_sample(&mut self, sample: &Sample64) -> Result<(), Error> {
        let Sample64 { time, value } = sample;
        self.writer.time_stream.write_value(time.as_u64())?;
        self.writer.float_stream.write_value(*value)?;
        Ok(())
    }
    pub fn metadata<K: AsRef<str> + 'a, V: AsRef<str> + 'a>(
        &'a mut self,
        key: &'a K,
        value: &'a V,
    ) -> Result<(), Error> {
        self.writer.metadata(key, value)
    }
    pub fn flush(&mut self) -> Result<(), Error> {
        self.writer.flush()
    }
    pub fn written_stats(&self) -> Vec<String> {
        self.writer.written_stats()
    }
}
pub struct CodedTimeSeriesIntWriter<'a> {
    writer: CodedTimeSeriesWriter<'a>,
}
impl<'a> CodedTimeSeriesIntWriter<'a> {
    pub fn write_sample(&mut self, time: Time64, value: u64) -> Result<(), Error> {
        self.writer.time_stream.write_value(time.as_u64())?;
        self.writer.int_stream.write_value(value)?;
        Ok(())
    }
    pub fn metadata<K: AsRef<str> + 'a, V: AsRef<str> + 'a>(
        &'a mut self,
        key: &'a K,
        value: &'a V,
    ) -> Result<(), Error> {
        self.writer.metadata(key, value)
    }
    pub fn flush(&mut self) -> Result<(), Error> {
        self.writer.flush()
    }
    pub fn written_stats(&self) -> Vec<String> {
        self.writer.written_stats()
    }
}
pub struct CodedTimeSeriesStrWriter<'a> {
    writer: CodedTimeSeriesWriter<'a>,
}
impl<'a> CodedTimeSeriesStrWriter<'a> {
    pub fn write_sample(&mut self, time: Time64, value: StrWrapper<'a>) -> Result<(), Error> {
        self.writer.time_stream.write_value(time.as_u64())?;
        self.writer.str_stream.write_value(value)?;
        Ok(())
    }
    pub fn metadata<K: AsRef<str> + 'a, V: AsRef<str> + 'a>(
        &'a mut self,
        key: &'a K,
        value: &'a V,
    ) -> Result<(), Error> {
        self.writer.metadata(key, value)
    }
    pub fn flush(&mut self) -> Result<(), Error> {
        self.writer.flush()
    }
    pub fn written_stats(&self) -> Vec<String> {
        self.writer.written_stats()
    }
}

pub enum TimeSeriesError {
    BitsError(BitsError),
    MissingMetadataStream,
    MissingFloatStream,
    MissingIntStream,
    MissingStrStream,
    MissingTimeStream,
}
impl TimeSeriesError {
    pub fn name(&self) -> &'static str {
        match self {
            TimeSeriesError::BitsError(..) => "BitsError",
            TimeSeriesError::MissingMetadataStream => "MissingMetadataStream",
            TimeSeriesError::MissingFloatStream => "MissingFloatStream",
            TimeSeriesError::MissingTimeStream => "MissingTimeStream",
            TimeSeriesError::MissingIntStream => "MissingIntStream",
            TimeSeriesError::MissingStrStream => "MissingStrStream",
        }
    }
}
impl From<BitsError> for TimeSeriesError {
    fn from(e: BitsError) -> Self {
        TimeSeriesError::BitsError(e)
    }
}
impl From<TimeSeriesError> for BitsError {
    fn from(e: TimeSeriesError) -> Self {
        match e {
            TimeSeriesError::BitsError(e) => e,
            _ => BitsError::new(BitsErrorKind::InvalidData, e.name()),
        }
    }
}
pub struct CodedTimeSeriesReader<'a> {
    metadata: OrderedHashMap<String, String>,
    float_decoder: Box<dyn Decoder<f64>>,
    time_decoder: Box<dyn Decoder<u64>>,
    int_decoder: Box<dyn Decoder<u64>>,
    str_decoder: Box<dyn Decoder<StrWrapper<'a>> + 'a>,
}
impl<'a> CodedTimeSeriesReader<'a> {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self, TimeSeriesError> {
        let mut reader = MultiStreamReader::open(path)?;
        let mut streams = reader.drain(..);
        let Some(meta_stream) = streams.next() else {
            return Err(TimeSeriesError::MissingMetadataStream);
        };
        let mut meta_stream = InflateStream::new(BitsWrapper::Owned(meta_stream));
        let mut metadata = OrderedHashMap::<String, String>::new();
        while meta_stream.has_more()? {
            let key = String::read_from_be_bits(&mut meta_stream)?;
            let value = String::read_from_be_bits(&mut meta_stream)?;
            metadata.insert(key, value);
        }

        let Some(time_stream) = streams.next() else {
            return Err(TimeSeriesError::MissingTimeStream);
        };
        let time_stream = InflateStream::new(BitsWrapper::Owned(time_stream));
        let time_stream = GroupDecodingStream::<u64, _>::new(BitsWrapper::Owned(time_stream));
        let time_stream = ZigZagDecoder::new(Box::new(time_stream));
        let time_stream = AddingDecoder::new(Box::new(time_stream));

        let Some(float_stream) = streams.next() else {
            return Err(TimeSeriesError::MissingFloatStream);
        };
        let float_stream = InflateStream::new(BitsWrapper::Owned(float_stream));
        let float_stream = GroupDecodingStream::<u64, _>::new(BitsWrapper::Owned(float_stream));
        let float_stream = U64ToF64Decoder::new(Box::new(float_stream));

        let Some(int_stream) = streams.next() else {
            return Err(TimeSeriesError::MissingIntStream);
        };
        let int_stream = InflateStream::new(BitsWrapper::Owned(int_stream));
        let int_stream = GroupDecodingStream::<u64, _>::new(BitsWrapper::Owned(int_stream));
        let int_stream = U64ToI64Decoder::new(Box::new(int_stream));
        let int_stream = AddingDecoder::new(Box::new(int_stream));

        let Some(str_stream) = streams.next() else {
            return Err(TimeSeriesError::MissingStrStream);
        };
        let str_stream = InflateStream::new(BitsWrapper::Owned(str_stream));
        let str_stream =
            GroupDecodingStream::<StrWrapper<'a>, _>::new(BitsWrapper::Owned(str_stream));

        Ok(Self {
            metadata,
            float_decoder: Box::new(float_stream),
            time_decoder: Box::new(time_stream),
            int_decoder: Box::new(int_stream),
            str_decoder: Box::new(str_stream),
        })
    }

    pub fn float_reader(self) -> CodedTimeSeriesFloatReader<'a> {
        CodedTimeSeriesFloatReader {
            reader: self,
            last_item: None,
        }
    }
    pub fn int_reader(self) -> CodedTimeSeriesIntReader<'a> {
        CodedTimeSeriesIntReader {
            reader: self,
            last_item: None,
        }
    }
    pub fn str_reader(self) -> CodedTimeSeriesStrReader<'a> {
        CodedTimeSeriesStrReader {
            reader: self,
            last_item: None,
        }
    }

    pub fn metadata(&self) -> impl Iterator<Item = (&String, &String)> {
        self.metadata.iter()
    }
}
pub struct CodedTimeSeriesFloatReader<'a> {
    reader: CodedTimeSeriesReader<'a>,
    last_item: Option<Sample64>,
}
impl<'a> CodedTimeSeriesFloatReader<'a> {
    pub fn peek(&mut self) -> Result<&mut Option<Sample64>, Error> {
        if self.last_item.is_some() {
            Ok(&mut self.last_item)
        } else {
            if let Some(v) = self.next() {
                let v = v?;
                self.last_item = Some(v);
            }
            Ok(&mut self.last_item)
        }
    }
}
impl<'a> Iterator for CodedTimeSeriesFloatReader<'a> {
    type Item = Result<Sample64, Error>;

    fn next(&mut self) -> Option<Result<Sample64, Error>> {
        let r1 = self.reader.float_decoder.next();
        let r2 = self.reader.time_decoder.next();
        let float = match r1 {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let time = match r2 {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let float = float?;
        let time = time?;
        let samp = Sample64 {
            value: float,
            time: Time64::from_unix_raw(time),
        };
        self.last_item = Some(samp);
        Some(Ok(samp))
    }
}
pub struct CodedTimeSeriesIntReader<'a> {
    reader: CodedTimeSeriesReader<'a>,
    last_item: Option<IntSample64>,
}
impl<'a> CodedTimeSeriesIntReader<'a> {
    pub fn peek(&mut self) -> Result<&mut Option<IntSample64>, Error> {
        if self.last_item.is_some() {
            Ok(&mut self.last_item)
        } else {
            if let Some(v) = self.next() {
                let v = v?;
                self.last_item = Some(v);
            }
            Ok(&mut self.last_item)
        }
    }
}
impl<'a> Iterator for CodedTimeSeriesIntReader<'a> {
    type Item = Result<IntSample64, Error>;

    fn next(&mut self) -> Option<Result<IntSample64, Error>> {
        let r1 = self.reader.int_decoder.next();
        let r2 = self.reader.time_decoder.next();
        let val = match r1 {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let time = match r2 {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let val = val?;
        let time = time?;
        let samp = IntSample64 {
            value: val,
            time: Time64::from_unix_raw(time),
        };
        self.last_item = Some(samp);
        Some(Ok(samp))
    }
}
pub struct CodedTimeSeriesStrReader<'a> {
    reader: CodedTimeSeriesReader<'a>,
    last_item: Option<StrSample64<'a>>,
}
impl<'a> CodedTimeSeriesStrReader<'a> {
    pub fn peek(&mut self) -> Result<&mut Option<StrSample64<'a>>, Error> {
        if self.last_item.is_some() {
            Ok(&mut self.last_item)
        } else {
            if let Some(v) = self.next() {
                let v = v?;
                self.last_item = Some(v);
            }
            Ok(&mut self.last_item)
        }
    }
}
impl<'a> Iterator for CodedTimeSeriesStrReader<'a> {
    type Item = Result<StrSample64<'a>, Error>;

    fn next(&mut self) -> Option<Result<StrSample64<'a>, Error>> {
        let r1 = self.reader.str_decoder.next();
        let r2 = self.reader.time_decoder.next();
        let val = match r1 {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let time = match r2 {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let val = val?;
        let time = time?;
        let samp = StrSample64 {
            value: val,
            time: Time64::from_unix_raw(time),
        };
        self.last_item = Some(samp.clone());
        Some(Ok(samp))
    }
}

#[cfg(test)]
mod tests {
    use crate::tsdf::Sample64;
    use crate::tsdf::{CodedTimeSeriesReader, CodedTimeSeriesWriter};
    use irox_bits::Error;
    use irox_time::Time64;
    use irox_tools::buf::UnlimitedBuffer;
    use irox_tools::random::{Random, PRNG};
    use irox_units::units::duration::Duration;
    use std::time::Instant;

    #[test]
    pub fn test() -> Result<(), Error> {
        let mut data = UnlimitedBuffer::<Sample64>::new();
        {
            let file = CodedTimeSeriesWriter::new("test_file.tsd")?;
            let mut file = file.float_stream();
            // let mut buf2 = UnlimitedBuffer::<u8>::new();
            let mut input = Time64::now();
            let incr = Duration::from_millis(100);
            let start = Instant::now();
            let count = 20_000_000u64;
            let center = 100f64;
            let variance = 0.001f64;
            let mut rand = Random::default();
            {
                // let mut cbuf1 = CompressStream::new(BitsWrapper::Borrowed(&mut buf1));
                // let mut cbuf2 = CompressStream::new(BitsWrapper::Borrowed(&mut buf2));
                for _i in 0..count {
                    let val = rand.next_in_range(0., 4096.); // 12-bit A2D
                                                             // let val = rand.next_in_range(0., 8192.); // 13-bit A2D
                                                             // let val = rand.next_in_range(0., 16384.); // 14-bit A2D
                    let val = center + val.round() * variance - variance / 2f64;
                    // let val = (_i as f64) * center + val;
                    // println!("value: {val} // {:08X}", val.to_bits());
                    // cbuf1.write_value(input.as_u64())?;
                    // cbuf2.write_value(val)?;
                    let samp = Sample64::new(input, val);
                    data.push_back(samp);
                    file.write_sample(&samp)?;
                    input += incr;
                }
                file.flush()?;
                // drop(cbuf1);
                // drop(cbuf2);
            }
            let written = std::fs::metadata("test_file.tsd")?.len();
            let input_size = count * 16;
            let end = start.elapsed();
            // irox_tools::hex::HexDump::hexdump(&buf);
            let ratio = 1. - (written as f64 / input_size as f64);
            let ratio = ratio * 100.;
            let ubps = input_size as f64 / end.as_secs_f64() / 1e6;
            println!(
                "Turned {input_size} bytes into {written} = {ratio:02.}% reduction = {ubps:02.02}MB/s"
            );
            println!("{:#?}", file.written_stats());
            drop(file);
        }

        let file = CodedTimeSeriesReader::new("test_file.tsd")?;
        let mut file = file.float_reader();
        let num_samps = data.len();
        assert!(num_samps > 0);
        let mut idx = 0;
        loop {
            let res = file.peek()?;
            let Some(val) = res.take() else {
                break;
            };
            let Some(v) = data.pop_front() else {
                panic!("should not happen");
            };
            assert_eq!(val, v, "{idx}");
            idx += 1;
        }
        assert_eq!(num_samps, idx);
        Ok(())
    }
}
