// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::Error;
use irox_bits::MutBits;
use irox_time::Time64;
use irox_types::PrimitiveValue;

#[derive(Debug)]
pub struct Sample {
    value: PrimitiveValue,
    timestamp: Time64,
}
impl Sample {
    pub fn new<V: Into<PrimitiveValue>>(value: V, timestamp: Time64) -> Sample {
        let value = value.into();
        Sample { value, timestamp }
    }
    #[must_use]
    pub fn get_value(&self) -> &PrimitiveValue {
        &self.value
    }
    #[must_use]
    pub fn get_timestamp(&self) -> &Time64 {
        &self.timestamp
    }
    pub fn encode<T: MutBits>(&self, out: &mut T) -> Result<usize, Error> {
        let ts = self.timestamp.as_u64();
        let ty = self.value.primitive();

        out.write_be_u64(ts)?;
        out.write_u8(ty as u8)?;
        self.value.write_be_to(out)?;
        Ok(9 + ty.bytes_length())
    }
}

pub enum SampleError {}
// pub type SampleSource<T> = dyn FnMut()->Result<Sample, SampleError>;
// pub trait SampleSource<T> {
//     fn take_sample(&mut self) -> Result<Sample, SampleError>;
// }

pub trait SampleSink<'a> {
    fn new_sample(&mut self, sample: &'a Sample);
}
impl<'a, F> SampleSink<'a> for F
where
    F: FnMut(&'a Sample) + 'a + Send + Sync,
{
    fn new_sample(&mut self, sample: &'a Sample) {
        self(sample)
    }
}
