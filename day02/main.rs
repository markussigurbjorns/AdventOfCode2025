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

pub unsafe fn parse_number(mut p: *const c_char) -> (u64, *const c_char) {
    let mut val: u64 = 0;

    loop {
        let ch = *p;
        if ch == b'-' as i8 || ch == 0 || ch == b',' as i8 { break; }
        if ch < b'0' as i8 || ch > b'9' as i8 { break; }

        let d = (ch - b'0' as i8) as u64;
        val = val * 10 + d;

        p = p.add(1);
    }

    (val, p)
}


pub unsafe fn num_to_string(n:u64) -> *mut c_char {
    let buf_size: usize = 21;
    let buf = malloc(buf_size) as *mut c_char;
    if buf.is_null() {
        return ptr::null_mut();
    }
    snprintf(buf, buf_size, c"%llu".as_ptr(), n);
    buf
}

pub unsafe fn get_str_len(str: *mut c_char) -> usize {
    strlen(str)
}

pub unsafe fn is_invalid(str: *mut c_char, len: usize) -> bool {
    let half = len / 2;

    let left  = malloc(half + 1) as *mut c_char;
    let right = malloc(half + 1) as *mut c_char;

    if left.is_null() || right.is_null() {
        return false;
    }

    for i in 0..half {
        ptr::write(left.add(i), ptr::read(str.add(i)));
    }
    ptr::write(left.add(half), 0); 

    for i in 0..half {
        ptr::write(right.add(i), ptr::read(str.add(half + i)));
    }
    ptr::write(right.add(half), 0); 

    if strcmp(left, right) == 0 {
        return true;
    }
    return false


}

pub unsafe fn process_part1(buf: *const c_char) -> u64 {
    let mut p = buf;
    let mut sum:u64 = 0;
        
    while *p != 0 {
        
        let (val1, new_p) = parse_number(p);
        let (val2, new_new_p) = parse_number(new_p.add(1));

        for d in val1..=val2 {
            let num_str = num_to_string(d);
            let str_len = get_str_len(num_str);
            if str_len % 2 == 0 {
                if is_invalid(num_str, str_len) {
                    sum +=d
                }
            }
            free(num_str as *mut c_void);
        }

        p = new_new_p;
        p = p.add(1);
    }
    sum
    
}

pub unsafe fn is_invalid_p2(str: *mut c_char, len: usize) -> bool {
    let half = len/2;
    let mut res: bool = false;
    for i in 1..=half {
        if len % i != 0 {
            continue;
        }
        let mut fail: bool = false;
        let mut p = str;
        let to_cmp = malloc(i + 1) as *mut c_char;
        for j in 0..i {
            ptr::write(to_cmp.add(j), ptr::read(str.add(j)));
            ptr::write(to_cmp.add(i), 0);
        }
    
        for _k in 0..(len/i){
            if strncmp(to_cmp, p, i as usize) != 0 {
                fail = true;
                break;
            }
            p = p.add(i);
        }
        if !fail {
            res = true;
        }

        free(to_cmp as *mut c_void);
    
    }
    res

}

pub unsafe fn process_part2(buf: *const c_char) -> u64 {
    let mut p = buf;
    let mut sum:u64 = 0;
        
    while *p != 0 {
        
        let (val1, new_p) = parse_number(p);
        let (val2, new_new_p) = parse_number(new_p.add(1));

        for d in val1..=val2 {
            let num_str = num_to_string(d);
            let str_len = get_str_len(num_str);
            if is_invalid_p2(num_str, str_len) {
                sum +=d
            }
            free(num_str as *mut c_void);
        }

        p = new_new_p;
        p = p.add(1);
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
        printf(c"%llu\n".as_ptr(), res1);
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
        pub fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
        pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
        pub fn fclose(file: *mut FILE) -> c_int;
        pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
        pub fn fread(ptr: *mut c_void, size: usize, nobj: usize, stream: *mut FILE) -> usize;
        pub fn ftell(stream: *mut FILE) -> c_long;
        pub fn malloc(size: usize) -> *mut c_void;
        pub fn free(p: *mut c_void);
        pub fn strlen(cs: *const c_char) -> usize;
        pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
        pub fn strncmp(cs: *const c_char, ct: *const c_char, n: usize) -> c_int;

    }
}
