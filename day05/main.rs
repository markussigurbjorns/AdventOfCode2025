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

pub unsafe fn parse_number(mut p: *const c_char) -> (u64, *const c_char) {
    let mut val: u64 = 0;

    loop {
        let ch = *p;
        if ch == b'-' as i8 || ch == 0  { break; }
        if ch < b'0' as i8 || ch > b'9' as i8 { break; }

        let d = (ch - b'0' as i8) as u64;
        val = val * 10 + d;

        p = p.add(1);
    }
    (val, p)
}

pub unsafe fn partition(arr: *mut Array<(u64,u64)>, lo: usize, hi:usize) -> usize {
    let pivot = (*(*arr).items.add(hi)).0;

    let mut idx:usize = lo;

    for j in lo..hi {
        let curr = (*arr).items.add(j);
        if (*curr).0 <= pivot {
            let tmp = da_replace(arr, j, *(*arr).items.add(idx));
            let _ =  da_replace(arr, idx, tmp);
            idx += 1;
        } 
    }
    let tmp = da_replace(arr, hi, *(*arr).items.add(idx));
    let _ =  da_replace(arr, idx, tmp);

    idx
}

pub unsafe fn sort(arr: *mut Array<(u64,u64)>, lo: usize, hi:usize) {
    if lo < hi {
        let idx = partition(arr, lo, hi);
        if idx > 0 {
            sort(arr, lo, idx-1);
        }
        sort(arr, idx+1, hi);
    }
}

pub unsafe fn merge_overlap(arr: *mut Array<(u64,u64)> ) -> Array<(u64,u64)> {

    let mut res: Array<(u64,u64)> = zeroed();
    let size = (*arr).count;
    
    for i in 0..size {
        let start = (*(*arr).items.add(i)).0;
        let mut end = (*(*arr).items.add(i)).1;

        if res.count != 0 && (*res.items.add(res.count-1)).1 >= end {
            continue;
        }
        for j in i..size {
            if (*(*arr).items.add(j)).0 <= end {
                if (*(*arr).items.add(j)).1 > end {
                    end = (*(*arr).items.add(j)).1;
                }
            }
        }
        da_append(&mut res, (start, end));
    }

    res

}

pub unsafe fn process_part1(buf: *const c_char) -> u64 {
    let mut p = buf;
    let mut fresh: u64 = 0;
    let mut array: Array<(u64,u64)> = zeroed();
    let mut interval_array: Array<(u64,u64)> = zeroed();
    let mut pre_process:bool = true;
    while *p != 0 {
        if pre_process {
            let (val1, new_p) = parse_number(p);
            let (val2, new_new_p) = parse_number(new_p.add(1));

            da_append(&mut array, (val1, val2));
            
            p = new_new_p;
            p = p.add(1);
            if *p == '\n' as i8 {
                sort(&mut array, 0_usize, array.count-1);
                interval_array = merge_overlap(&mut array);
                pre_process = false;
                p = p.add(1);
            }
        } else { 
            let (val, new_p) = parse_number(p);
            for d in 0..interval_array.count {
                let curr = *interval_array.items.add(d);
                if val >= curr.0 && val <= curr.1 {
                    fresh += 1;
                }
            }

            p = new_p;
            p = p.add(1);
        }
    }
    fresh
}


pub unsafe fn process_part2(buf: *const c_char) -> u64 {
    let mut p = buf;
    let mut fresh: u64 = 0;
    let mut array: Array<(u64,u64)> = zeroed();
    let mut interval_array: Array<(u64,u64)> = zeroed();
    let mut pre_process:bool = true;
    while *p != 0 {
        if pre_process {
            let (val1, new_p) = parse_number(p);
            let (val2, new_new_p) = parse_number(new_p.add(1));

            da_append(&mut array, (val1, val2));
            
            p = new_new_p;
            p = p.add(1);
            if *p == '\n' as i8 {
                sort(&mut array, 0_usize, array.count-1);
                interval_array = merge_overlap(&mut array);
                pre_process = false;
                p = p.add(1);
            }
        } else { 
            p = p.add(1);
        }
    }
    for d in 0..interval_array.count {
        let curr = *interval_array.items.add(d);
        fresh += (curr.1 - curr.0)+1;
    }
    
    fresh
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
        pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
        pub fn fclose(file: *mut FILE) -> c_int;
        pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
        pub fn fread(ptr: *mut c_void, size: usize, nobj: usize, stream: *mut FILE) -> usize;
        pub fn ftell(stream: *mut FILE) -> c_long;
        pub fn malloc(size: usize) -> *mut c_void;
        pub fn free(p: *mut c_void);

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
        let da_ref = &mut *da;

        if da_ref.count >= da_ref.capacity {
            if da_ref.capacity == 0 {
                da_ref.capacity = 256;
            } else {
                da_ref.capacity *= 2;
            }
            da_ref.items = libc::realloc_items(da_ref.items, da_ref.capacity);
        }

        *da_ref.items.add(da_ref.count) = item;
        da_ref.count += 1;
    }

    pub unsafe fn da_pop<T>(da: *mut Array<T>) -> Option<T> {
        let da_ref = &mut *da;

        if da_ref.count == 0 {
            return None;
        }

        da_ref.count -= 1;
        let ptr = da_ref.items.add(da_ref.count);

        Some(ptr::read(ptr))
    }

    pub unsafe fn da_remove<T>(da: *mut Array<T>, index: usize) -> Option<T> {
        let da_ref = &mut *da;

        if index >= da_ref.count {
            return None;
        }

        let ptr = da_ref.items.add(index);
        let removed = ptr::read(ptr);

        let num_to_move = da_ref.count - index - 1;
        if num_to_move > 0 {
            ptr::copy(ptr.add(1), ptr, num_to_move);
        }

        da_ref.count -= 1;

        Some(removed)
    }

    pub unsafe fn da_replace<T>(da: *mut Array<T>, index: usize, item: T) -> T {
        let da_ref = &mut *da;

        if index >= da_ref.count {
            libc::abort();
        }

        let ptr = da_ref.items.add(index);

        let old = ptr::read(ptr);
        ptr::write(ptr, item);

        old
    }

    pub unsafe fn da_destroy<T>(da: *mut Array<T>) {
        let da_ref = &mut *da;

        for i in 0..da_ref.count {
            ptr::drop_in_place(da_ref.items.add(i));
        }

        libc::free(da_ref.items as *mut c_void);
        da_ref.items = ptr::null_mut();
        da_ref.count = 0;
        da_ref.capacity = 0;
    }
}
