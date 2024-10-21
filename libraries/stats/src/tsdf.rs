// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::streams::{CompressStream, DeltaCompressStream};
use alloc::sync::Arc;
use irox_bits::{BitsWrapper, Error};
use irox_time::Time64;
use irox_tools::codec::vbyte::EncodeVByteTo;
use irox_tools::f64::FloatExt;
use irox_tools::read::{MultiStreamWriter, StreamWriter};
use std::path::Path;

pub struct Sample64 {
    pub time: Time64,
    pub value: f64,
}
impl Sample64 {
    pub fn new(time: Time64, value: f64) -> Self {
        Sample64 { time, value }
    }
}

pub struct TimeSeriesFloatDataFileWriter<'a> {
    writer: Arc<MultiStreamWriter>,
    time_stream: DeltaCompressStream<'a, u64, StreamWriter>,
    exponent_stream: CompressStream<'a, StreamWriter>,
    mantissa_stream: CompressStream<'a, StreamWriter>,
    last_value: f64,
}

fn rot54(value: u64) -> u64 {
    let [_, b, c, d, e, f, g, h] = value.to_be_bytes();
    irox_bits::FromBEBytes::from_be_bytes([0, h, g, f, e, d, c, b])
}

impl<'a> TimeSeriesFloatDataFileWriter<'a> {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let writer = MultiStreamWriter::new(path)?;
        let time_stream = writer.new_stream();
        let value_stream = writer.new_stream();
        let mantissa_stream = writer.new_stream();
        let time_stream = DeltaCompressStream::new(BitsWrapper::Owned(time_stream));
        let exponent_stream = CompressStream::new(BitsWrapper::Owned(value_stream));
        let mantissa_stream = CompressStream::new(BitsWrapper::Owned(mantissa_stream));
        Ok(TimeSeriesFloatDataFileWriter {
            writer,
            time_stream,
            exponent_stream,
            mantissa_stream,
            last_value: f64::default(),
        })
    }

    pub fn write_sample(&mut self, sample: &Sample64) -> Result<(), Error> {
        let Sample64 { time, value } = sample;
        self.time_stream.write_value(time.as_u64())?;

        let delta = value - self.last_value;
        self.last_value = *value;
        let exp = delta.exponent();
        let sig = delta.significand();
        let sig = rot54(sig);
        // println!("Delta: {delta} // {} // {}", (exp as f64).log2().ceil(), (sig as f64).log2().ceil());

        sig.encode_vbyte_to(&mut self.mantissa_stream)?;
        exp.encode_vbyte_to(&mut self.exponent_stream)?;
        Ok(())
    }

    pub fn len(&self) -> Result<u64, Error> {
        self.writer.len()
    }
    pub fn is_empty(&self) -> Result<bool, Error> {
        Ok(self.len()? == 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::streams::CompressStream;
    use crate::tsdf::Sample64;
    use crate::tsdf::TimeSeriesFloatDataFileWriter;
    use irox_bits::{BitsWrapper, Error};
    use irox_time::Time64;
    use irox_tools::buf::UnlimitedBuffer;
    use irox_tools::random::{Random, PRNG};
    use irox_units::units::duration::Duration;
    use std::time::Instant;

    #[test]
    pub fn test() -> Result<(), Error> {
        let mut file = TimeSeriesFloatDataFileWriter::new("test_file.tsd")?;
        let mut buf1 = UnlimitedBuffer::<u8>::new();
        let mut buf2 = UnlimitedBuffer::<u8>::new();
        let mut input = Time64::now();
        let incr = Duration::from_millis(100);
        let start = Instant::now();
        let count = 2_000_000;
        let center = 100f64;
        let variance = 0.01f64;
        let mut rand = Random::default();
        {
            let mut cbuf1 = CompressStream::new(BitsWrapper::Borrowed(&mut buf1));
            let mut cbuf2 = CompressStream::new(BitsWrapper::Borrowed(&mut buf2));
            for _ in 0..count {
                let val = (rand.next_u16() as f64 / u16::MAX as f64) * variance - variance / 2f64
                    + center;
                // println!("value: {val} // {:08X}", val.to_bits());
                cbuf1.write_value(input.as_u64())?;
                cbuf2.write_value(val)?;
                file.write_sample(Sample64::new(input, val))?;
                input += incr;
            }
            drop(file);
            drop(cbuf1);
            drop(cbuf2);
        }
        let written = std::fs::metadata("test_file.tsd").unwrap().len();
        let input_size = count * 16;
        let end = start.elapsed();
        // irox_tools::hex::HexDump::hexdump(&buf);
        let ratio = 1. - (written as f64 / input_size as f64);
        let ratio = ratio * 100.;
        let ubps = input_size as f64 / end.as_secs_f64() / 1e6;
        println!(
            "Turned {input_size} bytes into {written} = {ratio:02.}% reduction = {ubps:02.02}MB/s"
        );

        let input_size = count * 8;
        let written = buf1.len();
        let ratio = 1. - (written as f64 / input_size as f64);
        let ratio = ratio * 100.;
        println!("Turned time {input_size} bytes into {written} = {ratio:02.}% reduction");

        let input_size = count * 8;
        let written = buf2.len();
        let ratio = 1. - (written as f64 / input_size as f64);
        let ratio = ratio * 100.;
        println!("Turned f64 {input_size} bytes into {written} = {ratio:02.}% reduction");

        let input_size = count * 16;
        let written = buf1.len() + buf2.len();
        let ratio = 1. - (written as f64 / input_size as f64);
        let ratio = ratio * 100.;
        println!("Turned total {input_size} bytes into {written} = {ratio:02.}% reduction");

        Ok(())
    }
}
