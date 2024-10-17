// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use irox_metrics::{random_data, RoadWarrior};

pub fn criterion_benchmark(c: &mut Criterion) {
    let data = random_data();
    let mut eph = RoadWarrior::new();

    c.bench_with_input(BenchmarkId::new("encode", ""), &data, |b, data| {
        b.iter(|| {
            eph.seal(&data);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
