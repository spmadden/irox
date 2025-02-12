// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Streaming data encoders and decoders

extern crate alloc;
use crate::cfg_feature_miniz;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::UpperHex;
use core::ops::{Add, BitXor, DerefMut, Sub};
use irox_bits::{BitsWrapper, Error, MutBits, SharedROCounter, WriteToBEBits};
use irox_tools::codec::{EncodeVByteTo, ZagZig, ZigZag};
use irox_tools::{ToSigned, ToUnsigned};
use irox_types::{AnyUnsignedInteger, NumberSigned};

pub trait Stream<T> {
    fn write_value(&mut self, value: T) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
    fn written_stats(&self) -> String {
        String::new()
    }
}
pub trait Decoder<T> {
    fn next(&mut self) -> Result<Option<T>, Error>;
}

pub trait Streamable: Sized + Default + Copy + WriteToBEBits {}
impl<T> Streamable for T where T: Sized + Default + Copy + WriteToBEBits {}
pub trait StreamableVByte:
    Sized + Default + Copy + Sub<Output: EncodeVByteTo + UpperHex> + EncodeVByteTo + WriteToBEBits + Sub
{
}
impl<T> StreamableVByte for T where
    T: Sized
        + Default
        + Copy
        + Sub<Output: EncodeVByteTo + UpperHex>
        + EncodeVByteTo
        + WriteToBEBits
        + Sub
{
}
pub trait ValueOperation<'a, T> {
    fn encode(&'a mut self, value: &T) -> Result<T, Error>;
}
pub struct CompositeStream<'a, T: Streamable, B: MutBits> {
    writer: &'a mut B,
    operations: Vec<Box<dyn ValueOperation<'a, T>>>,
}
impl<'a, T: Streamable, B: MutBits> CompositeStream<'a, T, B> {
    pub fn new(writer: &'a mut B) -> CompositeStream<'a, T, B> {
        Self {
            writer,
            operations: Vec::new(),
        }
    }
    pub fn and_then<V: ValueOperation<'a, T> + 'static>(&mut self, value: Box<V>) {
        self.operations.push(value);
    }
    pub fn write_value(&'a mut self, value: T) -> Result<usize, Error> {
        let mut v = value;
        for op in &mut self.operations {
            v = op.encode(&v)?;
        }
        WriteToBEBits::write_be_to(&value, self.writer)
    }
}

pub struct DeltaOperation<T> {
    last_value: T,
}
impl<'a, T: Sub<T, Output = T> + Copy> ValueOperation<'a, T> for DeltaOperation<T> {
    fn encode(&'a mut self, value: &T) -> Result<T, Error> {
        let out = *value - self.last_value;
        self.last_value = out;
        Ok(out)
    }
}
pub struct VByteOperation;
impl<'a, T: Sub<T, Output = T> + Copy> ValueOperation<'a, T> for VByteOperation {
    fn encode(&'a mut self, _value: &T) -> Result<T, Error> {
        todo!()
    }
}

///
/// A stream impl that writes the difference between the last value and the current
/// value to the provided [`MutBits`] writer.  The previous value is initialized to 0.
pub struct DeltaStream<T: Streamable> {
    last_value: T,
    writer: Box<dyn Stream<T>>,
}

impl<T: Streamable> DeltaStream<T> {
    ///
    /// Create a new stream impl
    pub fn new(writer: Box<dyn Stream<T>>) -> Self {
        DeltaStream {
            last_value: Default::default(),
            writer,
        }
    }
}
impl<S: Streamable + ToSigned<Output = T>, T: Streamable + NumberSigned + Sub<Output = T>> Stream<S>
    for DeltaStream<T>
{
    ///
    /// Deltifies the value against the previous value and writes it out.
    fn write_value(&mut self, value: S) -> Result<usize, Error> {
        let value = ToSigned::to_signed(value);
        let delta = value - self.last_value;
        self.last_value = value;
        self.writer.write_value(delta)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.writer.flush()
    }
    fn written_stats(&self) -> String {
        self.writer.written_stats()
    }
}
pub struct AddingDecoder<T: Streamable> {
    last_value: T,
    reader: Box<dyn Decoder<T>>,
}
impl<T: Streamable> AddingDecoder<T> {
    pub fn new(reader: Box<dyn Decoder<T>>) -> Self {
        AddingDecoder {
            last_value: Default::default(),
            reader,
        }
    }
}
impl<T: Streamable + ToUnsigned<Output = R> + Add<Output = T>, R: Streamable> Decoder<R>
    for AddingDecoder<T>
{
    fn next(&mut self) -> Result<Option<R>, Error> {
        let a = self.reader.next()?;
        let Some(a) = a else {
            return Ok(None);
        };
        let next = a + self.last_value;
        self.last_value = next;
        let next = ToUnsigned::to_unsigned(next);
        Ok(Some(next))
    }
}

pub struct ZigZagStream<T> {
    writer: Box<dyn Stream<T>>,
}
impl<T: Streamable> ZigZagStream<T> {
    pub fn new(writer: Box<dyn Stream<T>>) -> Self {
        Self { writer }
    }
}
impl<D: Streamable, S: Streamable + ZigZag<Output = D>> Stream<S> for ZigZagStream<D> {
    fn write_value(&mut self, value: S) -> Result<usize, Error> {
        self.writer.write_value(ZigZag::zigzag(value))
    }
}

pub struct ZigZagDecoder<T> {
    reader: Box<dyn Decoder<T>>,
}
impl<T: Streamable> ZigZagDecoder<T> {
    pub fn new(reader: Box<dyn Decoder<T>>) -> Self {
        Self { reader }
    }
}
impl<D: Streamable, S: Streamable + ZagZig<Output = D>> Decoder<D> for ZigZagDecoder<S> {
    fn next(&mut self) -> Result<Option<D>, Error> {
        let a = self.reader.next()?;
        let Some(a) = a else {
            return Ok(None);
        };
        let a = ZagZig::zagzig(a);
        Ok(Some(a))
    }
}

pub struct I64ToU64Stream {
    writer: Box<dyn Stream<u64>>,
}
impl I64ToU64Stream {
    pub fn new(writer: Box<dyn Stream<u64>>) -> Self {
        Self { writer }
    }
}
impl Stream<i64> for I64ToU64Stream {
    fn write_value(&mut self, value: i64) -> Result<usize, Error> {
        self.writer.write_value(value.as_u64())
    }
}
pub struct U64ToI64Decoder {
    reader: Box<dyn Decoder<u64>>,
}
impl U64ToI64Decoder {
    pub fn new(reader: Box<dyn Decoder<u64>>) -> Self {
        Self { reader }
    }
}
impl Decoder<i64> for U64ToI64Decoder {
    fn next(&mut self) -> Result<Option<i64>, Error> {
        let Some(val) = self.reader.next()? else {
            return Ok(None);
        };
        Ok(Some(val as i64))
    }
}

///
/// Basic stream to convert from a [`f64`] to a [`u64`]
pub struct F64ToU64Stream {
    writer: Box<dyn Stream<u64>>,
}
impl F64ToU64Stream {
    pub fn new(writer: Box<dyn Stream<u64>>) -> Self {
        Self { writer }
    }
}
impl Stream<f64> for F64ToU64Stream {
    fn write_value(&mut self, value: f64) -> Result<usize, Error> {
        self.writer.write_value(value.to_bits())
    }
}
pub struct U64ToF64Decoder {
    reader: Box<dyn Decoder<u64>>,
}
impl U64ToF64Decoder {
    pub fn new(reader: Box<dyn Decoder<u64>>) -> Self {
        Self { reader }
    }
}
impl Decoder<f64> for U64ToF64Decoder {
    fn next(&mut self) -> Result<Option<f64>, Error> {
        let Some(val) = self.reader.next()? else {
            return Ok(None);
        };
        Ok(Some(f64::from_bits(val)))
    }
}

pub struct XorDeltaStream<T> {
    last_value: T,
    writer: Box<dyn Stream<T>>,
}
impl<T: Sized + Default> XorDeltaStream<T> {
    pub fn new(writer: Box<dyn Stream<T>>) -> Self {
        Self {
            writer,
            last_value: Default::default(),
        }
    }
}
impl<T: Streamable + BitXor<Output = T> + Copy> Stream<T> for XorDeltaStream<T> {
    fn write_value(&mut self, value: T) -> Result<usize, Error> {
        let out = BitXor::bitxor(self.last_value, value);
        self.last_value = value;
        self.writer.write_value(out)
    }
    fn flush(&mut self) -> Result<(), Error> {
        self.writer.flush()
    }

