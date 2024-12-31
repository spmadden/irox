// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::sampling::Sample64;
use crate::streams::DeltaCompressStream;
use alloc::sync::Arc;
use irox_bits::{BitsWrapper, Error};
use irox_tools::read::{MultiStreamWriter, StreamWriter};
use std::path::Path;

macro_rules! new_bdc {
    ($writer:ident) => {
        Box::new(DeltaCompressStream::new(BitsWrapper::Owned(
            $writer.new_stream(),
        )))
    };
}

pub trait SampleCompressor {
    fn write_sample(&mut self, sample: &Sample64) -> Result<(), Error>;
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

///
/// Breaks a [`u64`] into the 8 component bytes, and then delta-compresses them individually.
pub struct EightByteStream<'a> {
    pub(crate) fb1: Box<DeltaCompressStream<'a, i8, StreamWriter>>,
    pub(crate) fb2: Box<DeltaCompressStream<'a, i8, StreamWriter>>,
    pub(crate) fb3: Box<DeltaCompressStream<'a, i8, StreamWriter>>,
    pub(crate) fb4: Box<DeltaCompressStream<'a, i8, StreamWriter>>,
    pub(crate) fb5: Box<DeltaCompressStream<'a, i8, StreamWriter>>,
    pub(crate) fb6: Box<DeltaCompressStream<'a, i8, StreamWriter>>,
    pub(crate) fb7: Box<DeltaCompressStream<'a, i8, StreamWriter>>,
    pub(crate) fb8: Box<DeltaCompressStream<'a, i8, StreamWriter>>,
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
    pub fn write_value(&mut self, v: u64) -> Result<(), Error> {
        let [a, b, c, d, e, f, g, h] = v.to_be_bytes();
        self.fb1.write_value(a as i8)?;
        self.fb2.write_value(b as i8)?;
        self.fb3.write_value(c as i8)?;
        self.fb4.write_value(d as i8)?;
        self.fb5.write_value(e as i8)?;
        self.fb6.write_value(f as i8)?;
        self.fb7.write_value(g as i8)?;
        self.fb8.write_value(h as i8)?;
        Ok(())
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

    pub fn flush(&mut self) -> Result<(), Error> {
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
}
///
/// Time Series Data File.  
pub struct TimeSeriesFloatDataFileWriter<'a> {
    writer: Arc<MultiStreamWriter>,
    time_stream: EightByteStream<'a>,
    float_stream: EightByteStream<'a>,
    semi_last_value: f64,
    last_value: f64,
}

fn _rot54(value: u64) -> u64 {
    let [_, b, c, d, e, f, g, h] = value.to_be_bytes();
    irox_bits::FromBEBytes::from_be_bytes([0, h, g, f, e, d, c, b])
}

impl<'a> TimeSeriesFloatDataFileWriter<'a> {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let writer = MultiStreamWriter::new(path)?;
        let time_stream = EightByteStream::new(&writer);
        let float_stream = EightByteStream::new(&writer);

        Ok(TimeSeriesFloatDataFileWriter {
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

#[cfg(test)]
mod tests {
    use crate::tsdf::Sample64;
    use crate::tsdf::TimeSeriesFloatDataFileWriter;
    use irox_bits::Error;
    use irox_time::Time64;
    use irox_tools::random::{Random, PRNG};
    use irox_units::units::duration::Duration;
    use std::time::Instant;

    #[test]
    pub fn test() -> Result<(), Error> {
        let mut file = TimeSeriesFloatDataFileWriter::new("test_file.tsd")?;
        // let mut buf1 = UnlimitedBuffer::<u8>::new();
        // let mut buf2 = UnlimitedBuffer::<u8>::new();
        let mut input = Time64::now();
        let incr = Duration::from_millis(100);
        let start = Instant::now();
        let count = 20_000_000;
        let center = 100f64;
        let variance = 0.00001f64;
        let mut rand = Random::default();
        {
            // let mut cbuf1 = CompressStream::new(BitsWrapper::Borrowed(&mut buf1));
            // let mut cbuf2 = CompressStream::new(BitsWrapper::Borrowed(&mut buf2));
            for _i in 0..count {
                let val = (rand.next_u16() as f64 / u16::MAX as f64) * variance - variance / 2f64;
                let val = (_i as f64).to_radians().sin() * center + val;
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
        println!("floats: {:#?}", file.float_stream.written_stats());
        println!("times: {:#?}", file.time_stream.written_stats());

        // let input_size = count * 16;
        // let written = buf1.len() + buf2.len();
        // let ratio = 1. - (written as f64 / input_size as f64);
        // let ratio = ratio * 100.;
        // println!("Turned total {input_size} bytes into {written} = {ratio:02.}% reduction");

        Ok(())
    }
}
