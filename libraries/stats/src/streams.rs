// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Streaming data encoders and decoders

use crate::cfg_feature_miniz;
use core::fmt::UpperHex;
use core::ops::Sub;
use irox_bits::{Error, MutBits, WriteToBEBits};
use irox_tools::codec::vbyte::EncodeVByteTo;
use irox_tools::WrappingSub;
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub trait Streamable:
    Sized + Default + Copy + WriteToBEBits + Sub<Output: WriteToBEBits> + WrappingSub
{
}
impl<T> Streamable for T where
    T: Sized + Default + Copy + WriteToBEBits + Sub<Output: WriteToBEBits> + WrappingSub
{
}
pub trait StreamableVByte:
    Sized + Default + Copy + Sub<Output: EncodeVByteTo + UpperHex> + EncodeVByteTo + WrappingSub
{
}
impl<T> StreamableVByte for T where
    T: Sized + Default + Copy + Sub<Output: EncodeVByteTo + UpperHex> + EncodeVByteTo + WrappingSub
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
    pub fn write_value(&'a mut self, value: T) -> Result<(), Error> {
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
pub struct DeltaStream<'a, T: Streamable, B: MutBits> {
    last_value: T,
    writer: &'a mut B,
}

impl<'a, T: Streamable, B: MutBits> DeltaStream<'a, T, B> {
    ///
    /// Create a new stream impl
    pub fn new(writer: &'a mut B) -> Self {
        DeltaStream {
            last_value: Default::default(),
            writer,
        }
    }

    ///
    /// Deltifies the value against the previous value and writes it out.
    pub fn write_value(&mut self, value: T) -> Result<(), Error> {
        let delta = value - self.last_value;
        self.last_value = value;
        WriteToBEBits::write_be_to(&delta, self.writer)?;
        Ok(())
    }
}
pub struct VByteIntStream<'a, B: MutBits> {
    writer: &'a mut B,
}
impl<'a, B: MutBits> VByteIntStream<'a, B> {
    pub fn new(writer: &'a mut B) -> Self {
        Self { writer }
    }
    pub fn write_value<T: StreamableVByte>(&mut self, value: T) -> Result<(), Error> {
        EncodeVByteTo::encode_vbyte_to(&value, self.writer)
    }
}
macro_rules! impl_mutbits_for_stream {
    () => {
        fn write_u8(&mut self, val: u8) -> Result<(), Error> {
            self.write_value(val)
        }

        fn write_be_u16(&mut self, val: u16) -> Result<(), Error> {
            self.write_value(val)
        }

        fn write_be_u32(&mut self, val: u32) -> Result<(), Error> {
            self.write_value(val)
        }

        fn write_be_u64(&mut self, val: u64) -> Result<(), Error> {
            self.write_value(val)
        }

        fn write_be_u128(&mut self, val: u128) -> Result<(), Error> {
            self.write_value(val)
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
pub struct VByteDeltaIntStream<'a, T: StreamableVByte, B: MutBits> {
    last_value: T,
    writer: VByteIntStream<'a, B>,
}

impl<'a, T: StreamableVByte, B: MutBits> VByteDeltaIntStream<'a, T, B> {
    /// Creates a new stream
    pub fn new(writer: &mut B) -> VByteDeltaIntStream<T, B> {
        VByteDeltaIntStream {
            last_value: Default::default(),
            writer: VByteIntStream::new(writer),
        }
    }

    ///
    /// Takes the delta of the last value and this value, varint-encodes it,
    /// then writes it to the provided stream.
    pub fn write_value(&mut self, value: T) -> Result<(), Error> {
        let delta = value - self.last_value;
        self.last_value = value;
        EncodeVByteTo::encode_vbyte_to(&delta, &mut self.writer)
    }
}

cfg_feature_miniz! {
    use miniz_oxide::deflate::core::{compress_to_output, CompressorOxide, TDEFLFlush, TDEFLStatus};
    use miniz_oxide::deflate::CompressionLevel;
    use miniz_oxide::DataFormat;
    use alloc::collections::VecDeque;
    use irox_bits::{ErrorKind, BitsWrapper};

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
                inbuf: VecDeque::with_capacity(4096),
                compressor,
                written: 0,
            }
        }

        pub fn write_value<V: Streamable>(
                &mut self, value: V) -> Result<(), Error> {
            // println!("writing {value:08X}");
            WriteToBEBits::write_be_to(&value, &mut self.inbuf)?;
            if self.inbuf.len() < 4000 {
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

    ///
    /// A stream impl that writes the deflated, varint-encoded difference between
    /// the last value and the current value to the provided [`MutBits`] writer.
    /// The previous value is initialized to 0.
    pub struct DeltaCompressStream<'a, T: Streamable, B: MutBits> {
        last_value: T,
        compressor: CompressStream<'a, B>
    }
    impl<'a, T: Streamable, B: MutBits> DeltaCompressStream<'a, T, B> {
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
    impl<'a, T: Streamable, B: MutBits> Drop for DeltaCompressStream<'a, T, B> {
        /// Make sure the buffer is fully flushed on drop
        fn drop(&mut self) {
            let _ = self.flush();
        }
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
