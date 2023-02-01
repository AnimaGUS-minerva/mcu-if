
#[macro_export]
macro_rules! null_terminate_bytes {
    ($bytes:expr) => ({
        let mut v = ($bytes).to_vec();
        v.push(0x00);
        v
    });
}

#[macro_export]
macro_rules! null_terminate_str { ($str:expr) => (crate::null_terminate_bytes!(($str).as_bytes())); }

// Similar to `std::ffi::CString` but no `std` required!
#[macro_export]
macro_rules! cstr_from { ($str:expr) => (crate::null_terminate_str!($str).as_ptr() as *const i8); }

pub fn u8_slice_from(ptr: *const u8, sz: usize) -> &'static [u8] {
    unsafe { core::slice::from_raw_parts(ptr, sz) }
}

pub fn u8_slice_mut_from(ptr: *mut u8, sz: usize) -> &'static mut [u8] {
    unsafe { core::slice::from_raw_parts_mut(ptr, sz) }
}

// TODO check
#[test]
fn test_null_terminate() {
    // use mcu_if::alloc::{format, string::ToString};

    let abcn = [97, 98, 99, 0];

    assert_eq!(b"abc\0".to_vec(), abcn);
    assert_eq!(format!("{}{}", "abc", '\0').as_bytes(), abcn);
    assert_eq!(format!("{}{}", "abc".to_string(), '\0').as_bytes(), abcn);

    assert_eq!(null_terminate_bytes!("abc".as_bytes()), abcn);
    assert_eq!(null_terminate_str!("abc"), abcn);
}
