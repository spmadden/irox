// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use irox_tools::random::{PcgXslRrRr, Random, PRNG};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rnd = Random::new_seed(0);
    let mut grp = c.benchmark_group("rnd_u32");
    grp.throughput(Throughput::Bytes(4));
    grp.bench_function("next_u32", |b| b.iter(|| rnd.next_u32()));
    grp.finish();

    let mut rnd = PcgXslRrRr::new_seed(0);
    let mut grp = c.benchmark_group("rnd_u128");
    grp.throughput(Throughput::Bytes(16));
    grp.bench_function("next_u128", |b| b.iter(|| rnd.next_u128()));
    grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
