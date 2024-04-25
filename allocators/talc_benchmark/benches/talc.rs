#[macro_use]
extern crate criterion;

use criterion::{Criterion, Throughput};
use std::alloc::Layout;

use talc::{ErrOnOom, Talc};

const HEAP_SIZE: usize = 64 * 1024 * 1024; // 64 MB
static mut HEAP: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];
const ALLOC_SIZE: usize = 32 * 1024 * 1024;

fn with_allocator<F: FnOnce(Talc<ErrOnOom>)>(f: F) {
    unsafe {
        let talc = Talc::new(ErrOnOom);
        f(talc)
    }
}

fn bench_alloc(allocator: &mut Talc<ErrOnOom>, alloc_size: usize) {
    let layout = Layout::from_size_align(alloc_size * 8, 8).unwrap();
    for i in 0..(ALLOC_SIZE / alloc_size) {
        unsafe { allocator.malloc(layout) };
    }
}

fn bench_alloc_then_free(allocator: &mut Talc<ErrOnOom>, alloc_size: usize) {
    let count = ALLOC_SIZE / alloc_size;
    let mut ptrs = Vec::with_capacity(count);
    let layout = Layout::from_size_align(alloc_size * 8, 8).unwrap();
    for i in 0..count {
        unsafe {
            let ptr = allocator.malloc(layout);
            ptrs.push(ptr);
        };
    }
    // for _i in 0..count {
    //     allocator.free(ptrs.pop(), layout);
    // }
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("alloc");
    for &size in &[16, 32, 64, 128] {
        let count = ALLOC_SIZE / size;
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(format!("{} Bytes", size), &size, |b, &size| {
            with_allocator(|mut allocator| b.iter(|| bench_alloc(&mut allocator, size)));
        });
    }
    group.finish();

    // let mut group = c.benchmark_group("alloc then free");
    // for &size in &[16, 32, 64, 128] {
    //     let count = ALLOC_SIZE / size;
    //     group.throughput(Throughput::Elements(count as u64));
    //     group.bench_with_input(format!("{} Bytes", size), &size, |b, &size| {
    //         with_allocator(|mut allocator| b.iter(|| bench_alloc_then_free(&mut allocator, size)));
    //     });
    // }
    // group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench
);
criterion_main!(benches);
