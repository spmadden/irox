// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use irox_tools::random::{PcgRxsMXs64, PcgXshRs, PcgXslRrRr, Random, PRNG};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rnd = Random::new_seed(0);
    let mut grp = c.benchmark_group("PCG-XSH-RR");
    grp.throughput(Throughput::Bytes(4));
    grp.bench_function("next_u32", |b| b.iter(|| rnd.next_u32()));
    grp.finish();

    let mut rnd = PcgXshRs::new_seed(0);
    let mut grp = c.benchmark_group("PCG-XSH-RS");
    grp.throughput(Throughput::Bytes(4));
    grp.bench_function("next_u32", |b| b.iter(|| rnd.next_u32()));
    grp.finish();

    let mut rnd = PcgRxsMXs64::new_seed(0);
    let mut grp = c.benchmark_group("PCG-RXS-MXs64");
    grp.throughput(Throughput::Bytes(8));
    grp.bench_function("next_u64", |b| b.iter(|| rnd.next_u64()));
    grp.finish();

    let mut rnd = PcgXslRrRr::new_seed(0);
    let mut grp = c.benchmark_group("PCG-XSL-RR-RR");
    grp.throughput(Throughput::Bytes(16));
    grp.bench_function("next_u128", |b| b.iter(|| rnd.next_u128()));
    grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
