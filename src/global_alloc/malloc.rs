use std::alloc::{GlobalAlloc, Layout};
use std::os::raw::c_void;

use libc::{free, malloc, realloc};

pub struct GlibcMallocAlloc;

unsafe impl Sync for GlibcMallocAlloc {}

unsafe impl GlobalAlloc for GlibcMallocAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let raw_ptr = malloc(layout.size());
        raw_ptr.cast::<u8>()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr.cast::<c_void>())
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        realloc(ptr.cast::<c_void>(), new_size) as *mut u8
    }
}
