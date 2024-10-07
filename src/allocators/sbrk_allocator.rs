use libc::{c_int, c_void, sbrk};
use std::{
    alloc::{AllocError, Allocator, Layout},
    cell::Cell,
    ptr::NonNull,
};

#[derive(Clone)]
pub struct SbrkAllocator {
    inner: Cell<Inner>,
}

#[derive(Copy)]
struct Inner {
    arena: *mut u8,
    size: usize,
    offset: usize,
}

impl Clone for Inner {
    fn clone(&self) -> Self {
        Self {
            arena: self.arena,
            size: self.size,
            offset: 0,
        }
    }
}

impl SbrkAllocator {
    pub const fn new() -> Self {
        let allocator = SbrkAllocator {
            inner: Cell::new(Inner {
                arena: std::ptr::null_mut(),
                size: 0,
                offset: 0,
            }),
        };
        allocator
    }
}

const PAGE_SIZE: usize = 4096;

/// Aligns the given offset to the given alignment.
fn align_up(offset: usize, align: usize) -> usize {
    (offset + align - 1) & !(align - 1)
}

unsafe impl Allocator for SbrkAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let Inner {
            size: initial_size,
            offset: initial_offset,
            ..
        } = self.inner.get();
        let size = layout.size();
        let align = layout.align();
        let new_offset = align_up(initial_offset, align);

        if new_offset + size > initial_size {
            let missing_size = new_offset + size - initial_size;
            let new_allocation_size = align_up(missing_size, PAGE_SIZE);
            self.increase_heap_size(new_allocation_size as isize)?;
        }

        self.inner.set(Inner {
            offset: new_offset + size,

            ..self.inner.get()
        });
        let ptr = unsafe { self.inner.get().arena.add(new_offset) };
        Ok(NonNull::slice_from_raw_parts(
            NonNull::new(ptr).unwrap(),
            size,
        ))
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        // No-op: sbrk does not provide a straightforward way to release memory back to the OS.
    }
}

impl SbrkAllocator {
    pub fn increase_heap_size(&self, size: isize) -> Result<(), AllocError> {
        println!("increase_heap_size by {}", size);
        let ptr = unsafe { sbrk(size as c_int) };
        if ptr == -1isize as *mut c_void {
            return Err(AllocError);
        }
        let inner = self.inner.get();
        self.inner.set(Inner {
            arena: ptr as *mut u8,
            size: inner.size.saturating_add_signed(size),
            offset: inner.offset,
        });
        Ok(())
    }
}

impl Drop for SbrkAllocator {
    fn drop(&mut self) {}
}
