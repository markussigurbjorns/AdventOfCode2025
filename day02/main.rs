#![no_std]
#![no_main]

use core::ffi::*;
use core::ptr::*;
use core::ptr;
use core::panic::PanicInfo;
use libc::*;


#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    abort();
}

pub unsafe fn read_file(path: *const c_char) -> *mut c_char {
    
    let file = fopen(path, c"rb".as_ptr());
    
    if file.is_null() {
        printf(c"failed to open file\n".as_ptr());
        return null_mut();
    }
    fseek(file, 0, 2);
    let size = ftell(file);
    fseek(file, 0, 0);
   
    let size_usize = (size.saturating_add(1)) as usize;

    let buf = malloc(size_usize) as *mut c_char;
    if buf.is_null() {
        printf(c"malloc failed\n".as_ptr());
        fclose(file);
        return null_mut();
    }
    
    fread(buf as *mut c_void, 1, size as usize, file);
    
    ptr::write(buf.add(size as usize), 0);
    
    fclose(file);
    
    buf
}

pub unsafe fn parse_number(mut p: *const c_char) -> (i32, *const c_char) {
    let mut val: i32 = 0;

    loop {
        let ch = ptr::read(p);
        if ch == b'\n' as i8 || ch == 0 { break; }
        if ch < b'0' as i8 || ch > b'9' as i8 { break; }

        let d = (ch.saturating_sub(b'0' as i8)) as i32;
        val = val.wrapping_mul(10).wrapping_add(d);

        p = p.add(1);
    }

    (val, p)
}


pub unsafe fn add_num(num:i32) -> i32 {

    num+4_i32
}

pub unsafe fn process_part1(buf: *const c_char) -> i32 {
    
    add_num(5_i32)
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: i32, argv: *mut *mut c_char) -> i32 {
    
    if argc == 1 {
        printf(c"please provide input\n".as_ptr());
        return 0;
    }
    let arg1 = ptr::read(argv.add(1));
    let text = read_file(arg1);

    if !text.is_null() {
        // START HERE
        process_part1(text);
    }
    0
}

mod libc {
    use core::ffi::*;

    #[repr(C)]
    pub struct FILE {
        _private: [u8; 0],
    }


    extern "C" {
        pub fn abort() -> !;
        pub fn printf(format: *const c_char, ...) -> c_int;
        pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
        pub fn fclose(file: *mut FILE) -> c_int;
        pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
        pub fn fread(ptr: *mut c_void, size: usize, nobj: usize, stream: *mut FILE) -> usize;
        pub fn ftell(stream: *mut FILE) -> c_long;
        pub fn malloc(size: usize) -> *mut c_void;
    }
}
