#![no_std]
#![no_main]

use core::ffi::*;
use core::panic::PanicInfo;
use libc::*;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    abort()
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: i32, _argv: *mut *mut u8) -> i32 {
    printf(c"hello, world\n".as_ptr());
    0
}

#[macro_use]
mod libc {
    use core::ffi::*;
    extern "C" {
        pub fn abort() -> !;
        pub fn printf(format: *const c_char, ...) -> c_int;
    }
}