    fn written_stats(&self) -> String {
        self.writer.written_stats()
    }
}
impl Stream<f64> for XorDeltaStream<u64> {
    fn write_value(&mut self, value: f64) -> Result<usize, Error> {
        let value = value.to_bits();
        self.write_value(value)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.writer.flush()
    }

    fn written_stats(&self) -> String {
        self.writer.written_stats()
    }
}

pub struct VByteIntStream<'a, B: MutBits> {
    writer: BitsWrapper<'a, B>,
}
impl<'a, B: MutBits> VByteIntStream<'a, B> {
    pub fn new(writer: BitsWrapper<'a, B>) -> Self {
        Self { writer }
    }
}
impl<'a, B: MutBits, T: StreamableVByte + WriteToBEBits> Stream<T> for VByteIntStream<'a, B> {
    fn write_value(&mut self, value: T) -> Result<usize, Error> {
        EncodeVByteTo::encode_vbyte_to(&value, self.writer.deref_mut())
    }
}
macro_rules! impl_mutbits_for_stream {
    () => {
        fn write_u8(&mut self, val: u8) -> Result<(), Error> {
            self.write_value(val)?;
            Ok(())
        }

        fn write_be_u16(&mut self, val: u16) -> Result<(), Error> {
            self.write_value(val)?;
            Ok(())
        }

        fn write_be_u32(&mut self, val: u32) -> Result<(), Error> {
            self.write_value(val)?;
            Ok(())
        }

        fn write_be_u64(&mut self, val: u64) -> Result<(), Error> {
            self.write_value(val)?;
            Ok(())
        }

        fn write_be_u128(&mut self, val: u128) -> Result<(), Error> {
            self.write_value(val)?;
            Ok(())
        }
    };
}
impl<'a, B: MutBits> MutBits for VByteIntStream<'a, B> {
    impl_mutbits_for_stream!();
}

