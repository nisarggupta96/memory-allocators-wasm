#[macro_use]
extern crate criterion;

use criterion::{Criterion, Throughput};
use std::alloc::{alloc, dealloc, Layout};

const HEAP_SIZE: usize = 64 * 1024 * 1024; // 64 MB
const ALLOC_SIZE: usize = 32 * 1024 * 1024;

fn bench_alloc(alloc_size: usize) {
    let count = ALLOC_SIZE / alloc_size;
    let layout = Layout::from_size_align(alloc_size, 8).unwrap();

    // Allocate memory
    let mut ptrs = Vec::with_capacity(count);
    for _i in 0..count {
        let ptr = unsafe { alloc(layout) };
        ptrs.push(ptr);
    }

    // Free memory
    for ptr in ptrs {
        unsafe { dealloc(ptr, layout) };
    }
}

fn bench_alloc_then_free(alloc_size: usize) {
    let count = ALLOC_SIZE / alloc_size;
    let layout = Layout::from_size_align(alloc_size, 8).unwrap();

    // Allocate memory
    let mut ptrs = Vec::with_capacity(count);
    for _i in 0..count {
        let ptr = unsafe { alloc(layout) };
        ptrs.push(ptr);
    }

    // Free memory
    while let Some(ptr) = ptrs.pop() {
        unsafe { dealloc(ptr, layout) };
    }
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("alloc");
    for &size in &[16, 32, 64, 128] {
        let count = ALLOC_SIZE / size;
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(format!("{} Bytes", size), &size, |b, &size| {
            b.iter(|| bench_alloc(size));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("alloc then free");
    for &size in &[16, 32, 64, 128] {
        let count = ALLOC_SIZE / size;
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(format!("{} Bytes", size), &size, |b, &size| {
            b.iter(|| bench_alloc_then_free(size));
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench
);
criterion_main!(benches);
