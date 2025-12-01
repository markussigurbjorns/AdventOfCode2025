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

pub unsafe fn process_part1(buf: *const c_char) -> i32 {
    let mut p = buf;
    let mut res:i32 = 50;
    let mut count: i32 = 0;
    while ptr::read(p) != 0 {
        let ch = ptr::read(p);

        if ch == 'L' as i8 {
            p = p.add(1);
            let (val, new_p) = parse_number(p);
            p = new_p;               
            res = res.saturating_sub(val);
            while res < 0 {
                res = res.saturating_add(100);
            }
            if res == 0 || res == 100{
                count = count.saturating_add(1);
            }
        }
        if ch == 'R' as i8 {
            p = p.add(1);
            let (val, new_p) = parse_number(p);
            p = new_p;               
            res = res.saturating_add(val);
            while res > 100 {
                res = res.saturating_sub(100);
            }
            if res == 0 || res == 100 {
                count = count.saturating_add(1);
            }
        }


        if ptr::read(p) == b'\n' as i8 {
            p = p.add(1);
        }
    }
    count
}

pub unsafe fn reduce_mod_100(mut x: i32) -> i32 {
    while x >= 100 {
        x = x.saturating_sub(100);
    }
    while x < 0 {
        x = x.saturating_add(100);
    }
    x
}

pub unsafe fn div100(mut x: i32) -> i32 {
    let mut q:i32 = 0;
    while x >= 100 {
        x = x.saturating_sub(100);
        q = q.saturating_add(1);
    }
    q
}

pub unsafe fn distR(pos: i32) -> i32 {
    if pos == 0 {
        100
    } else {
        100_i32.saturating_sub(pos)
    }
}

pub unsafe fn distL(pos: i32) -> i32 {
    if pos == 0 {
        100
    } else {
        pos
    }
}

pub unsafe fn count_hits_R(pos: i32, steps: i32) -> i32 {
    let d = distR(pos);

    if steps < d {
        return 0;
    }

    let mut rem = steps.saturating_sub(d);
    let mut extra = div100(rem);

    1_i32.saturating_add(extra)
}

pub unsafe fn count_hits_L(pos: i32, steps: i32) -> i32 {
    let d = distL(pos);

    if steps < d {
        return 0;
    }

    let mut rem = steps.saturating_sub(d);
    let mut extra = div100(rem);

    1_i32.saturating_add(extra)
}

pub unsafe fn process_part2(buf: *const c_char) -> i32 {
    let mut p = buf;
    let mut pos: i32 = 50;
    let mut count: i32 = 0;

    while ptr::read(p) != 0 {
        let ch = ptr::read(p);

        if ch == 'L' as i8 || ch == 'R' as i8 {
            p = p.add(1);

            let (steps, new_p) = parse_number(p);
            p = new_p;

            if ch == 'L' as i8 {
                count = count.saturating_add(count_hits_L(pos, steps));

                let mut new_pos = pos.saturating_sub(steps);
                while new_pos < 0 {
                    new_pos = new_pos.saturating_add(100);
                }
                pos = reduce_mod_100(new_pos);
            } else {
                count = count.saturating_add(count_hits_R(pos, steps));

                let mut new_pos = pos.saturating_add(steps);
                while new_pos >= 100 {
                    new_pos = new_pos.saturating_sub(100);
                }
                pos = reduce_mod_100(new_pos);
            }
        }

        if ptr::read(p) == b'\n' as i8 {
            p = p.add(1);
        }
    }

    count
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
        printf(c"result p1: %d\n".as_ptr(), res1);
        let res2 = process_part2(text);
        printf(c"result p2: %d\n".as_ptr(), res2);
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
