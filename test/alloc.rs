#![no_std]
#![no_main]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

extern "C" {
    fn kolibri_alloc(size: usize) -> *mut u8;
    fn kolibri_free(ptr: *mut u8);
}

struct KolibriAllocator;

unsafe impl GlobalAlloc for KolibriAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        kolibri_alloc(layout.size())
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        kolibri_free(ptr);
    }
}

#[global_allocator]
static ALLOCATOR: KolibriAllocator = KolibriAllocator;
