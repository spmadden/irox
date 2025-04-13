// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use irox_tools::hash::murmur3::{Murmur3_128, Murmur3_32};
use irox_tools::hash::{BLAKE2b512, BLAKE2s256, SHA256, SHA512};
use std::time::Duration;

struct Hasher {
    iter: [u8; 128],
}
impl Default for Hasher {
    fn default() -> Self {
        Self { iter: [0; 128] }
    }
}
impl Hasher {
    pub fn hash_murmur_3(&mut self) {
        let hash = Murmur3_128::new();
        let _ = hash.hash(&self.iter);
        self.iter[0] += 1;
    }
    pub fn hash_murmur_32(&mut self) {
        let hash = Murmur3_32::new();
        let _ = hash.hash(&self.iter);
        self.iter[0] += 1;
    }
    pub fn hash_sha356(&mut self) {
        let _hash = SHA256::new().hash(&self.iter);
        self.iter[0] += 1;
    }
    pub fn hash_sha512(&mut self) {
        let _hash = SHA512::new().hash(&self.iter);
        self.iter[0] += 1;
    }
    pub fn hash_blake2s(&mut self) {
        let _hash = BLAKE2s256::default().hash(&self.iter);
        self.iter[0] += 1;
    }
    pub fn hash_blake2b(&mut self) {
        let _hash = BLAKE2b512::default().hash(&self.iter);
        self.iter[0] += 1;
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut hasher = Hasher::default();
    let mut grp = c.benchmark_group("sha512");
    grp.throughput(Throughput::Bytes(128));
    grp.bench_function("hash_sha512", |b| {
        b.iter(|| {
            hasher.hash_sha512();
        })
    });
    grp.finish();
    std::thread::sleep(Duration::from_secs(20));
    let mut grp = c.benchmark_group("sha256");
    grp.throughput(Throughput::Bytes(128));
    grp.bench_function("hash_sha256", |b| {
        b.iter(|| {
            hasher.hash_sha356();
        })
    });
    grp.finish();
    std::thread::sleep(Duration::from_secs(20));
    let mut grp = c.benchmark_group("murmur3_128");
    grp.throughput(Throughput::Bytes(128));
    grp.bench_function("hash_murmur_3", |b| {
        b.iter(|| {
            hasher.hash_murmur_3();
        })
    });
    grp.finish();
    std::thread::sleep(Duration::from_secs(20));
    let mut grp = c.benchmark_group("murmur3_32");
    grp.throughput(Throughput::Bytes(128));
    grp.bench_function("hash_murmur_32", |b| {
        b.iter(|| {
            hasher.hash_murmur_32();
        })
    });
    grp.finish();
    std::thread::sleep(Duration::from_secs(20));
    let mut grp = c.benchmark_group("blake2s256");
    grp.throughput(Throughput::Bytes(128));
    grp.bench_function("hash_blake2s256", |b| {
        b.iter(|| {
            hasher.hash_blake2s();
        })
    });
    grp.finish();
    std::thread::sleep(Duration::from_secs(20));
    let mut grp = c.benchmark_group("blake2b512");
    grp.throughput(Throughput::Bytes(128));
    grp.bench_function("hash_blake2b512", |b| {
        b.iter(|| {
            hasher.hash_blake2b();
        })
    });
    grp.finish();
}
criterion_group!(hashes, criterion_benchmark);
criterion_main!(hashes);
