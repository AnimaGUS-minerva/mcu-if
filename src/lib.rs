#![no_std]
#![feature(panic_info_message)]

pub mod utils;

mod ctypes;
pub use ctypes::c_types;

pub use libc_print::libc_println as println;
pub use core2;

pub extern crate alloc;

#[cfg(feature = "allocator")]
mod allocator;


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

// for #[alloc_error_handler]
pub fn alloc_error(layout: alloc::alloc::Layout) -> ! {
    panic!("alloc_error_handler(): layout: {:?}", layout)
}

extern "C" {
    fn abort() -> !;
}