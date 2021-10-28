#![no_std]
#![feature(panic_unwind)]
#![feature(std_internals)]
#![feature(start)]
#![feature(c_unwind)]
#![feature(rustc_attrs)]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]
#![feature(panic_info_message)]
#![warn(unsafe_op_in_unsafe_fn)]

extern "C" {
    fn puts(str: *const u8) -> i32;
}

extern crate alloc;
mod global_alloc;
mod panic;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    if let Err(value) = panic::catch_unwind(|| {
        unsafe {
            puts(b"Hello, World!\0".as_ptr());
        }
        panic!("boom!");
    }) {
        unsafe {
            puts(b"Caught the panic!\0".as_ptr());
        }
        value.downcast_ref::<&str>().unwrap().len() as isize
    } else {
        -1
    }
}
