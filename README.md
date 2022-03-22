# mcu-if

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/AnimaGUS-minerva/voucher/blob/master/LICENSE

This crate facilitates the most common Rust programming APIs to be used under `no_std` MCU firmware development.

Specifically, it provides:
- a `no_std` equivalent of the standard Rust APIs such as `println!`, `alloc::{boxed::Box, vec, vec::Vec}`,
  and `io::{Cursor, Seek, SeekFrom, Write, ...}` assuming that the `#[global_allocator]` requirement can be satisfied by using
  [target MCU's `malloc()` and `free()` C API](https://github.com/AnimaGUS-minerva/mcu-if/blob/6543b39e2d6fbfb4662d3bb39fcdae777ee7e165/src/lib.rs#L12-L16).
- base implementations of `#[panic_handler]` and `#[alloc_error_handler]` that are required when `no_std` Rust code is
  based on the custom `#[global_allocator]` above.

For a practical firmware example based on [RIOT-OS](https://github.com/RIOT-OS/RIOT), see the [xbd-base](https://github.com/AnimaGUS-minerva/iot-rust-module-studio#examplesxbd-base) demo.

This crate requires Rust nightly.
