use libc::{c_int, c_void, sbrk};
use std::alloc::{GlobalAlloc, Layout};
use std::ptr::null_mut;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use alloc_fmt::{alloc_eprintln, alloc_println};

pub struct SbrkAlloc {
    ptr: AtomicPtr<u8>,
    pub offset: AtomicUsize,
    size: AtomicUsize,
}

/// Aligns the given offset to the given alignment.
fn align_up(offset: usize, align: usize) -> usize {
    (offset + align - 1) & !(align - 1)
}

impl SbrkAlloc {
    pub const fn new() -> Self {
        SbrkAlloc {
            ptr: AtomicPtr::new(null_mut()),
            offset: AtomicUsize::new(0),
            size: AtomicUsize::new(0),
        }
    }
}

impl Default for SbrkAlloc {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for SbrkAlloc {}

unsafe impl GlobalAlloc for SbrkAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let offset = self.offset.load(Relaxed);
        let ptr = align_up(offset, align);
        let new_offset = ptr + size;
        if new_offset > self.size.load(Relaxed) {
            let ptr = self.increase_heap_size(size);
            if ptr.is_null() {
                return null_mut();
            }
            self.ptr.store(ptr, Relaxed);
            self.size.fetch_add(size, Relaxed);
        }
        self.offset.store(new_offset, Relaxed);
        let ptr = self.ptr.load(Relaxed).cast::<u8>().add(offset);
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

impl SbrkAlloc {
    pub fn increase_heap_size(&self, size: usize) -> *mut u8 {
        alloc_println!("increase_heap_size by {}", size);
        let ptr = unsafe { sbrk(size as c_int) };
        if ptr == -1isize as *mut c_void {
            return null_mut();
        }
        ptr as *mut u8
    }
}
