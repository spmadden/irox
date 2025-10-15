// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use irox_metrics::{random_data, random_keys, RoadWarrior};

pub fn criterion_benchmark(c: &mut Criterion) {
    let (pubk, _privk) = random_keys().unwrap();
    let data = random_data().unwrap();
    let mut eph = RoadWarrior::new(pubk).unwrap();

    c.bench_with_input(BenchmarkId::new("encode", ""), &data, |b, data| {
        b.iter(|| {
            eph.seal(data).unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
