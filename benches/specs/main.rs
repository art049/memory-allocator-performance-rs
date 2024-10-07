mod big_or_small;
mod storage_sparse;
mod world;

use std::alloc::GlobalAlloc;

use big_or_small::bench_big_or_small;
use criterion::criterion_main;
use memory_allocator_performance_rs::{GlibcMallocAlloc, SbrkAlloc};
use mimalloc::MiMalloc;
use std::alloc::System;

use storage_sparse::benches_sparse;
use world::bench_world;

#[global_allocator]
// static ALLOCATOR: System = System;
// static ALLOCATOR: MallocAlloc = MallocAlloc;
// static ALLOCATOR: Jemalloc = Jemalloc;
static ALLOCATOR: MiMalloc = MiMalloc;

// static ALLOCATOR: SbrkAlloc = SbrkAlloc::new();

criterion_main!(bench_world, bench_big_or_small, benches_sparse);
