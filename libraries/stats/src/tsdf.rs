// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::sampling::Sample64;
use crate::streams::{CompressStream, DeltaStream, Stream, StreamStageStats, Streamable};
use alloc::sync::Arc;
use irox_bits::{BitsWrapper, Error, MutBits, SharedCountingBits, SharedROCounter};
use irox_tools::buf::{Buffer, FixedBuf};
use irox_tools::codec::GroupVarintCodeEncoder;
use irox_tools::read::{MultiStreamWriter, StreamWriter};
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

///
/// Stream to collect values into groups of 4 and then run them through a [`GroupVarintCodeEncoder`]
pub struct GroupCodingStream<'a, T: core::hash::Hash + Eq + Streamable, B: MutBits> {
    buf: FixedBuf<4, T>,
    inner: GroupVarintCodeEncoder<'a, T, B>,
}
impl<'a, T: core::hash::Hash + Eq + Default + Copy + Streamable, B: MutBits>
    GroupCodingStream<'a, T, B>
{
    pub fn new(inner: BitsWrapper<'a, B>) -> Self {
        Self {
            buf: FixedBuf::new(),
            inner: GroupVarintCodeEncoder::new(inner),
        }
    }
    pub fn counter(&self) -> SharedROCounter {
        self.inner.counter()
    }
}
impl<'a, T: core::hash::Hash + Eq + Streamable, B: MutBits> Stream<T>
    for GroupCodingStream<'a, T, B>
{
    fn write_value(&mut self, value: T) -> Result<usize, Error> {
        if !self.buf.is_full() {
            let _ = self.buf.push_back(value);
            return Ok(0);
        }
        let d = self.buf.pop_front().unwrap_or_default();
        let c = self.buf.pop_front().unwrap_or_default();
        let b = self.buf.pop_front().unwrap_or_default();
        let a = self.buf.pop_front().unwrap_or_default();
        self.inner.encode_4(&[a, b, c, d])
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.inner.flush()
    }
}
impl<'a, T: core::hash::Hash + Eq + Streamable, B: MutBits> Drop for GroupCodingStream<'a, T, B> {
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
pub struct CodedTimeSeriesWriter {
    float_stream: Box<dyn Stream<f64>>,
    time_stream: Box<dyn Stream<u64>>,
    stats: StreamStageStats,
}

impl CodedTimeSeriesWriter {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let mut stats = StreamStageStats::default();

        let writer = MultiStreamWriter::new(path)?;

        let float_stream = writer.new_stream();
        let float_stream = SharedCountingBits::new(BitsWrapper::Owned(float_stream));
        stats.stage_counting("1.data_gz", float_stream.get_count());
        let float_stream = CompressStream::new(BitsWrapper::Owned(float_stream));
        let float_stream = SharedCountingBits::new(BitsWrapper::Owned(float_stream));
        stats.stage_counting("2.data_vgb", float_stream.get_count());
        let float_stream = GroupCodingStream::<u64, _>::new(BitsWrapper::Owned(float_stream));
        stats.stage_counting("3.data_codes", float_stream.counter());
        let float_stream = F64ToU64Stream::new(Box::new(float_stream));

        let time_stream = writer.new_stream();
        let time_stream = SharedCountingBits::new(BitsWrapper::Owned(time_stream));
        stats.stage_counting("1.time_gz", time_stream.get_count());
        let time_stream = CompressStream::new(BitsWrapper::Owned(time_stream));
        let time_stream = SharedCountingBits::new(BitsWrapper::Owned(time_stream));
        stats.stage_counting("2.time_vgb", time_stream.get_count());
        let time_stream = GroupCodingStream::<u64, _>::new(BitsWrapper::Owned(time_stream));
        stats.stage_counting("3.time_codes", time_stream.counter());
        let time_stream = DeltaStream::new(Box::new(time_stream));

        Ok(Self {
            float_stream: Box::new(float_stream),
            time_stream: Box::new(time_stream),
            stats,
        })
    }

    pub fn write_sample(&mut self, sample: &Sample64) -> Result<(), Error> {
        let Sample64 { time, value } = sample;
        self.time_stream.write_value(time.as_u64())?;
        self.float_stream.write_value(*value)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.time_stream.flush()?;
        self.float_stream.flush()?;
        Ok(())
    }
    pub fn written_stats(&self) -> Vec<String> {
        let stats = self.float_stream.written_stats();
        let mut out = self.stats.stats();
        out.push(stats);
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::tsdf::CodedTimeSeriesWriter;
    use crate::tsdf::Sample64;
    use irox_bits::Error;
    use irox_time::Time64;
    use irox_tools::random::{Random, PRNG};
    use irox_units::units::duration::Duration;
    use std::time::Instant;

    #[test]
    pub fn test() -> Result<(), Error> {
        let mut file = CodedTimeSeriesWriter::new("test_file.tsd")?;
        // let mut buf1 = UnlimitedBuffer::<u8>::new();
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
                file.write_sample(&Sample64::new(input, val))?;
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
        // println!("floats: {:#?}", file.float_stream.written_stats());
        // println!("times: {:#?}", file.time_stream.written_stats());

        // let input_size = count * 16;
        // let written = buf1.len() + buf2.len();
        // let ratio = 1. - (written as f64 / input_size as f64);
        // let ratio = ratio * 100.;
        // println!("Turned total {input_size} bytes into {written} = {ratio:02.}% reduction");

        Ok(())
    }
}
