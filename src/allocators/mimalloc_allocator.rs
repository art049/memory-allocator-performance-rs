use std::{
    alloc::{AllocError, Allocator, GlobalAlloc, Layout},
    ptr::NonNull,
};

use mimalloc::MiMalloc;

#[derive(Clone)]
pub struct MiMallocAllocator;

unsafe impl Allocator for MiMallocAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { MiMalloc.alloc(layout) };
        if ptr.is_null() {
            Err(AllocError)
        } else {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new(ptr).unwrap(),
                layout.size(),
            ))
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        MiMalloc.dealloc(ptr.as_ptr(), layout);
    }
}
