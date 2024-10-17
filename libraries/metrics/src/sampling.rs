// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

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
}

pub enum SampleError {}
// pub type SampleSource<T> = dyn FnMut()->Result<Sample, SampleError>;
// pub trait SampleSource<T> {
//     fn take_sample(&mut self) -> Result<Sample, SampleError>;
// }

pub trait SampleSink<'a> {
    fn new_sample(&mut self, sample: &'a Sample);
}
impl<'b, 'a: 'b, F> SampleSink<'a> for F
where
    F: FnMut(&'a Sample) + 'a + Send + Sync,
{
    fn new_sample(&mut self, sample: &'a Sample) {
        self(sample)
    }
}
