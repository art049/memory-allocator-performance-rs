use std::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};
pub struct VerboseAllocator<A: Allocator> {
    inner: A,
}

impl<A: Allocator> VerboseAllocator<A> {
    pub fn new(inner: A) -> Self {
        VerboseAllocator { inner }
    }
}

unsafe impl<A: Allocator> Allocator for VerboseAllocator<A> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = self.inner.allocate(layout)?;
        println!(
            "[VerboseAllocator] Allocating {:?} bytes at {:p}",
            layout.size(),
            ptr
        );
        Ok(ptr)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        println!(
            "[VerboseAllocator] Deallocating {:?} bytes at {:p}",
            layout.size(),
            ptr
        );
        self.inner.deallocate(ptr, layout)
    }
}
