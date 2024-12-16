// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};

pub fn criterion_benchmark(c: &mut Criterion) {
    let flag = Arc::new(AtomicBool::new(true));
    let mut grp = c.benchmark_group("AtomicBool");
    grp.bench_function("atomicbool-load-relaxed", |b| {
        b.iter(|| {
            flag.load(Ordering::Relaxed);
        })
    });
    grp.bench_function("atomicbool-store-relaxed", |b| {
        b.iter(|| {
            flag.store(false, Ordering::Relaxed);
        })
    });
    grp.bench_function("atomicbool-load-ceqcst", |b| {
        b.iter(|| {
            flag.load(Ordering::SeqCst);
        })
    });
    grp.bench_function("atomicbool-store-ceqcst", |b| {
        b.iter(|| {
            flag.store(false, Ordering::SeqCst);
        })
    });
    grp.bench_function("atomicbool-load-acq", |b| {
        b.iter(|| {
            flag.load(Ordering::Acquire);
        })
    });
    grp.bench_function("atomicbool-store-rel", |b| {
        b.iter(|| {
            flag.store(false, Ordering::Release);
        })
    });
    grp.finish();

    let flag = Arc::new(RwLock::new(false));
    let mut grp = c.benchmark_group("RwLock");
    grp.bench_function("wrlock-write", |b| {
        b.iter(|| {
            let _unused = flag.write().unwrap();
        })
    });
    grp.finish();
    let flag = Arc::new(RwLock::new(false));
    let mut grp = c.benchmark_group("RwLock");
    grp.bench_function("wrlock-read", |b| {
        b.iter(|| {
            let _unused = flag.read().unwrap();
        })
    });
    grp.finish();
    let flag = Arc::new(Mutex::new(false));
    let mut grp = c.benchmark_group("Mutex");
    grp.bench_function("mutex", |b| {
        b.iter(|| {
            let _unused = flag.lock().unwrap();
        })
    });
    grp.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
