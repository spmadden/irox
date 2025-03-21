// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use criterion::measurement::{Measurement, ValueFormatter};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use irox_arch_x86_64::cpu::rdtsc;
use std::time::Duration;

pub struct Timer;
impl Default for Timer {
    fn default() -> Self {
        Self {}
    }
}
impl Timer {
    fn elements_per_second(&self, elems: f64, typical: f64, values: &mut [f64]) -> &'static str {
        let elems_per_second = elems * (1e9 / typical);
        let (denominator, unit) = if elems_per_second < 1000.0 {
            (1.0, " elem/s")
        } else if elems_per_second < 1000.0 * 1000.0 {
            (1000.0, "Kelem/s")
        } else if elems_per_second < 1000.0 * 1000.0 * 1000.0 {
            (1000.0 * 1000.0, "Melem/s")
        } else {
            (1000.0 * 1000.0 * 1000.0, "Gelem/s")
        };

        for val in values {
            let elems_per_second = elems * (1e9 / *val);
            *val = elems_per_second / denominator;
        }

        unit
    }
}
impl ValueFormatter for Timer {
    fn scale_values(&self, _typical: f64, _values: &mut [f64]) -> &'static str {
        "cycles"
    }

    fn scale_throughputs(
        &self,
        typical: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        match throughput {
            Throughput::Bytes(_) => {}
            Throughput::BytesDecimal(_) => {}
            Throughput::Elements(e) => {
                self.elements_per_second(*e as f64, typical, values);
                return "elems";
            }
        }
        "cycles"
    }

    fn scale_for_machines(&self, _values: &mut [f64]) -> &'static str {
        "cycles"
    }
}
impl Measurement for Timer {
    type Intermediate = u64;
    type Value = u64;

    fn start(&self) -> Self::Intermediate {
        rdtsc()
    }

    fn end(&self, i: Self::Intermediate) -> Self::Value {
        rdtsc() - i
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        *v1 + *v2
    }

    fn zero(&self) -> Self::Value {
        0
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        *value as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        self
    }
}
fn timer() -> Criterion<Timer> {
    Criterion::default()
        .with_measurement(Timer::default())
        .sample_size(500)
}

pub struct Bencher {
    pub k: [u8; 32],
    pub u: [u8; 32],
}
impl Default for Bencher {
    fn default() -> Self {
        Self {
            k: *irox_cryptids::x25519::BASE,
            u: *irox_cryptids::x25519::BASE,
        }
    }
}
impl Bencher {
    pub fn iter_once(&mut self) {
        let r = irox_cryptids::x25519::scalarmult(&self.k, &self.u).unwrap();
        self.u = self.k;
        self.k = r;
    }
}

pub fn criterion_benchmark(c: &mut Criterion<Timer>) {
    core_affinity::set_for_current(core_affinity::CoreId { id: 2 });
    let mut grp = c.benchmark_group("x25519");
    grp.warm_up_time(Duration::from_millis(100));
    grp.sample_size(250);
    grp.measurement_time(Duration::from_secs(60));
    let mut bencher = Bencher::default();
    grp.bench_function("x25519_scalarmult", |b| b.iter(|| bencher.iter_once()));

    grp.finish();
}

criterion_group! {
    name = benches;
    config = timer();
    targets = criterion_benchmark
}
criterion_main!(benches);
