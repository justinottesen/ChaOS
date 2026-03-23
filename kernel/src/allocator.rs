#![allow(dead_code)]

use core::alloc::{GlobalAlloc, Layout};

use crate::mem::align_up;
use crate::sync::Spinlock;

// --- BumpAllocator -----------------------------------------------------------

struct BumpInner {
    start: usize,
    size: usize,
    used: usize,
}

impl BumpInner {
    const fn empty() -> Self {
        Self { start: 0, size: 0, used: 0 }
    }

    /// Initialize the allocator over the given memory region.
    ///
    /// # Safety
    /// The caller must ensure that `[start, start + size)` is valid, writable
    /// physical memory that no other code will access for the lifetime of this
    /// allocator.
    unsafe fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.size = size;
        self.used = 0;
    }

    fn alloc(&mut self, layout: Layout) -> *mut u8 {
        // Round the current pointer up to the required alignment.
        let alloc_start = align_up(self.start + self.used, layout.align());
        let alloc_end = alloc_start + layout.size();

        if alloc_end > self.start + self.size {
            // Out of memory.
            return core::ptr::null_mut();
        }

        self.used = alloc_end - self.start;
        alloc_start as *mut u8
    }
}

pub struct BumpAllocator {
    inner: Spinlock<BumpInner>,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self { inner: Spinlock::new(BumpInner::empty()) }
    }

    /// Initialize the allocator over the given memory region.
    ///
    /// # Safety
    /// The caller must ensure that `[start, start + size)` is valid, writable
    /// physical memory that no other code will access for the lifetime of this
    /// allocator.
    pub unsafe fn init(&self, start: usize, size: usize) {
        unsafe { self.inner.lock().init(start, size) };
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.inner.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocators do not support freeing individual allocations.
        // Memory is reclaimed only when the allocator itself is replaced.
    }
}

