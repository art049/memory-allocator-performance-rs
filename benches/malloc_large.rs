#![feature(allocator_api)]
/// Strongly inspired by:
/// https://github.com/daanx/mimalloc-bench/tree/master/bench/malloc-large
use bumpalo::Bump;
use criterion::{criterion_group, criterion_main, Criterion};
use memory_allocator_performance_rs::{GlibcMallocAllocator, JemallocAllocator, MiMallocAllocator};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::alloc::{Allocator, Layout, System};
use std::ptr::NonNull;

const NUM_BUFFERS: usize = 20;
const MB: usize = 1024 * 1024;
const MIN_BUFFER_SIZE: usize = 5 * MB;
const MAX_BUFFER_SIZE: usize = 25 * MB;
const NUM_ITERATIONS: usize = 1000;

#[derive(Clone, Copy)]
struct Buffer {
    ptr: NonNull<u8>,
    layout: Layout,
}

fn bench_large_block_allocation<A: Allocator>(allocator: &A) {
    let mut buffers: [Option<Buffer>; NUM_BUFFERS] = [None; NUM_BUFFERS];
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    for _ in 0..NUM_ITERATIONS {
        let buffer_idx = rng.gen_range(0..NUM_BUFFERS);
        let new_size = rng.gen_range(MIN_BUFFER_SIZE..=MAX_BUFFER_SIZE);

        // Deallocate the old buffer if it exists
        if let Some(old_buffer) = buffers[buffer_idx].take() {
            unsafe {
                allocator.deallocate(old_buffer.ptr, old_buffer.layout);
            }
        }

        // Allocate a new buffer
        let layout = Layout::from_size_align(new_size, 1).unwrap();
        let ptr = allocator
            .allocate(layout)
            .expect("Allocation failed")
            .cast::<u8>();

        buffers[buffer_idx] = Some(Buffer { ptr, layout });
    }

    // Clean up remaining buffers
    for buffer in buffers.into_iter().flatten() {
        unsafe {
            allocator.deallocate(buffer.ptr, buffer.layout);
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_block_allocation");

    group.bench_function("System", |b| {
        b.iter(|| bench_large_block_allocation(&System))
    });

    group.bench_function("GlibcMalloc", |b| {
        b.iter(|| bench_large_block_allocation(&GlibcMallocAllocator))
    });

    group.bench_function("Jemalloc", |b| {
        b.iter(|| bench_large_block_allocation(&JemallocAllocator::default()))
    });

    group.bench_function("MiMalloc", |b| {
        b.iter(|| bench_large_block_allocation(&MiMallocAllocator))
    });

    group.bench_function("bumpallo", |b| {
        b.iter(|| bench_large_block_allocation(&&Bump::new()))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
