#![no_std]
#![no_main]

use core::ffi::*;
use core::ptr::*;
use core::ptr;
use core::mem::*;
use core::panic::PanicInfo;
use libc::*;
use da::*;


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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct FloorPlan {
    pub length: u32,
    pub height: u32,
    pub rolls: Array<u32>,
}

pub unsafe fn new_floor_plan() -> FloorPlan {
    FloorPlan {
        length:0,
        height:0,
        rolls: zeroed()
    }
}

pub unsafe fn process_part1(buf: *const c_char) -> u32 {
    let mut p = buf;
    let mut rolls: u32 = 0;
    let mut fp: FloorPlan = new_floor_plan();
    let mut l: u32 = 0;
    let mut h: u32 = 0;
    let mut i: u32 = 0;
    while *p != 0 {
        if *p == b'\n' as i8 {
            fp.length = l;
            h += 1;
        }
        if fp.length < 1 {
            l +=1;
        }
        if *p == b'@' as i8 {
            da_append(&mut fp.rolls, i);
        }
        i +=1;
        p = p.add(1);
    }
    fp.height = h;

    printf(c"length is %lu\n".as_ptr(), l);
    printf(c"height is %lu\n".as_ptr(), h);

    for r in 0..fp.rolls.count {
        let ro = fp.rolls.items.add(r);

        printf(c"roll has index %lu\n".as_ptr(), *ro);
    }
    rolls
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

    }
    0
}

pub mod libc {
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
        pub fn free(p: *mut c_void);
       // pub fn realloc(p: *mut c_void, size: usize) -> *mut c_void;
    }

    pub unsafe fn realloc_items<T>(ptr: *mut T, count: usize) -> *mut T {
        extern "C" {
            #[link_name = "realloc"]
            fn realloc_raw(ptr: *mut c_void, size: usize) -> *mut c_void;
        }
        realloc_raw(ptr as *mut c_void, size_of::<T>()*count) as *mut T
    }
}

pub mod da { 
    use crate::libc;
    use core::ptr;
    use core::ffi::*;


    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Array<T> {
        pub items: *mut T,
        pub count: usize,
        pub capacity: usize,
    }

    pub unsafe fn da_append<T>(da: *mut Array<T>, item: T) {
        if (*da).count >= (*da).capacity {
            if (*da).capacity == 0 {
                (*da).capacity = 256;
            } else {
                (*da).capacity *= 2;
            }
            (*da).items = libc::realloc_items((*da).items, (*da).capacity);
        }
        *((*da).items.add((*da).count)) = item;
        (*da).count += 1;
    }

    pub unsafe fn da_destroy<T>(da: *mut Array<T>) {
        libc::free((*da).items as *mut c_void);
        (*da).items = ptr::null_mut();
        (*da).count = 0;
        (*da).capacity = 0;
    }
}


