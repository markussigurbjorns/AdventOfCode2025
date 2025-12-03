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


pub unsafe fn find_highest_two_digit_num(buf: *const c_char) -> (u32, *const c_char) {
    let mut p = buf;
    let mut first_digit = buf;
    let mut second_digit = buf;
    let mut val: u32 = 0;
    loop {
        let ch = *p;
        let peek = *p.add(1);
        if peek == b'\n' as i8 {break;}
        if ch > *first_digit {
            first_digit = p;
        }
        p = p.add(1);
    }

    p = first_digit.add(1);
    second_digit = first_digit.add(1);

    loop {
        let ch = *p;
        if ch == b'\n' as i8 {break;}
        if ch > *second_digit {
            second_digit = p;
        }
        p = p.add(1);
    }
    
    let fd = (*first_digit - b'0' as i8 ) as u32;
    val = fd;
    let sd = (*second_digit - b'0' as i8) as u32;
    val = val * 10 + sd;

    (val, p)
}


pub unsafe fn process_part1(buf: *const c_char) -> u32 {
    let mut p = buf; 
    let mut sum: u32 = 0;
    while *p != 0 {
   
        let (val, new_p) = find_highest_two_digit_num(p);
        sum += val;
        p = new_p;
        p = p.add(1);
    }
    sum
}

pub unsafe fn find_highest_n_digit_num(buf: *const c_char, n:u8) -> (u64, *const c_char) {
    
    let mut p = buf;
    let mut current_digit = buf;
    let mut val: u64 = 0;
    for i in (0..n).rev(){
        p = current_digit;
        loop {
            let ch = *p;
            let peek = *p.add(i as usize);
            if peek == b'\n' as i8 {break;}
            if ch > *current_digit {
                current_digit = p;
            }
            p = p.add(1);
        }
        let d = (*current_digit - b'0' as i8) as u64;
        val = val * 10 + d;
        current_digit = current_digit.add(1);
    }

    (val, p.add(1))

}


pub unsafe fn process_part2(buf: *const c_char) -> u64 {
    let mut p = buf; 
    let mut sum: u64 = 0;
    while *p != 0 {
   
        let (val, new_p) = find_highest_n_digit_num(p, 12_u8);
        sum += val;
        p = new_p;
    }
    sum
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
        let res1 = process_part1(text);
        printf(c"%lu\n".as_ptr(), res1);
        let res2 = process_part2(text);
        printf(c"%llu\n".as_ptr(), res2);

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
