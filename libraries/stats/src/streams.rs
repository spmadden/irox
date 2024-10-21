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

pub trait Streamable: Sized + Default + Copy + WriteToBEBits + Sub<Output: WriteToBEBits> {}
impl<T> Streamable for T where T: Sized + Default + Copy + WriteToBEBits + Sub<Output: WriteToBEBits>
{}
pub trait StreamableVByte:
    Sized + Default + Copy + Sub<Output: EncodeVByteTo + UpperHex> + EncodeVByteTo
{
}
impl<T> StreamableVByte for T where
    T: Sized + Default + Copy + Sub<Output: EncodeVByteTo + UpperHex> + EncodeVByteTo
{
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
    }
    impl<'a, T: MutBits> CompressStream<'a, T> {
        pub fn new(writer: BitsWrapper<'a, T>) -> Self {
            let mut compressor = CompressorOxide::default();
            compressor.set_format_and_level(DataFormat::Raw, CompressionLevel::BestCompression as u8);
            Self {
                writer,
                inbuf: VecDeque::with_capacity(4096),
                compressor
            }
        }

        pub fn write_value<V: Streamable>(
                &mut self, value: V) -> Result<(), Error> {
            // println!("writing {value:08X}");
            WriteToBEBits::write_be_to(&value, &mut self.inbuf)?;

            let v = self.inbuf.make_contiguous();

            let (status, size) = compress_to_output(&mut self.compressor, v, TDEFLFlush::None, |out| {
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
    pub struct DeltaCompressStream<'a, T: StreamableVByte, B: MutBits> {
        last_value: T,
        compressor: CompressStream<'a, B>
    }
    impl<'a, T: StreamableVByte, B: MutBits> DeltaCompressStream<'a, T, B> {
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
            let delta = value - self.last_value;
            self.last_value = value;
            // println!("Delta: {delta:08X}");
            EncodeVByteTo::encode_vbyte_to(&delta, &mut self.compressor)?;

            Ok(())
        }

        pub fn flush(&mut self) -> Result<(), Error> {
            self.compressor.flush()
        }

    }
    impl<'a, T: StreamableVByte, B: MutBits> Drop for DeltaCompressStream<'a, T, B> {
        /// Make sure the buffer is fully flushed on drop
        fn drop(&mut self) {
            let _ = self.flush();
        }
    }
}

#[cfg(all(test, feature = "miniz", feature = "std"))]
mod test {
    use crate::streams::{BitsWrapper, DeltaCompressStream};
    use irox_bits::{Error, MutBitsArray};
    use irox_time::Time64;
    use irox_units::units::duration::Duration;
    use std::time::Instant;

    ///
    /// Writes out 8*1M = 8MB to the underlying stream.
    #[test]
    pub fn test1() -> Result<(), Error> {
        let mut buf = [0u8; 4096];
        let mut input = 0;
        let start = Instant::now();
        let written = {
            let mut arr: MutBitsArray<4096> = (&mut buf).into();
            let wrapper = BitsWrapper::Borrowed(&mut arr);
            let mut vbout = DeltaCompressStream::<u64, _>::new(wrapper);

            for i in 0..4_000_000 {
                input += 8;
                vbout.write_value(i)?;
            }
            vbout.flush()?;
            drop(vbout);
            arr.len()
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
        let mut buf = [0u8; 16384];
        let mut input = Time64::now();
        let incr = Duration::from_millis(100);
        let start = Instant::now();
        let count = 2_000_000;
        let written = {
            let mut arr: MutBitsArray<16384> = (&mut buf).into();
            let wrapper = BitsWrapper::Borrowed(&mut arr);
            let mut vbout = DeltaCompressStream::new(wrapper);

            for _ in 0..count {
                input += incr;
                vbout.write_value(input.as_u64())?;
            }
            vbout.flush()?;
            drop(vbout);
            arr.len()
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
