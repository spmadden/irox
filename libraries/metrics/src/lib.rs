// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Secure metrology for your application & library
//!

#![forbid(unsafe_code)]

pub use error::*;
pub use gauge::*;
use irox_bits::MutBits;
use irox_time::epoch::UnixTimestamp;
use irox_time::Time64;
use irox_tools::static_init;
use irox_types::PrimitiveValue;
pub use net::*;
pub use sampling::*;
use std::sync::{Arc, Mutex};
pub use time::*;

mod error;
mod gauge;
mod net;
mod sampling;
mod time;

static_init!(get_metrics, Metrics<'static>, Metrics::new());

pub trait Metric {
    fn get_name(&self) -> &str;
    fn encode<T: MutBits>(&self, out: &mut T) -> Result<usize, Error>;
}

pub fn time_infallible<V: Into<PrimitiveValue>, F: FnMut() -> V>(mut func: F) -> Sample {
    let time: Time64 = UnixTimestamp::now().into();
    Sample::new(func(), time)
}

struct MetricsInner<'a> {
    sinks: Vec<Arc<dyn Fn(&'a Sample) + Send + Sync + 'a>>,
}
impl<'a> MetricsInner<'a> {
    fn new() -> Self {
        MetricsInner { sinks: Vec::new() }
    }
}

pub struct Metrics<'a> {
    inner: Arc<Mutex<MetricsInner<'a>>>,
}
impl<'a> Default for Metrics<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl Metrics<'static> {
    pub fn as_ref() -> &'static Metrics<'static> {
        get_metrics()
    }
}
impl<'a> Metrics<'a> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MetricsInner::new())),
        }
    }
    pub fn add_global_sink<S: Fn(&'a Sample) + Send + Sync + 'static>(&'a self, sink: S) {
        let sink = Arc::new(sink);
        if let Ok(mut lock) = self.inner.lock() {
            lock.sinks.push(sink);
        }
    }

    pub fn gauge<S: AsRef<str>>(name: S) -> Gauge {
        get_metrics().new_gauge(name)
    }

    pub fn new_gauge<S: AsRef<str>>(&self, name: S) -> Gauge {
        Gauge::new(name)
    }
}

#[cfg(test)]
mod test {
    use crate::{time_infallible, Metrics};

    #[test]
    pub fn test_gauge() {
        Metrics::as_ref().add_global_sink(|v| {
            println!("{:?}", v);
        });
        let mut gauge = Metrics::gauge("test");
        gauge.update_infallible_value(time_infallible, || 0);

        gauge.set_value(time_infallible(|| 0));
    }
}