///
/// A stream impl that writes the varint-encoded difference between the last
/// value and the current value to the provided [`MutBits`] writer.  The previous
/// value is initialized to 0.
pub struct VByteDeltaIntStream<'a, T, B: MutBits> {
    last_value: T,
    writer: VByteIntStream<'a, B>,
}

impl<'a, T: Streamable, B: MutBits> VByteDeltaIntStream<'a, T, B> {
    /// Creates a new stream
    pub fn new(writer: BitsWrapper<'a, B>) -> VByteDeltaIntStream<'a, T, B> {
        VByteDeltaIntStream {
            last_value: Default::default(),
            writer: VByteIntStream::new(writer),
        }
    }
}
impl<
        'a,
        T: Streamable + Sub<Output = T> + EncodeVByteTo + UpperHex + Sub<T> + NumberSigned,
        B: MutBits,
    > Stream<T> for VByteDeltaIntStream<'a, T, B>
{
    fn write_value(&mut self, value: T) -> Result<usize, Error> {
        let delta = value - self.last_value;
        self.last_value = value;
        self.writer.write_value(delta)
    }
}

cfg_feature_miniz! {
    use miniz_oxide::deflate::core::{compress_to_output, CompressorOxide, TDEFLFlush, TDEFLStatus};
    use miniz_oxide::deflate::CompressionLevel;
    use miniz_oxide::DataFormat;
    use alloc::collections::VecDeque;
    use irox_bits::{Bits, ErrorKind};
    use irox_tools::buf::{Buffer, RoundU8Buffer};

    pub struct CompressStream<'a, T: MutBits> {
        writer: BitsWrapper<'a, T>,
        inbuf: VecDeque<u8>,
        compressor: CompressorOxide,
        written: u64,
    }
    impl<'a, T: MutBits> CompressStream<'a, T> {
        pub fn new(writer: BitsWrapper<'a, T>) -> Self {
            let mut compressor = CompressorOxide::default();
            compressor.set_format_and_level(DataFormat::Raw, CompressionLevel::DefaultCompression as u8);
            Self {
                writer,
                inbuf: VecDeque::with_capacity(32768),
                compressor,
                written: 0,
            }
        }

