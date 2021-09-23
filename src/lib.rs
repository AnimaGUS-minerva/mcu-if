#![no_std]
#![feature(panic_info_message)]

pub mod utils;

mod ctypes;
pub use ctypes::c_types;

pub use libc_print::libc_println as println;
pub use core2;

extern "C" {
    fn abort() -> !;
    fn malloc(sz: u32) -> *mut c_types::c_void;
    fn free(ptr: *mut c_types::c_void);
}

//

// for #[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("🦀 panic(): ----");

    if let Some(message) = info.message() {
        println!("  message: \"{}\"", message);
    }

    if let Some(location) = info.location() {
        println!("  line: {}", location.line());
        println!("  file: '{}'", location.file());
    } else {
        println!("panic occurred but can't get location information...");
    }

    unsafe { abort(); }
}

//

pub extern crate alloc;

// for #[alloc_error_handler]
pub fn alloc_error(layout: alloc::alloc::Layout) -> ! {
    panic!("alloc_error_handler(): layout: {:?}", layout)
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

use alloc::alloc::{GlobalAlloc, Layout};

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size() as u32) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_types::c_void);
    }
}