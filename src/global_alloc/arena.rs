use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use alloc_fmt::alloc_eprintln;

const ARENA_SIZE: usize = 128 * 1024 * 1024;

pub struct SimpleAlloc {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    pub offset: AtomicUsize,
}

/// Aligns the given offset to the given alignment.
fn align_up(offset: usize, align: usize) -> usize {
    (offset + align - 1) & !(align - 1)
}

impl SimpleAlloc {
    pub const fn new() -> Self {
        SimpleAlloc {
            arena: UnsafeCell::new([0; ARENA_SIZE]),
            offset: AtomicUsize::new(0),
        }
    }
}

unsafe impl Sync for SimpleAlloc {}

unsafe impl GlobalAlloc for SimpleAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let offset = self.offset.load(Relaxed);
        let ptr = align_up(offset, align);
        let new_offset = ptr + size;
        if new_offset > ARENA_SIZE {
            return null_mut();
        }
        self.offset.store(new_offset, Relaxed);
        let ptr = self.arena.get().cast::<u8>().add(offset);
        alloc_eprintln!(
            "allocating {:?} bytes at {:p} (align: {:?})",
            size,
            ptr,
            align
        );
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        alloc_eprintln!("deallocating {:p}", ptr);
    }
}