        pub fn write_value<V: WriteToBEBits+Copy>(
                &mut self, value: V) -> Result<(), Error> {
            // println!("writing {value:08X}");
            WriteToBEBits::write_be_to(&value, &mut self.inbuf)?;
            if self.inbuf.len() < 32768 {
                return Ok(())
            }
            let (a,b) = self.inbuf.as_slices();
            let v = if a.is_empty() {
                b
            } else  {
                a
            };

            let (status, size) = compress_to_output(&mut self.compressor, v, TDEFLFlush::None, |out| {
                self.written = self.written.wrapping_add(out.len() as u64);
                self.writer.write_all_bytes(out).is_ok()
            });
            if status != TDEFLStatus::Okay {
                return Err(ErrorKind::BrokenPipe.into());
            }
            self.inbuf.drain(0..size);
            Ok(())
        }

         pub fn flush(&mut self) -> Result<(), Error> {
            loop {
                let v = self.inbuf.make_contiguous();
                let (status, size) = compress_to_output(&mut self.compressor, v, TDEFLFlush::Finish, |out| {
                    self.written = self.written.wrapping_add(out.len() as u64);
                    self.writer.write_all_bytes(out).is_ok()
                });
                self.inbuf.drain(0..size);
                return match status {
                    TDEFLStatus::BadParam => {
                        Err(ErrorKind::InvalidInput.into())
                    }
                    TDEFLStatus::PutBufFailed => {
                        Err(ErrorKind::BrokenPipe.into())
                    }
                    TDEFLStatus::Okay => {
                        continue;
                    }
                    TDEFLStatus::Done => {
                        break;
                    }
                }
            }
            Ok(())
        }
        pub fn written(&self) -> u64 {
            self.written
        }
    }
    impl<'a, B: MutBits> Drop for CompressStream<'a, B> {
        /// Make sure the buffer is fully flushed on drop
        fn drop(&mut self) {
            let _ = self.flush();
        }
    }
    impl<'a, B: MutBits> MutBits for CompressStream<'a, B> {
        impl_mutbits_for_stream!();
    }

    impl<'a, B: MutBits, T: Streamable> Stream<T> for CompressStream<'a, B> {
        fn write_value(&mut self, value: T) -> Result<usize, Error> {
            WriteToBEBits::write_be_to(&value, self)
        }

        fn flush(&mut self) -> Result<(), Error> {
            Self::flush(self)
        }

        fn written_stats(&self) -> String {
            format!("{}", self.written)
        }

    }

    ///
    /// A stream impl that writes the deflated, varint-encoded difference between
    /// the last value and the current value to the provided [`MutBits`] writer.
    /// The previous value is initialized to 0.
    pub struct DeltaCompressStream<'a, T: Streamable+Copy, B: MutBits> {
        last_value: T,
        compressor: CompressStream<'a, B>
    }
    impl<'a, T: Streamable+Copy, B: MutBits> DeltaCompressStream<'a, T, B> {
        /// Create a new stream
        pub fn new(writer: BitsWrapper<'a, B>) -> DeltaCompressStream<'a, T, B> {
            DeltaCompressStream {
                last_value: Default::default(),
                compressor: CompressStream::new(writer),
            }
        }

        ///
        /// Encodes & writes the value out.
        pub fn write_value(&mut self, value: T) -> Result<(), Error> {

            let delta = value; //value.wrapping_sub(self.last_value);
            self.last_value = value;
            // println!("Delta: {delta:08X}");
            // EncodeVByteTo::encode_vbyte_to(&delta, &mut self.compressor)?;
            self.compressor.write_value(delta)?;
            Ok(())
        }

        pub fn flush(&mut self) -> Result<(), Error> {
            self.compressor.flush()
        }

        pub fn written(&self) -> u64 {
            self.compressor.written()
        }

    }
    impl<'a, T: Streamable+Copy, B: MutBits> Drop for DeltaCompressStream<'a, T, B> {
        /// Make sure the buffer is fully flushed on drop
        fn drop(&mut self) {
            let _ = self.flush();
        }
    }

    pub struct InflateStream<'a, B: irox_bits::BufBits> {
        reader: BitsWrapper<'a, B>,
        out_buffer: RoundU8Buffer<4096>,
        inflater: miniz_oxide::inflate::stream::InflateState,
    }
    impl<'a, B: irox_bits::BufBits> InflateStream<'a, B> {
        pub fn new(reader: BitsWrapper<'a, B>) -> Self {
            let inflater = miniz_oxide::inflate::stream::InflateState::new(DataFormat::Raw);
            let out_buffer = RoundU8Buffer::<4096>::default();
            Self {
                reader,
                out_buffer,
                inflater
            }
        }
        pub fn has_more(&mut self) -> Result<bool, Error> {
            if self.out_buffer.is_empty() {
                self.try_fill_buf()?;
            }
            Ok(!self.out_buffer.is_empty())
        }
        fn try_fill_buf(&mut self) -> Result<(), Error> {
            if !self.out_buffer.is_empty() {
                return Ok(());
            }
            self.out_buffer.clear();

            let outbuf = self.out_buffer.as_ref_mut_available();
            let inbuf = self.reader.fill_buf()?;
            let res = miniz_oxide::inflate::stream::inflate(&mut self.inflater, inbuf, outbuf, miniz_oxide::MZFlush::None);
            self.reader.consume(res.bytes_consumed);
            self.out_buffer.mark_some_used(res.bytes_written)?;
            Ok(())
        }
    }
    impl<'a, B: irox_bits::BufBits> Bits for InflateStream<'a, B> {
        fn next_u8(&mut self) -> Result<Option<u8>, Error> {
            if let Some(v) = self.out_buffer.pop_front() {
                return Ok(Some(v));
            }
            self.try_fill_buf()?;

            Ok(self.out_buffer.pop_front())
        }

    }
}
#[derive(Default)]
pub struct StreamStageStats {
    stats: BTreeMap<String, Box<dyn Fn() -> String>>,
}
impl StreamStageStats {
    pub fn stage(&mut self, name: &str, value: Box<dyn Fn() -> String>) {
        self.stats.insert(name.to_string(), value);
    }
    pub fn stage_counting(&mut self, name: &str, value: SharedROCounter) {
        self.stage(name, Box::new(move || value.get_count().to_string()))
    }
    pub fn stats(&self) -> Vec<String> {
        self.stats
            .iter()
            .map(|(k, v)| format!("{k}: {}", v()))
            .collect::<Vec<String>>()
    }
}

