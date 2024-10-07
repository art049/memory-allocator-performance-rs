#![feature(allocator_api)]

mod allocators;
mod global_alloc;

pub use allocators::arena_allocator::ArenaAllocator;
pub use allocators::glibc_allocator::GlibcMallocAllocator;
pub use allocators::jemalloc_allocator::JemallocAllocator;
pub use allocators::mimalloc_allocator::MiMallocAllocator;
pub use allocators::sbrk_allocator::SbrkAllocator;
pub use allocators::verbose_allocator::VerboseAllocator;

pub use global_alloc::arena::SimpleAlloc;
pub use global_alloc::malloc::GlibcMallocAlloc;
pub use global_alloc::sbrk::SbrkAlloc;
