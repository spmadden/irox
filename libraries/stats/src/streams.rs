// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Streaming data encoders and decoders

use crate::cfg_feature_miniz;
use alloc::boxed::Box;
use core::ops::Sub;
use irox_bits::{Error, MutBits, WriteToBEBits};
use irox_tools::codec::vbyte::EncodeVByteTo;
use irox_types::AnyUnsignedInteger;
use irox_types::Number;

///
/// A stream impl that writes the difference between the last value and the current
/// value to the provided [`MutBits`] writer.  The previous value is initialized to 0.
pub struct DeltaStream<T: Number + WriteToBEBits + Sub<Output: WriteToBEBits>> {
    last_value: T,
    writer: Box<dyn MutBits>,
}

impl<T: Number + WriteToBEBits + Sub<Output: WriteToBEBits>> DeltaStream<T> {
    ///
    /// Create a new stream impl
    pub fn new(writer: Box<dyn MutBits>) -> Self {
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
        WriteToBEBits::write_be_to(&delta, self.writer.as_mut())?;
        Ok(())
    }
}

///
/// A stream impl that writes the varint-encoded difference between the last
/// value and the current value to the provided [`MutBits`] writer.  The previous
/// value is initialized to 0.
pub struct VByteDeltaIntStream<
    'a,
    T: AnyUnsignedInteger + Sub<Output: EncodeVByteTo> + EncodeVByteTo,
> {
    last_value: T,
    writer: Box<(dyn MutBits + 'a)>,
}

impl<'a, T: AnyUnsignedInteger + Sub<Output: EncodeVByteTo> + EncodeVByteTo>
    VByteDeltaIntStream<'a, T>
{
    /// Creates a new stream
    pub fn new(writer: Box<(dyn MutBits + 'a)>) -> VByteDeltaIntStream<T> {
        VByteDeltaIntStream {
            last_value: Default::default(),
            writer,
        }
    }

    ///
    /// Takes the delta of the last value and this value, varint-encodes it,
    /// then writes it to the provided stream.
    pub fn write_value(&mut self, value: T) -> Result<(), Error> {
        let delta = value - self.last_value;
        self.last_value = value;
        EncodeVByteTo::encode_vbyte_to(&delta, self.writer.as_mut())?;
        Ok(())
    }
}

cfg_feature_miniz! {
    use miniz_oxide::deflate::core::{compress_to_output, CompressorOxide, TDEFLFlush, TDEFLStatus};
    use miniz_oxide::deflate::CompressionLevel;
    use miniz_oxide::DataFormat;
    use alloc::collections::VecDeque;
    use irox_bits::ErrorKind;

    ///
    /// A stream impl that writes the deflated, varint-encoded difference between
    /// the last value and the current value to the provided [`MutBits`] writer.
    /// The previous value is initialized to 0.
    pub struct DeltaCompressStream<'a, T: AnyUnsignedInteger+Sub<Output: EncodeVByteTo>+EncodeVByteTo, B: MutBits> {
        last_value: T,
        writer: &'a mut B,
        inbuf: VecDeque<u8>,
        compressor: CompressorOxide,
    }
    impl<'a, T: AnyUnsignedInteger+Sub<Output: EncodeVByteTo>+EncodeVByteTo, B: MutBits> DeltaCompressStream<'a, T, B> {
        /// Create a new stream
        pub fn new(writer: &'a mut B) -> DeltaCompressStream<'a, T, B> {
            let mut compressor = CompressorOxide::default();
            compressor.set_format_and_level(DataFormat::Raw, CompressionLevel::DefaultLevel as u8);
            DeltaCompressStream {
                last_value: Default::default(),
                writer,
                inbuf: VecDeque::with_capacity(4096),
                compressor,
            }
        }

        ///
        /// Encodes & writes the value out.
        pub fn write_value(&mut self, value: T) -> Result<(), Error> {
            let delta = value - self.last_value;
            self.last_value = value;
            EncodeVByteTo::encode_vbyte_to(&delta, &mut self.inbuf)?;

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
    impl<'a, T: AnyUnsignedInteger+Sub<Output: EncodeVByteTo>+EncodeVByteTo, B: MutBits> Drop for DeltaCompressStream<'a, T, B> {
        /// Make sure the buffer is fully flushed on drop
        fn drop(&mut self) {
            let _ = self.flush();
        }
    }
}

#[cfg(all(test, feature = "miniz", feature = "std"))]
mod test {
    use crate::streams::DeltaCompressStream;
    use irox_bits::{Error, MutBitsArray};

    ///
    /// Writes out 8*1M = 8MB to the underlying stream.
    #[test]
    pub fn test1() -> Result<(), Error> {
        let mut buf = [0u8; 4096];
        let mut input = 0;
        let written = {
            let mut arr: MutBitsArray<4096> = (&mut buf).into();
            let mut vbout = DeltaCompressStream::<u64, _>::new(&mut arr);

            for i in 0..4_000_000 {
                input += 8;
                vbout.write_value(i)?;
            }
            vbout.flush()?;
            drop(vbout);
            arr.len()
        };
        // irox_tools::hex::HexDump::hexdump(&buf);
        let ratio = 1. - (written as f64 / input as f64);
        let ratio = ratio * 100.;
        println!("Turned {input} bytes into {written} = {ratio:02.}% reduction");
        Ok(())
    }
}