#[cfg(all(test, feature = "miniz", feature = "std"))]
mod test {
    use crate::streams::{BitsWrapper, DeltaCompressStream};
    use irox_bits::Error;
    use irox_time::Time64;
    use irox_units::units::duration::Duration;
    use std::time::Instant;

    ///
    /// Writes out 8*1M = 8MB to the underlying stream.
    #[test]
    pub fn test1() -> Result<(), Error> {
        let mut buf = Vec::with_capacity(32768);
        let mut input = 0;
        let start = Instant::now();
        let written = {
            let wrapper = BitsWrapper::Borrowed(&mut buf);
            let mut vbout = DeltaCompressStream::<u64, _>::new(wrapper);

            for i in 0..4_000_000 {
                input += 8;
                vbout.write_value(i)?;
            }
            vbout.flush()?;
            drop(vbout);
            buf.len()
        };
        let end = start.elapsed();
        // irox_tools::hex::HexDump::hexdump(&buf);
        let ratio = 1. - (written as f64 / input as f64);
        let ratio = ratio * 100.;
        let ubps = input as f64 / end.as_secs_f64() / 1e6;
        println!("Turned {input} bytes into {written} = {ratio:02.}% reduction = {ubps:02.02}MB/s");
        Ok(())
    }

    #[test]
    pub fn test_nanos() -> Result<(), Error> {
        let mut buf = Vec::with_capacity(32768);
        let mut input = Time64::now();
        let incr = Duration::from_millis(100);
        let start = Instant::now();
        let count = 2_000_000;
        let written = {
            let wrapper = BitsWrapper::Borrowed(&mut buf);
            let mut vbout = DeltaCompressStream::new(wrapper);

            for _ in 0..count {
                input += incr;
                vbout.write_value(input.as_u64())?;
            }
            vbout.flush()?;
            drop(vbout);
            buf.len()
        };
        let count = count * 8;
        let end = start.elapsed();
        // irox_tools::hex::HexDump::hexdump(&buf);
        let ratio = 1. - (written as f64 / count as f64);
        let ratio = ratio * 100.;
        let ubps = count as f64 / end.as_secs_f64() / 1e6;
        println!("Turned {count} bytes into {written} = {ratio:02.}% reduction = {ubps:02.02}MB/s");

        Ok(())
    }
}
