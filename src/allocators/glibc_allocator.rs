use std::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

use libc::{free, malloc, realloc};

#[derive(Clone)]
pub struct GlibcMallocAllocator;

unsafe impl Allocator for GlibcMallocAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let size = layout.size();
        let ptr = unsafe { malloc(size) };
        if ptr.is_null() {
            Err(AllocError)
        } else {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new(ptr as *mut u8).unwrap(),
                layout.size(),
            ))
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        free(ptr.as_ptr() as *mut std::ffi::c_void);
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        _old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let new_size = new_layout.size();
        let new_ptr = realloc(ptr.as_ptr() as *mut std::ffi::c_void, new_size);
        if new_ptr.is_null() {
            Err(AllocError)
        } else {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new(new_ptr as *mut u8).unwrap(),
                new_size,
            ))
        }
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        _old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let new_size = new_layout.size();
        let new_ptr = realloc(ptr.as_ptr() as *mut std::ffi::c_void, new_size);
        if new_ptr.is_null() {
            Err(AllocError)
        } else {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new(new_ptr as *mut u8).unwrap(),
                new_size,
            ))
        }
    }
}
