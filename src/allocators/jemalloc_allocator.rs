use std::{
    alloc::{AllocError, Allocator, GlobalAlloc, Layout},
    ptr::NonNull,
};

use jemallocator::Jemalloc;

#[derive(Clone, Default)]
pub struct JemallocAllocator {
    inner: Jemalloc,
}

unsafe impl Allocator for JemallocAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { self.inner.alloc(layout) };
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
        self.inner.dealloc(ptr.as_ptr(), layout);
    }
}
