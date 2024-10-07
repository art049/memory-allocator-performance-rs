use std::{
    alloc::{alloc, dealloc, AllocError, Allocator, Layout},
    cell::Cell,
    ptr::NonNull,
};
pub struct ArenaAllocator {
    arena: *mut u8,
    size: usize,
    offset: Cell<usize>,
    layout: Option<Layout>,
}

impl ArenaAllocator {
    pub fn with_capacity(size: usize) -> Self {
        let layout = Layout::from_size_align(size, 1).unwrap();
        let arena = unsafe { alloc(layout) };
        if arena.is_null() {
            panic!("Failed to allocate memory for arena");
        }
        ArenaAllocator {
            arena,
            size,
            offset: Cell::new(0),
            layout: Some(layout),
        }
    }

    pub fn from_ptr(ptr: *mut [u8]) -> Self {
        ArenaAllocator {
            arena: ptr as *mut u8,
            size: ptr.len(),
            offset: Cell::new(0),
            layout: None,
        }
    }
}

impl Drop for ArenaAllocator {
    fn drop(&mut self) {
        if let Some(layout) = self.layout {
            unsafe {
                dealloc(self.arena, layout);
            }
        }
    }
}

/// Aligns the given offset to the given alignment.
fn align_up(offset: usize, align: usize) -> usize {
    (offset + align - 1) & !(align - 1)
}

unsafe impl Allocator for ArenaAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let size = layout.size();
        let align = layout.align();
        let ptr_offset = align_up(self.offset.get(), align);
        let new_offset = ptr_offset + size;
        if new_offset > self.size {
            return Err(AllocError);
        }
        let ptr = unsafe { self.arena.add(ptr_offset) };
        self.offset.set(new_offset);
        Ok(NonNull::slice_from_raw_parts(
            NonNull::new(ptr).unwrap(),
            size,
        ))
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        println!("Not deallocating memory in arena");
    }
}

impl Clone for ArenaAllocator {
    fn clone(&self) -> Self {
        let layout = self.layout;

        let arena = if let Some(layout) = layout {
            unsafe { alloc(layout) }
        } else {
            self.arena
        };

        ArenaAllocator {
            arena,
            size: self.size,
            offset: Cell::new(0),
            layout,
        }
    }
}
