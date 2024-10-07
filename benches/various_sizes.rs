#![feature(allocator_api)]

use bumpalo::Bump;
use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion,
};
use memory_allocator_performance_rs::{
    ArenaAllocator, JemallocAllocator, MiMallocAllocator, SbrkAllocator,
};
use std::{
    alloc::{Allocator, Layout, System},
    ptr::addr_of_mut,
};

fn bench_allocator(c: &mut Criterion, allocator_name: &str, allocator: impl Allocator + Clone) {
    let mut g = c.benchmark_group("SingleAllocation");
    let sizes = [64, 512, 1024, 4096];
    for &size in &sizes {
        g.bench_with_input(BenchmarkId::new(allocator_name, size), &size, |b, &size| {
            let layout = Layout::from_size_align(size, 8).unwrap();
            b.iter_batched(
                || allocator.clone(),
                |allocator| {
                    let ptr = allocator.allocate(layout).unwrap().cast::<u8>();
                    unsafe { allocator.deallocate(ptr.cast(), layout) };
                },
                criterion::BatchSize::PerIteration,
            );
        });
    }
    g.finish();
    // let mut g = c.benchmark_group("MultiSizeAllocation");
    // let sizes = [64, 512, 1024, 4096];
    // g.bench_function(allocator_name, |b| {
    //     b.iter_batched(
    //         || allocator.clone(),
    //         |allocator| {
    //             for i in 0..1000 {
    //                 let size = sizes[i % sizes.len()];
    //                 let layout = Layout::from_size_align(size, 8).unwrap();
    //                 let ptr = allocator.allocate(layout).unwrap().cast::<u8>();
    //                 unsafe { allocator.deallocate(ptr.cast(), layout) };
    //             }
    //         },
    //         criterion::BatchSize::PerIteration,
    //     );
    // });
    // g.finish();
}

fn benchmark_allocators(c: &mut Criterion) {
    // bench_allocator(c, "System", System);

    // bench_allocator(
    //     c,
    //     "HeapArena_8MB",
    //     ArenaAllocator::with_capacity(8 * 1024 * 1024),
    // );

    // static mut STATIC_ARENA_MEM: [u8; 8 * 1024 * 1024] = [0; 8 * 1024 * 1024];
    // bench_allocator(
    //     c,
    //     "StaticArena_8MB",
    //     ArenaAllocator::from_static(unsafe { addr_of_mut!(STATIC_ARENA_MEM) }),
    // );
    // bench_allocator(c, "JemallocAllocator", JemallocAllocator::default());
    // bench_allocator(c, "MiMallocAllocator", MiMallocAllocator);
    // bench_allocator(c, "bumpalloAllocator", &Bump::new());
    let sbrk_allocator = SbrkAllocator::new();
    sbrk_allocator.increase_heap_size(4096).unwrap();
    bench_allocator(c, "sbrkAllocator", sbrk_allocator);
}

criterion_group!(benches, benchmark_allocators);
criterion_main!(benches);
