// This is an adaptation of https://github.com/esp-rs/esp-idf-sys/blob/master/src/lib.rs

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
pub mod c_types {
    pub type c_void = core::ffi::c_void;
    pub type c_uchar = u8;
    pub type c_schar = i8;
    pub type c_char = i8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
}
