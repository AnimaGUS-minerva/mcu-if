#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

use alloc::alloc::{GlobalAlloc, Layout};
use super::ctypes::c_types;

struct Allocator;

extern "C" {
    fn malloc(sz: u32) -> *mut c_types::c_void;
    fn free(ptr: *mut c_types::c_void);
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size() as u32) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_types::c_void);
    }
}