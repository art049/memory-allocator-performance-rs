#![feature(allocator_api)]

use criterion::{criterion_group, criterion_main, Criterion};
use memory_allocator_performance_rs::{GlibcMallocAllocator, JemallocAllocator, MiMallocAllocator};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::alloc::{Allocator, Layout, System};
use std::sync::{Arc, Mutex};
use std::thread;

const STRING_UNIT: usize = 10;
const STRING_SIZES: usize = 5;
const LARGE_UNIT: usize = 1000;
const LARGE_CHUNK_SIZES: usize = 100;
const SEED: u64 = 1;

struct ThreadSafeAllocator<A: Allocator>(Arc<Mutex<A>>);

unsafe impl<A: Allocator> Allocator for ThreadSafeAllocator<A> {
    fn allocate(&self, layout: Layout) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        self.0.lock().unwrap().allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: Layout) {
        self.0.lock().unwrap().deallocate(ptr, layout)
    }
}

impl<A: Allocator> Clone for ThreadSafeAllocator<A> {
    fn clone(&self) -> Self {
        ThreadSafeAllocator(Arc::clone(&self.0))
    }
}

struct Allocation {
    ptr: std::ptr::NonNull<u8>,
    layout: Layout,
}

unsafe impl Send for Allocation {}

fn stress<A: Allocator>(
    allocator: &A,
    allocate_count: usize,
    retain_count: usize,
    chunk_size: usize,
    retained: &mut Vec<Allocation>,
) {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let mut chunk = Vec::new();
    let mut local_allocate_count = allocate_count;
    let mut local_retain_count = retain_count;

    while local_retain_count > 0 || local_allocate_count > 0 {
        if local_retain_count == 0 || (rng.gen::<f32>() < 0.5 && local_allocate_count > 0) {
            let size = STRING_UNIT * rng.gen_range(1..=STRING_SIZES);
            let layout = Layout::array::<u8>(size).unwrap();
            let ptr = allocator
                .allocate(layout)
                .expect("Allocation failed")
                .cast::<u8>();
            chunk.push(Allocation { ptr, layout });
            local_allocate_count -= 1;
            if chunk.len() > chunk_size {
                for alloc in chunk.drain(..) {
                    unsafe {
                        allocator.deallocate(alloc.ptr, alloc.layout);
                    }
                }
                let large_size = LARGE_UNIT * LARGE_CHUNK_SIZES;
                let layout = Layout::array::<u8>(large_size).unwrap();
                let ptr = allocator
                    .allocate(layout)
                    .expect("Allocation failed")
                    .cast::<u8>();
                chunk.push(Allocation { ptr, layout });
            }
        } else {
            let size = STRING_UNIT * rng.gen_range(1..=STRING_SIZES);
            let layout = Layout::array::<u8>(size).unwrap();
            let ptr = allocator
                .allocate(layout)
                .expect("Allocation failed")
                .cast::<u8>();
            retained.push(Allocation { ptr, layout });
            local_retain_count -= 1;
        }
    }

    // Cleanup chunk
    for alloc in chunk {
        unsafe {
            allocator.deallocate(alloc.ptr, alloc.layout);
        }
    }
}

fn run_stress_test<A: Allocator + Clone + Send + Sync + 'static>(allocator: A, threads: usize) {
    let total_allocate_count = 1_000_000;
    let total_retain_count = 600_000;
    let total_chunk_size = 200_000;

    let allocator = ThreadSafeAllocator(Arc::new(Mutex::new(allocator)));
    let retained = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let allocator_clone = allocator.clone();
            let retained_clone = Arc::clone(&retained);
            thread::spawn(move || {
                let mut local_retained = Vec::new();
                stress(
                    &allocator_clone,
                    total_allocate_count / threads,
                    total_retain_count / threads,
                    total_chunk_size / threads,
                    &mut local_retained,
                );
                let mut retained = retained_clone.lock().unwrap();
                retained.extend(local_retained);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // Cleanup retained allocations
    let mut retained = retained.lock().unwrap();
    for alloc in retained.drain(..) {
        unsafe {
            allocator
                .0
                .lock()
                .unwrap()
                .deallocate(alloc.ptr, alloc.layout);
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let threads = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1);
    println!("Running with {} threads", threads);

    let mut group = c.benchmark_group("mixed_allocation_stress_test");

    group.bench_function("System", |b| b.iter(|| run_stress_test(System, threads)));

    group.bench_function("GlibcMalloc", |b| {
        b.iter(|| run_stress_test(GlibcMallocAllocator, threads))
    });

    group.bench_function("Jemalloc", |b| {
        b.iter(|| run_stress_test(JemallocAllocator::default(), threads))
    });

    group.bench_function("MiMalloc", |b| {
        b.iter(|| run_stress_test(MiMallocAllocator, threads))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
