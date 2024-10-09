#![feature(allocator_api)]
/// Strongly inspired by the glibc malloc benchmarks
/// https://github.com/bminor/glibc/tree/master/benchtests
use bumpalo::Bump;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use memory_allocator_performance_rs::{GlibcMallocAllocator, JemallocAllocator, MiMallocAllocator};
use std::alloc::{Allocator, Layout, System};
use std::ptr::{null_mut, NonNull};

const CHUNKS_TO_ALLOCATE: usize = 1600;

unsafe fn single_thread_benchmark(size: usize, allocator: &impl Allocator) {
    let mut chunks: [*mut u8; CHUNKS_TO_ALLOCATE] = [null_mut(); CHUNKS_TO_ALLOCATE];
    let layout = Layout::from_size_align_unchecked(size, std::mem::align_of::<u8>());

    // Allocation
    for a in chunks.iter_mut() {
        // Allocate and cast to *mut u8
        let ptr = allocator
            .allocate(layout)
            .expect("Allocation failed")
            .cast::<u8>()
            .as_ptr();
        *a = ptr;

        // Initialize memory
        for j in 0..size {
            ptr.add(j).write(j as u8);
        }
    }

    // Free half in FIFO order
    for a in chunks.iter().take(CHUNKS_TO_ALLOCATE / 2) {
        allocator.deallocate(NonNull::new_unchecked(*a), layout);
    }

    // Free other half in LIFO order
    for a in chunks.iter().skip(CHUNKS_TO_ALLOCATE / 2).rev() {
        allocator.deallocate(NonNull::new_unchecked(*a), layout);
    }
}

fn bench_allocator(c: &mut Criterion, allocator_name: &str, allocator: impl Allocator + Clone) {
    let mut group = c.benchmark_group("glibc_malloc_bench");
    let sizes = vec![16, 32, 64, 128, 256];
    for &size in &sizes {
        group.bench_function(BenchmarkId::new(allocator_name, size), |b| {
            b.iter(|| unsafe { single_thread_benchmark(black_box(size), allocator.by_ref()) })
        });
    }
    group.finish();
}

fn bench_allocator_multi_thread(
    c: &mut Criterion,
    allocator_name: &str,
    allocator: impl Allocator + Clone + Sync + Send + 'static,
) {
    let mut group = c.benchmark_group("glibc_malloc_bench_128B_multi_thread");
    let thread_counts = vec![4, 8, 16];
    for &thread_count in &thread_counts {
        group.bench_function(BenchmarkId::new(allocator_name, thread_count), |b| {
            b.iter(|| {
                let mut threads = Vec::new();
                for _ in 0..thread_count {
                    let allocator_clone = allocator.clone();
                    threads.push(std::thread::spawn(move || unsafe {
                        single_thread_benchmark(128, &allocator_clone);
                    }));
                }

                for thread in threads {
                    thread.join().unwrap();
                }
            })
        });
    }
    group.finish();
}

fn benchmark_allocators(c: &mut Criterion) {
    // bench_allocator(c, "System", System);
    // bench_allocator(c, "glibc_malloc", &GlibcMallocAllocator);

    // bench_allocator(c, "Jemalloc", JemallocAllocator::default());
    // bench_allocator(c, "MiMalloc", MiMallocAllocator);

    // bench_allocator(c, "bumpallo", &Bump::new());

    bench_allocator_multi_thread(c, "System", System);
    bench_allocator_multi_thread(c, "glibc_malloc", &GlibcMallocAllocator);

    bench_allocator_multi_thread(c, "Jemalloc", JemallocAllocator::default());
    bench_allocator_multi_thread(c, "MiMalloc", MiMallocAllocator);
}

criterion_group!(benches, benchmark_allocators);
criterion_main!(benches);
