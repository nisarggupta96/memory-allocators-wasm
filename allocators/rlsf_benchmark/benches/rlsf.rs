#[macro_use]
extern crate criterion;

use criterion::{Criterion, Throughput};

use rlsf::Tlsf;
use std::{alloc::Layout, mem::MaybeUninit, ptr::NonNull};

const HEAP_SIZE: usize = 64 * 1024 * 1024; // 64 MB
const ALLOC_SIZE: usize = 32 * 1024 * 1024;
static mut POOL: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

fn with_allocator<F: FnOnce(Tlsf<u8, u8, 8, 8>)>(f: F) {
    // let mut pool = [MaybeUninit::uninit(); HEAP_SIZE];
    // let mut tlsf: Tlsf<u8, u8, 16, 16> = Tlsf::new();
    // tlsf.insert_free_block(&mut pool);
    let mut tlsf: Tlsf<u8, u8, 8, 8> = Tlsf::new();
    unsafe {
        tlsf.insert_free_block_ptr(NonNull::new(POOL.as_mut_ptr()).unwrap());
    }
    f(tlsf);
}

fn bench_alloc(allocator: &mut Tlsf<u8, u8, 8, 8>, alloc_size: usize) {
    for _i in 0..(ALLOC_SIZE / alloc_size) {
        match alloc_size {
            16 => allocator.allocate(Layout::new::<u16>()),
            32 => allocator.allocate(Layout::new::<u16>()),
            32 => allocator.allocate(Layout::new::<u32>()),
            64 => allocator.allocate(Layout::new::<u64>()),
            128 => allocator.allocate(Layout::new::<u128>()),
            _ => allocator.allocate(Layout::new::<u8>()),
        };
    }
}

fn bench_alloc_then_free(allocator: &mut Tlsf<u8, u8, 8, 8>, alloc_size: usize) {
    let count = ALLOC_SIZE / alloc_size;
    let mut ptrs = Vec::with_capacity(count);
    for _i in 0..count {
        ptrs.push(allocator.allocate(Layout::new::<u8>()).unwrap());
    }
    while let Some(ptr) = ptrs.pop() {
        println!("{:p}", ptr);
        unsafe {
            allocator.deallocate(ptr, Layout::new::<u8>().align());
        }
    }
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
